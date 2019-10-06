use input::Input;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

pub use Placement::*;


// START OF GENERIC ATTEMPT
// Thoughts
// - position is not included in the Widget and Widget::Delta
// - OR: position is included and exposed with Widget::pos(),
//       or maybe DerefMut<Pos>, with `struct Pos {pub x: i32, pub y: i32}
//
// - Q: How to add a widget?
//      - Expose in "widget tuple of vecs": `get_state` and `get_state_mut`
// - Q: How to include the Id in DeltaData? Either have to make it return `Vec<(Id, W::Delta)>` 
//      ... or use HashMaps instead of Vec

// Say `Gui<(Button, Healt)>
//  -  automatic `impl WidgetData for (Button, Health)`
//  -  with associated type `(HashMap<Button>, HashMap<Health>)`
//
// TODO NEXT
// Actually need to map `State -> Delta`
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// impl for Button etc
pub trait Widget: Deref<Target=Position> + DerefMut<Target=Position> {
    type Delta;
    fn update(&mut self) -> Option<Self::Delta>;
}

// impl for HashMap<W> with DeltaData = HashMap<W::Delta>
// impl for (HashMap<W>, HashMap<V>) with DeltaData = (HashMap<W::Delta>, HashMap<V::Delta>)
// ...
//
pub trait WidgetData<Id: Eq + Hash> {
    type State;
    type Delta;
    fn update(state:&mut Self::State) -> Self::Delta;
}

macro_rules! impl_widget_data {
    ( $($ty: tt,)* ) => {
        paste::item! {
            impl<Id: Eq + Hash + Clone, $( [<T $ty>] ),*> WidgetData<Id> for ( $( [<T $ty>], )* )
            where $( [<T $ty>]: Widget ),*
            {
                type Delta = ( $( HashMap<Id, [<T $ty>]::Delta>, )* );
                type State = ( $( HashMap<Id, [<T $ty>]>, )* );
                fn update(state: &mut Self::State) -> Self::Delta {
                    // TODO: call update on each tuple element.
                    // for each $ty, need a different index into the tuple
                    ( $( {
                        let mut deltas = HashMap::new();
                        for (k, v) in state.$ty.iter_mut() {
                            if let Some(delta) = v.update() {
                                deltas.insert(k.clone(), delta);
                            }
                        }
                        deltas
                    }, )* )
                }
            }
        }
    }
}


// impl_widget_data_for_tuples!(1,2,3,4,5,6,7,8,9,11,12,13,14,15,16,17,18,19,20);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,13,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,12,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,11,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,10,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,9,);
impl_widget_data!(0,1,2,3,4,5,6,7,8,);
impl_widget_data!(0,1,2,3,4,5,6,7,);
impl_widget_data!(0,1,2,3,4,5,6,);
impl_widget_data!(0,1,2,3,4,5,);
impl_widget_data!(0,1,2,3,4,);
impl_widget_data!(0,1,2,3,);
impl_widget_data!(0,1,2,);
impl_widget_data!(0,1,);
impl_widget_data!(0,);

#[derive(Default)]
pub struct Gui<Id: Eq + Hash, W: WidgetData<Id>> {
    pub state: W::State,
}

impl<Id: Eq + Hash, W: WidgetData<Id>> Gui<Id, W> {
    pub fn update(&mut self, input: &Input) -> W::Delta {
        W::update(&mut self.state)
    }
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
}


#[derive(Copy, Clone)]
pub enum Placement<Id> {
    /// Relative from top left
    Pos (i32),
    /// Relative to screen from right or bottom
    Neg (i32),
    /// Relative to another widget
    FromWidget (Id, i32),
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

// EITHER
pub struct ButtonDelta {
    pub pos: Option<Position>,
    pub w: Option<i32>,
    pub h: Option<i32>,
    pub state: Option<ButtonState>,
}

// OR
enum ButtonDelta {
    Pos (Position),
    State {new: ButtonState},
}



