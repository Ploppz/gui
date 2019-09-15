/*
- *ID* vs messaging

- tying the creation of a widget to the usage of that widget's events/info?
    Could have a macro that defines a widget, and generates an ID enum
    usage:

```
mod my_widget {
    // generates fn create_gui(&mut gui)
    define_widget! {
        MyButton: add_button(1,2,3,4)
    }
    // auto generates `enum Widgets`, and `pub use Widgets::*;`
}

fn main() {
    let gui = my_widget::create_gui();
    loop {
        // .. stuff
        if gui.button(my_widget::MyButton).pressed {
            println!("hey");
        }
    }
}

```

- several Gui<T> can be created  - each one a group of widgets (for example one menu, each with their own `enum` to identify stuffs

Widgets
 - button
 - toggle button
 - slider
 - tree of stuffff (future- at which point we probably also want to deal more with tree structures of widgets)

LATEST;
 - storing the abs pos in WidgetInternal

*/

use input::Input;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub use Position::*;

#[derive(Default)]
pub struct Gui<Id: Eq + Hash> {
    screen: (i32, i32),
    widgets: HashMap<Id, WidgetInternal<Id>>,

    events: Vec<Event<Id>>,
    // Working memory
    positions: HashMap<Id, Option<(i32, i32)>>,
}

// TODO NEXT: would like to give WidgetInternal to application
// application needs to know whether it's a button, a _ or _ ...
// Would be nice with polymorphism. I.e. let application implement `trait Render` for all widgets -
// at least so as to itself call `render()` on any Box received from the lib. Consider it "user
// data".
//
pub struct WidgetInternal<Id> {
    pub widget: Box<dyn Widget>,
    /// Relative x position as declared
    x_pos: Position<Id>,
    /// Relative y position as declared
    y_pos: Position<Id>,
    /// Absolute x position as rendered
    pub x: i32,
    /// Absolute y position as rendered
    pub y: i32,
}
impl<Id> WidgetInternal<Id> {
    pub fn new<W: Widget>(w: W, x: Position<Id>, y: Position<Id>) -> WidgetInternal<Id> {
        WidgetInternal {
            widget: Box::new(w),
            x_pos: x,
            y_pos: y,
            x: 0,
            y: 0,
        }
    }
}
/// Geometry of a widget, for rendering
enum Geometry {
    Button {x: f32, }
    // TODO: struct with x, y, w, h, id, type?
}

pub trait Widget: 'static {
    /// x, y: of the widget
    fn update(&mut self, input: &Input, x: i32, y: i32);
}



impl<Id: Eq + Hash + Copy + Clone + Debug> Gui<Id> {
    pub fn update(&mut self, _input: &Input, screen_w: i32, screen_h: i32) {
        self.screen = (screen_w, screen_h);
        self.positions = HashMap::new();

        // Update positions
        let keys: Vec<Id> = self.widgets.keys().map(|id| id.clone()).collect::<Vec<_>>();
        for id in keys {
            self.update_position(id.clone());
        }

        // TODO: Update state based on input
    }

    fn update_position(&mut self, id: Id) {
        let WidgetInternal {widget: _, x_pos, y_pos, x: _, y: _} = self.widgets[&id];
        let x = match x_pos {
            Position::Pos (offset) => offset,
            Position::Neg (offset) => self.screen.0 - offset,
            Position::FromWidget (other_id, offset) => {
                if let None = self.positions.get(&other_id) {
                    self.update_position(other_id.clone());
                }
                self.positions[&other_id].unwrap().0 + offset
            }
        };
        let y = match y_pos {
            Position::Pos (offset) => offset,
            Position::Neg (offset) => self.screen.1 - offset,
            Position::FromWidget (other_id, offset) => {
                if let None = self.positions[&other_id] {
                    self.update_position(other_id);
                }
                self.positions[&other_id].unwrap().1 + offset
            }
        };
        let w = self.widgets.get_mut(&id).unwrap();
        w.x = x;
        w.y = y;
    }

    pub fn collect_events(&mut self) -> Vec<Event<Id>> {
        std::mem::replace(&mut self.events, Vec::new())
    }

    // pub fn collect_layout(&self) -> Vec<> {
    // }

    pub fn add_widget<W: Widget>(&mut self, id: Id, w: W, x: Position<Id>, y: Position<Id>) {
        self.widgets.insert(id, WidgetInternal::new(w, x, y));
    }
}
#[derive(Copy, Clone)]
pub enum Position<Id> {
    /// Relative from top left
    Pos (i32),
    /// Relative to screen from right or bottom
    Neg (i32),
    /// Relative to another widget
    FromWidget (Id, i32),
}

pub enum Event<Id> {
    ButtonPress (Id),
    Slider (Id, f32),
}

pub struct Button {
    text: String,
    w: i32,
    h: i32,
    state: ButtonState,
}
impl Button {
    pub fn new(text: String, w: i32, h: i32) -> Button {
        Button {
            state: ButtonState::None,
            text, w, h
        }
    }
}
impl Widget for Button {
    fn update(&mut self, input: &Input, x: i32, y: i32) {
    }
}

pub enum ButtonState {
    Hover,
    None,
}
