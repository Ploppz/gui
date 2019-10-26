#[macro_use]
extern crate mopa;
#[macro_use]
extern crate derive_deref;
use mopa::Any;
use std::collections::HashMap;
use std::hash::Hash;
use winput::Input;

mod button;
pub use button::*;
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

pub trait Event: Any + std::fmt::Debug {}
mopafy!(Event);

pub trait Widget: Any + std::fmt::Debug {
    fn update(&mut self, _: &Input, x: f32, y: f32, mx: f32, my: f32) -> Option<Box<dyn Event>>;
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
    pub fn update(&mut self, input: &Input, sw: f32, sh: f32, mx: f32, my: f32) -> HashMap<Id, Box<dyn Event>> {
        self.screen = (sw, sh);

        // Update positions
        let mut updated_positions = HashMap::new();
        let ids: Vec<Id> = self.widgets.keys().cloned().collect();
        for id in &ids {
            let pos = self.update_position(id.clone(), &mut updated_positions);
            self.widgets.get_mut(&id).unwrap().pos = pos;
        }

        // Update each widget
        let mut events = HashMap::new();
        for (id, w) in self.widgets.iter_mut() {
            let pos = w.pos;
            let event = w.update(input, pos.x, pos.y, mx, my);
            if let Some(event) = event {
                events.insert(id.clone(), event);
            }
        }
        events
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
