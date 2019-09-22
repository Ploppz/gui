/*!
# Ideas

## Tying the creation of a widget to the usage of that widget's events/info
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


# Several Gui<T> can be created
-- each one a group of widgets (for example one menu, each with their own `enum` to identify stuffs

Widgets
 - button
 - toggle button and/or checkboxes
 - slider
 - tree of stuffff (future- at which point we probably also want to deal more with tree structures of widgets)

TODO:
 - where should origin of widgets be? configurable
 - relative position: from center, start point, or start + width?
*/

use input::Input;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub use Position::*;

#[derive(Debug, Clone)]
pub enum Widget {
    Button (Button),
}

pub struct WidgetInternal<Id> {
    pub widget: Widget,
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
    pub fn new(widget: Widget, x: Position<Id>, y: Position<Id>) -> WidgetInternal<Id> {
        WidgetInternal {
            widget,
            x_pos: x,
            y_pos: y,
            x: 0,
            y: 0,
        }
    }
    pub fn update(&mut self, input: &Input) {
        match &self.widget {
            Widget::Button (button) => {
                // TODO
            }
        }
    }
}

#[derive(Default)]
pub struct Gui<Id: Eq + Hash> {
    screen: (i32, i32),
    widgets: HashMap<Id, WidgetInternal<Id>>,

    events: Vec<Event<Id>>,
    // Working memory
    // (maybe weird solution, just to know in the current frame which positions have
    // been updated and which not (look at `update_position`))
    positions: HashMap<Id, Option<(i32, i32)>>,
}

pub struct Updates<Id> {
    pub positions: Vec<(Id, (i32, i32))>,
    pub buttons: Vec<(Id, ButtonState)>,
}

impl<Id: Eq + Hash + Copy + Clone + Debug> Gui<Id> {
    pub fn get_state(&self) -> Vec<(Id, (i32, i32), Widget)> {
        self.widgets.iter()
            .map(|(id, w)| (*id, (w.x, w.y), w.widget.clone()))
            .collect::<Vec<_>>()
    }
    pub fn update(&mut self, _input: &Input, screen_w: i32, screen_h: i32) -> Updates<Id> {
        self.screen = (screen_w, screen_h);
        self.positions = HashMap::new();

        // Update positions
        let keys: Vec<Id> = self.widgets.keys().map(|id| id.clone()).collect::<Vec<_>>();
        let mut updated_positions: Vec<(Id, (i32, i32))> = Vec::new();
        for id in keys {
            self.update_position(id.clone(), &mut updated_positions);
        }

        // TODO: Update state based on input
        
        Updates {
            positions: updated_positions,
            buttons: Vec::new(), //TODO
        }
    }

    /// `updated` is passed only to collect information about which widgets were updated with a new
    /// position
    fn update_position(&mut self, id: Id, updated: &mut Vec<(Id, (i32, i32))>) {
        let WidgetInternal {widget: _, x_pos, y_pos, x: prev_x, y: prev_y} = self.widgets[&id];
        let x = match x_pos {
            Position::Pos (offset) => offset,
            Position::Neg (offset) => self.screen.0 - offset,
            Position::FromWidget (other_id, offset) => {
                if let None = self.positions.get(&other_id) {
                    self.update_position(other_id.clone(), updated);
                }
                self.positions[&other_id].unwrap().0 + offset
            }
        };
        let y = match y_pos {
            Position::Pos (offset) => offset,
            Position::Neg (offset) => self.screen.1 - offset,
            Position::FromWidget (other_id, offset) => {
                if let None = self.positions[&other_id] {
                    self.update_position(other_id, updated);
                }
                self.positions[&other_id].unwrap().1 + offset
            }
        };
        let w = self.widgets.get_mut(&id).unwrap();
        w.x = x;
        w.y = y;
        self.positions.insert(id, Some((x,y)));

        if x != prev_x || y != prev_y {
            updated.push((id, (x, y)));
        }
    }

    pub fn collect_events(&mut self) -> Vec<Event<Id>> {
        std::mem::replace(&mut self.events, Vec::new())
    }


    pub fn add_widget(&mut self, id: Id, w: Widget, x: Position<Id>, y: Position<Id>) {
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

#[derive(Debug, Clone)]
pub enum Event<Id> {
    ButtonPress (Id),
    Slider (Id, f32),
}

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub w: i32,
    pub h: i32,
    pub state: ButtonState,
}
#[derive(Debug, Clone)]
pub enum ButtonState {
    Hover,
    None,
}
impl Button {
    pub fn new(text: String, w: i32, h: i32) -> Button {
        Button {
            state: ButtonState::None,
            text, w, h
        }
    }
    pub fn wrap(self) -> Widget {
        Widget::Button (self)
    }
}

