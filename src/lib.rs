use winput::Input;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

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

// impl for Button etc
pub trait Widget: Deref<Target=Position> + DerefMut<Target=Position> {
    fn update(&mut self);
}

pub struct WidgetInternal<Id> {
    pub widget: Box<dyn Widget>,
    /// Relative x position as declared
    x_pos: Placement<Id>,
    /// Relative y position as declared
    y_pos: Placement<Id>,
}


#[derive(Default)]
pub struct Gui<Id: Eq + Hash> {
    pub widgets: HashMap<Id, WidgetInternal<Id>>,

    screen: (i32, i32),
}

impl<Id: Eq + Hash + Clone> Gui<Id> {
    pub fn update(&mut self, _input: &Input) {
        let mut updated_positions = HashMap::new();

        let ids: Vec<Id> = self.widgets.keys().cloned().collect();
        for id in ids {
            let _ = self.update_position(id.clone(), &mut updated_positions);
        }
    }

    fn update_position(&mut self, id: Id, positions: &mut HashMap<Id, Position>) -> Position {
        if let Some(pos) = positions.get(&id) {
            return *pos;
        }

        let WidgetInternal {ref x_pos, ref y_pos, ..} = self.widgets[&id];
        let (x_pos, y_pos) = (x_pos.clone(), y_pos.clone());
        let x = match x_pos {
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
        let y = match y_pos {
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
        // let w = self.widgets.get_mut(&id).unwrap();
        // w.x = x;
        // w.y = y;
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

