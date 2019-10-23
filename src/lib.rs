#[macro_use]
extern crate mopa;
#[macro_use]
extern crate derive_deref;
use mopa::Any;
use std::collections::HashMap;
use std::hash::Hash;
use winput::Input;
use cgmath::Matrix4;

pub use Placement::*;

#[derive(Copy, Clone)]
pub enum Placement<Id> {
    /// Relative from top left
    Pos(f32),
    /// Relative to screen from right or bottom
    Neg(f32),
    /// Relative to another widget
    FromWidget(Id, f32),
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn zero() -> Position {
        Position { x: 0.0, y: 0.0 }
    }
    pub fn to_tuple(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}

// impl for Button etc
pub trait Widget: Any + std::fmt::Debug {
    fn update(&mut self, _: &Input, x: f32, y: f32, mx: f32, my: f32);
}
mopafy!(Widget);

#[derive(Deref, DerefMut)]
pub struct WidgetInternal<Id> {
    #[deref_target]
    pub widget: Box<dyn Widget>,
    pub pos: Position,
    /// Relative x position as declared
    place_x: Placement<Id>,
    /// Relative y position as declared
    place_y: Placement<Id>,
}

#[derive(Default)]
pub struct Gui<Id: Eq + Hash> {
    pub widgets: HashMap<Id, WidgetInternal<Id>>,
    screen: (f32, f32),
}

impl<Id: Eq + Hash + Clone> Gui<Id> {
    pub fn insert<W: Widget + 'static>(
        &mut self,
        id: Id,
        widget: W,
        place_x: Placement<Id>,
        place_y: Placement<Id>,
    ) {
        self.widgets.insert(
            id,
            WidgetInternal {
                widget: Box::new(widget),
                pos: Position::zero(),
                place_x,
                place_y,
            },
        );
    }
    pub fn update(&mut self, input: &Input, sw: f32, sh: f32, mx: f32, my: f32) {
        self.screen = (sw, sh);

        // Update positions
        let mut updated_positions = HashMap::new();
        let ids: Vec<Id> = self.widgets.keys().cloned().collect();
        for id in &ids {
            let pos = self.update_position(id.clone(), &mut updated_positions);
            self.widgets.get_mut(&id).unwrap().pos = pos;
        }

        // Update each widget
        for w in self.widgets.values_mut() {
            let pos = w.pos;
            w.update(input, pos.x, pos.y, mx, my);
        }
    }

    fn update_position(&mut self, id: Id, positions: &mut HashMap<Id, Position>) -> Position {
        if let Some(pos) = positions.get(&id) {
            return *pos;
        }

        let WidgetInternal {
            ref place_x,
            ref place_y,
            ..
        } = self.widgets[&id];
        let (place_x, place_y) = (place_x.clone(), place_y.clone());
        let x = match place_x {
            Placement::Pos(offset) => offset,
            Placement::Neg(offset) => self.screen.0 - offset,
            Placement::FromWidget(other_id, offset) => {
                if let Some(pos) = positions.get(&other_id) {
                    offset + pos.x
                } else {
                    offset + self.update_position(other_id.clone(), positions).x
                }
            }
        };
        let y = match place_y {
            Placement::Pos(offset) => offset,
            Placement::Neg(offset) => self.screen.1 - offset,
            Placement::FromWidget(other_id, offset) => {
                if let Some(pos) = positions.get(&other_id) {
                    offset + pos.y
                } else {
                    offset + self.update_position(other_id.clone(), positions).y
                }
            }
        };
        positions.insert(id, Position { x, y });
        Position { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub w: f32,
    pub h: f32,
    pub state: ButtonState,
}
impl Button {
    pub fn new(text: String, w: f32, h: f32) -> Button {
        Button {
            text,
            w,
            h,
            state: ButtonState::None,
        }
    }
}
impl Widget for Button {
    fn update(&mut self, input: &Input, x: f32, y: f32, mx: f32, my: f32) {
        let (top, bot, right, left) = (y + self.h/2.0, y - self.h/2.0, x + self.w/2.0, x - self.w/2.0);
        if my > bot && my < top && mx > left && mx < right {
            self.state = ButtonState::Hover
        } else {
            self.state = ButtonState::None
        }
        // if input.is_mouse_button_toggled_up(winit::MouseButton::Left) {
            // println!("HEY");
        // }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Hover,
    None,
}
