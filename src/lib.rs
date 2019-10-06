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
            impl<Id: Eq + Hash + Clone, $( [<T $ty>] ),*> WidgetData<Id> for ( $( [<T $ty>] ),* )
            where $( [<T $ty>]: Widget ),*
            {
                type Delta = ( $( HashMap<Id, [<T $ty>]::Delta> ),* );
                type State = ( $( HashMap<Id, [<T $ty>]> ),* );
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
                    } ),* )
                }
            }
        }
    }
}
macro_rules! rev {
    (@ $mac:ident $(,$args:expr)* ; $a:expr               ; $($c:expr),* ) => (        $mac!($($args),* ; $a           $(,$c)* ) );
    (@ $mac:ident $(,$args:expr)* ; $a:expr  $(,$b:expr)* ; $($c:expr),* ) => ( rev!(@ $mac $(,$args)*  ; $($b),* ; $a $(,$c)* ) );
    (  $mac:ident $(,$args:expr)* ; $a:expr,                             ) => (        $mac!($($args),* ; $a                   ) );
    (  $mac:ident $(,$args:expr)* ; $a:expr, $($b:expr,)*                ) => ( rev!(@ $mac $(,$args)*  ; $($b),* ; $a         ) );
}

macro_rules! impl_widget_data_for_tuples {
    ( ) => { };
    ( $first:tt, $($rest: tt),* ) => {
        impl_widget_data!($first, $($rest,)*);
        impl_widget_data_for_tuples!($($rest),*);
    }
}

// impl_widget_data_for_tuples!(1,2,3,4,5,6,7,8,9,11,12,13,14,15,16,17,18,19,20);
impl_widget_data_for_tuples!(2,1,0);

pub struct Gui<Id: Eq + Hash, W: WidgetData<Id>> {
    state: W::State,
}
impl<Id: Eq + Hash, W: WidgetData<Id>> Gui<Id, W> {
    fn update(&mut self, input: &Input) -> W::Delta {
        W::update(&mut self.state)
    }
}
