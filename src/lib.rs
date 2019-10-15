#[macro_use]
extern crate mopa;
use winput::Input;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use mopa::Any;

pub use Placement::*;


#[derive(Copy, Clone)]
pub enum Placement<Id> {
    /// Relative from top left
    Pos (i32),
    /// Relative to screen from right or bottom
    Neg (i32),
    /// Relative to another widget
    FromWidget (Id, i32),
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    pub fn zero() -> Position {
        Position {x: 0, y: 0}
    }
}

// impl for Button etc
pub trait Widget: Any + std::fmt::Debug {
    fn update(&mut self, _: &Input);
}
mopafy!(Widget);


pub struct WidgetInternal<Id> {
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
    screen: (i32, i32),
}

impl<Id: Eq + Hash + Clone> Gui<Id> {
    pub fn insert<W: Widget + 'static>(&mut self, id: Id, widget: W, place_x: Placement<Id>, place_y: Placement<Id>) {
        self.widgets.insert(id, WidgetInternal {widget: Box::new(widget), pos: Position::zero(), place_x, place_y});
    }
    pub fn update(&mut self, _input: &Input, sw: i32, sh: i32) {
        self.screen = (sw, sh);
        let mut updated_positions = HashMap::new();

        let ids: Vec<Id> = self.widgets.keys().cloned().collect();
        for id in ids {
            let pos = self.update_position(id.clone(), &mut updated_positions);
            self.widgets.get_mut(&id).unwrap().pos = pos;
        }
    }

    fn update_position(&mut self, id: Id, positions: &mut HashMap<Id, Position>) -> Position {
        if let Some(pos) = positions.get(&id) {
            return *pos;
        }

        let WidgetInternal {ref place_x, ref place_y, ..} = self.widgets[&id];
        let (place_x, place_y) = (place_x.clone(), place_y.clone());
        let x = match place_x {
            Placement::Pos (offset) => offset,
            Placement::Neg (offset) => self.screen.0 - offset,
            Placement::FromWidget (other_id, offset) => {
                if let Some(pos) = positions.get(&other_id) {
                    offset + pos.x
                } else {
                    offset + self.update_position(other_id.clone(), positions).x
                }
            }
        };
        let y = match place_y {
            Placement::Pos (offset) => offset,
            Placement::Neg (offset) => self.screen.1 - offset,
            Placement::FromWidget (other_id, offset) => {
                if let Some(pos) = positions.get(&other_id) {
                    offset + pos.y
                } else {
                    offset + self.update_position(other_id.clone(), positions).y
                }
            }
        };
        positions.insert(id, Position {x,y});
        Position {x,y}
    }
}


#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub pos: Position,
    pub w: i32,
    pub h: i32,
    pub state: ButtonState,
}
impl Button {
    pub fn new(text: String, w: i32, h: i32) -> Button {
        Button {
            text,
            pos: Position::zero(),
            w,
            h,
            state: ButtonState::None,
        }
    }
}
impl Widget for Button {
    fn update(&mut self, _input: &Input) {
        // TODO
    }
}
impl Deref for Button {
    type Target = Position;
    fn deref(&self) -> &Position {
        &self.pos
    }
}
impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Position {
        &mut self.pos
    }
}
#[derive(Debug, Clone)]
pub enum ButtonState {
    Hover,
    None,
}

