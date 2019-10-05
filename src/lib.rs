use input::Input;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};


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
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// impl for Button etc
pub trait Widget: Deref<Target=Position> + DerefMut<Target=Position> {
    type Delta;
    fn update(&mut self) -> Self::Delta;
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
    ( $($ty: ident,)* ) => {
        impl<Id: Eq + Hash, $( $ty ),*> WidgetData<Id> for ( $( $ty ),* )
        where $( $ty: Widget ),*
        {
            type Delta = ( $( HashMap<Id, $ty::Delta> ),* );
            type State = ( $( HashMap<Id, $ty> ),* );
            fn update(state: &mut Self::State) -> Self::Delta {
                // TODO: call update on each tuple element.
                // for each $ty, need a different index into the tuple
                
                unimplemented!()
            }
        }
    }
}
macro_rules! impl_widget_data_for_tuples {
    ( ) => { };
    ( $first:ident $(,)? $($rest: ident),* ) => {
        impl_widget_data!($first, $($rest,)*);
        impl_widget_data_for_tuples!($($rest),*);
    }
}

impl_widget_data_for_tuples!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);

pub struct Gui<Id: Eq + Hash, W: WidgetData<Id>> {
    state: W::State,
}
impl<Id: Eq + Hash, W: WidgetData<Id>> Gui<Id, W> {
    fn update(&mut self, input: &Input) -> W::Delta {
        W::update(&mut self.state)
    }
}
