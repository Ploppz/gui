#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
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
pub trait Widget: Deref<Target = Position> + DerefMut<Target = Position> {
    type
    Delta;
    fn update(&mut self)
    -> Self::Delta;
}

// impl for HashMap<W> with DeltaData = HashMap<W::Delta>
// impl for (HashMap<W>, HashMap<V>) with DeltaData = (HashMap<W::Delta>, HashMap<V::Delta>)
// ...
//
pub trait WidgetData<Id: Eq + Hash> {
    type
    State;
    type
    Delta;
    fn update(state: &mut Self::State)
    -> Self::Delta;
}

macro_rules! impl_widget_data {
    ($ ($ ty : ident,) *) =>
    {
        impl < Id : Eq + Hash, $ ($ ty), * > WidgetData < Id > for
        ($ ($ ty), *) where $ ($ ty : Widget), *
        {
            type Delta = ($ (HashMap < Id, $ ty :: Delta >), *) ; type State =
            ($ (HashMap < Id, $ ty >), *) ; fn update
            (state : & mut Self :: State) -> Self :: Delta
            {
                unimplemented ! ()
                // TODO: call update on each tuple element.
                // for each $ty, need a different index into the tuple

            }
        }
    }
}
macro_rules! impl_widget_data_for_tuples {
    () => { } ; ($ first : ident $ (,) ? $ ($ rest : ident), *) =>
    {
        impl_widget_data ! ($ first, $ ($ rest,) *) ;
        impl_widget_data_for_tuples ! ($ ($ rest), *) ;
    }
}
impl <Id: Eq + Hash, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,
      T, U, V, W, X, Y, Z> WidgetData<Id> for
 (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
  Z) where A: Widget, B: Widget, C: Widget, D: Widget, E: Widget, F: Widget,
 G: Widget, H: Widget, I: Widget, J: Widget, K: Widget, L: Widget, M: Widget,
 N: Widget, O: Widget, P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget,
 U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, A::Delta>, HashMap<Id, B::Delta>, HashMap<Id, C::Delta>,
     HashMap<Id, D::Delta>, HashMap<Id, E::Delta>, HashMap<Id, F::Delta>,
     HashMap<Id, G::Delta>, HashMap<Id, H::Delta>, HashMap<Id, I::Delta>,
     HashMap<Id, J::Delta>, HashMap<Id, K::Delta>, HashMap<Id, L::Delta>,
     HashMap<Id, M::Delta>, HashMap<Id, N::Delta>, HashMap<Id, O::Delta>,
     HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>,
     HashMap<Id, S::Delta>, HashMap<Id, T::Delta>, HashMap<Id, U::Delta>,
     HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, A>, HashMap<Id, B>, HashMap<Id, C>, HashMap<Id, D>,
     HashMap<Id, E>, HashMap<Id, F>, HashMap<Id, G>, HashMap<Id, H>,
     HashMap<Id, I>, HashMap<Id, J>, HashMap<Id, K>, HashMap<Id, L>,
     HashMap<Id, M>, HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>,
     HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>,
     HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>,
     HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {


        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T,
      U, V, W, X, Y, Z> WidgetData<Id> for
 (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
 where B: Widget, C: Widget, D: Widget, E: Widget, F: Widget, G: Widget,
 H: Widget, I: Widget, J: Widget, K: Widget, L: Widget, M: Widget, N: Widget,
 O: Widget, P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget, U: Widget,
 V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, B::Delta>, HashMap<Id, C::Delta>, HashMap<Id, D::Delta>,
     HashMap<Id, E::Delta>, HashMap<Id, F::Delta>, HashMap<Id, G::Delta>,
     HashMap<Id, H::Delta>, HashMap<Id, I::Delta>, HashMap<Id, J::Delta>,
     HashMap<Id, K::Delta>, HashMap<Id, L::Delta>, HashMap<Id, M::Delta>,
     HashMap<Id, N::Delta>, HashMap<Id, O::Delta>, HashMap<Id, P::Delta>,
     HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>, HashMap<Id, S::Delta>,
     HashMap<Id, T::Delta>, HashMap<Id, U::Delta>, HashMap<Id, V::Delta>,
     HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, B>, HashMap<Id, C>, HashMap<Id, D>, HashMap<Id, E>,
     HashMap<Id, F>, HashMap<Id, G>, HashMap<Id, H>, HashMap<Id, I>,
     HashMap<Id, J>, HashMap<Id, K>, HashMap<Id, L>, HashMap<Id, M>,
     HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>,
     HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>,
     HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>,
     HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U,
      V, W, X, Y, Z> WidgetData<Id> for
 (C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
 where C: Widget, D: Widget, E: Widget, F: Widget, G: Widget, H: Widget,
 I: Widget, J: Widget, K: Widget, L: Widget, M: Widget, N: Widget, O: Widget,
 P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget, U: Widget, V: Widget,
 W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, C::Delta>, HashMap<Id, D::Delta>, HashMap<Id, E::Delta>,
     HashMap<Id, F::Delta>, HashMap<Id, G::Delta>, HashMap<Id, H::Delta>,
     HashMap<Id, I::Delta>, HashMap<Id, J::Delta>, HashMap<Id, K::Delta>,
     HashMap<Id, L::Delta>, HashMap<Id, M::Delta>, HashMap<Id, N::Delta>,
     HashMap<Id, O::Delta>, HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>,
     HashMap<Id, R::Delta>, HashMap<Id, S::Delta>, HashMap<Id, T::Delta>,
     HashMap<Id, U::Delta>, HashMap<Id, V::Delta>, HashMap<Id, W::Delta>,
     HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, C>, HashMap<Id, D>, HashMap<Id, E>, HashMap<Id, F>,
     HashMap<Id, G>, HashMap<Id, H>, HashMap<Id, I>, HashMap<Id, J>,
     HashMap<Id, K>, HashMap<Id, L>, HashMap<Id, M>, HashMap<Id, N>,
     HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>,
     HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>,
     HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
      W, X, Y, Z> WidgetData<Id> for
 (D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where
 D: Widget, E: Widget, F: Widget, G: Widget, H: Widget, I: Widget, J: Widget,
 K: Widget, L: Widget, M: Widget, N: Widget, O: Widget, P: Widget, Q: Widget,
 R: Widget, S: Widget, T: Widget, U: Widget, V: Widget, W: Widget, X: Widget,
 Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, D::Delta>, HashMap<Id, E::Delta>, HashMap<Id, F::Delta>,
     HashMap<Id, G::Delta>, HashMap<Id, H::Delta>, HashMap<Id, I::Delta>,
     HashMap<Id, J::Delta>, HashMap<Id, K::Delta>, HashMap<Id, L::Delta>,
     HashMap<Id, M::Delta>, HashMap<Id, N::Delta>, HashMap<Id, O::Delta>,
     HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>,
     HashMap<Id, S::Delta>, HashMap<Id, T::Delta>, HashMap<Id, U::Delta>,
     HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, D>, HashMap<Id, E>, HashMap<Id, F>, HashMap<Id, G>,
     HashMap<Id, H>, HashMap<Id, I>, HashMap<Id, J>, HashMap<Id, K>,
     HashMap<Id, L>, HashMap<Id, M>, HashMap<Id, N>, HashMap<Id, O>,
     HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>,
     HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>,
     HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W,
      X, Y, Z> WidgetData<Id> for
 (E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where
 E: Widget, F: Widget, G: Widget, H: Widget, I: Widget, J: Widget, K: Widget,
 L: Widget, M: Widget, N: Widget, O: Widget, P: Widget, Q: Widget, R: Widget,
 S: Widget, T: Widget, U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget,
 Z: Widget {
    type
    Delta
    =
    (HashMap<Id, E::Delta>, HashMap<Id, F::Delta>, HashMap<Id, G::Delta>,
     HashMap<Id, H::Delta>, HashMap<Id, I::Delta>, HashMap<Id, J::Delta>,
     HashMap<Id, K::Delta>, HashMap<Id, L::Delta>, HashMap<Id, M::Delta>,
     HashMap<Id, N::Delta>, HashMap<Id, O::Delta>, HashMap<Id, P::Delta>,
     HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>, HashMap<Id, S::Delta>,
     HashMap<Id, T::Delta>, HashMap<Id, U::Delta>, HashMap<Id, V::Delta>,
     HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, E>, HashMap<Id, F>, HashMap<Id, G>, HashMap<Id, H>,
     HashMap<Id, I>, HashMap<Id, J>, HashMap<Id, K>, HashMap<Id, L>,
     HashMap<Id, M>, HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>,
     HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>,
     HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>,
     HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
      Y, Z> WidgetData<Id> for
 (F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where
 F: Widget, G: Widget, H: Widget, I: Widget, J: Widget, K: Widget, L: Widget,
 M: Widget, N: Widget, O: Widget, P: Widget, Q: Widget, R: Widget, S: Widget,
 T: Widget, U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, F::Delta>, HashMap<Id, G::Delta>, HashMap<Id, H::Delta>,
     HashMap<Id, I::Delta>, HashMap<Id, J::Delta>, HashMap<Id, K::Delta>,
     HashMap<Id, L::Delta>, HashMap<Id, M::Delta>, HashMap<Id, N::Delta>,
     HashMap<Id, O::Delta>, HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>,
     HashMap<Id, R::Delta>, HashMap<Id, S::Delta>, HashMap<Id, T::Delta>,
     HashMap<Id, U::Delta>, HashMap<Id, V::Delta>, HashMap<Id, W::Delta>,
     HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, F>, HashMap<Id, G>, HashMap<Id, H>, HashMap<Id, I>,
     HashMap<Id, J>, HashMap<Id, K>, HashMap<Id, L>, HashMap<Id, M>,
     HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>,
     HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>,
     HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>,
     HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
      Z> WidgetData<Id> for
 (G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where G: Widget,
 H: Widget, I: Widget, J: Widget, K: Widget, L: Widget, M: Widget, N: Widget,
 O: Widget, P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget, U: Widget,
 V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, G::Delta>, HashMap<Id, H::Delta>, HashMap<Id, I::Delta>,
     HashMap<Id, J::Delta>, HashMap<Id, K::Delta>, HashMap<Id, L::Delta>,
     HashMap<Id, M::Delta>, HashMap<Id, N::Delta>, HashMap<Id, O::Delta>,
     HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>,
     HashMap<Id, S::Delta>, HashMap<Id, T::Delta>, HashMap<Id, U::Delta>,
     HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, G>, HashMap<Id, H>, HashMap<Id, I>, HashMap<Id, J>,
     HashMap<Id, K>, HashMap<Id, L>, HashMap<Id, M>, HashMap<Id, N>,
     HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>,
     HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>,
     HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z>
 WidgetData<Id> for (H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
 where H: Widget, I: Widget, J: Widget, K: Widget, L: Widget, M: Widget,
 N: Widget, O: Widget, P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget,
 U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, H::Delta>, HashMap<Id, I::Delta>, HashMap<Id, J::Delta>,
     HashMap<Id, K::Delta>, HashMap<Id, L::Delta>, HashMap<Id, M::Delta>,
     HashMap<Id, N::Delta>, HashMap<Id, O::Delta>, HashMap<Id, P::Delta>,
     HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>, HashMap<Id, S::Delta>,
     HashMap<Id, T::Delta>, HashMap<Id, U::Delta>, HashMap<Id, V::Delta>,
     HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, H>, HashMap<Id, I>, HashMap<Id, J>, HashMap<Id, K>,
     HashMap<Id, L>, HashMap<Id, M>, HashMap<Id, N>, HashMap<Id, O>,
     HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>,
     HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>,
     HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z>
 WidgetData<Id> for (I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)
 where I: Widget, J: Widget, K: Widget, L: Widget, M: Widget, N: Widget,
 O: Widget, P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget, U: Widget,
 V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, I::Delta>, HashMap<Id, J::Delta>, HashMap<Id, K::Delta>,
     HashMap<Id, L::Delta>, HashMap<Id, M::Delta>, HashMap<Id, N::Delta>,
     HashMap<Id, O::Delta>, HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>,
     HashMap<Id, R::Delta>, HashMap<Id, S::Delta>, HashMap<Id, T::Delta>,
     HashMap<Id, U::Delta>, HashMap<Id, V::Delta>, HashMap<Id, W::Delta>,
     HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, I>, HashMap<Id, J>, HashMap<Id, K>, HashMap<Id, L>,
     HashMap<Id, M>, HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>,
     HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>,
     HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>,
     HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z>
 WidgetData<Id> for (J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where
 J: Widget, K: Widget, L: Widget, M: Widget, N: Widget, O: Widget, P: Widget,
 Q: Widget, R: Widget, S: Widget, T: Widget, U: Widget, V: Widget, W: Widget,
 X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, J::Delta>, HashMap<Id, K::Delta>, HashMap<Id, L::Delta>,
     HashMap<Id, M::Delta>, HashMap<Id, N::Delta>, HashMap<Id, O::Delta>,
     HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>,
     HashMap<Id, S::Delta>, HashMap<Id, T::Delta>, HashMap<Id, U::Delta>,
     HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, J>, HashMap<Id, K>, HashMap<Id, L>, HashMap<Id, M>,
     HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>,
     HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>,
     HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>,
     HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z>
 WidgetData<Id> for (K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where
 K: Widget, L: Widget, M: Widget, N: Widget, O: Widget, P: Widget, Q: Widget,
 R: Widget, S: Widget, T: Widget, U: Widget, V: Widget, W: Widget, X: Widget,
 Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, K::Delta>, HashMap<Id, L::Delta>, HashMap<Id, M::Delta>,
     HashMap<Id, N::Delta>, HashMap<Id, O::Delta>, HashMap<Id, P::Delta>,
     HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>, HashMap<Id, S::Delta>,
     HashMap<Id, T::Delta>, HashMap<Id, U::Delta>, HashMap<Id, V::Delta>,
     HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, K>, HashMap<Id, L>, HashMap<Id, M>, HashMap<Id, N>,
     HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>,
     HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>,
     HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z>
 WidgetData<Id> for (L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where
 L: Widget, M: Widget, N: Widget, O: Widget, P: Widget, Q: Widget, R: Widget,
 S: Widget, T: Widget, U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget,
 Z: Widget {
    type
    Delta
    =
    (HashMap<Id, L::Delta>, HashMap<Id, M::Delta>, HashMap<Id, N::Delta>,
     HashMap<Id, O::Delta>, HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>,
     HashMap<Id, R::Delta>, HashMap<Id, S::Delta>, HashMap<Id, T::Delta>,
     HashMap<Id, U::Delta>, HashMap<Id, V::Delta>, HashMap<Id, W::Delta>,
     HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, L>, HashMap<Id, M>, HashMap<Id, N>, HashMap<Id, O>,
     HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>,
     HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>,
     HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> WidgetData<Id>
 for (M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where M: Widget, N: Widget,
 O: Widget, P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget, U: Widget,
 V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, M::Delta>, HashMap<Id, N::Delta>, HashMap<Id, O::Delta>,
     HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>,
     HashMap<Id, S::Delta>, HashMap<Id, T::Delta>, HashMap<Id, U::Delta>,
     HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, M>, HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>,
     HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>,
     HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>,
     HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> WidgetData<Id> for
 (N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where N: Widget, O: Widget,
 P: Widget, Q: Widget, R: Widget, S: Widget, T: Widget, U: Widget, V: Widget,
 W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, N::Delta>, HashMap<Id, O::Delta>, HashMap<Id, P::Delta>,
     HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>, HashMap<Id, S::Delta>,
     HashMap<Id, T::Delta>, HashMap<Id, U::Delta>, HashMap<Id, V::Delta>,
     HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, N>, HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>,
     HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>,
     HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>,
     HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, O, P, Q, R, S, T, U, V, W, X, Y, Z> WidgetData<Id> for
 (O, P, Q, R, S, T, U, V, W, X, Y, Z) where O: Widget, P: Widget, Q: Widget,
 R: Widget, S: Widget, T: Widget, U: Widget, V: Widget, W: Widget, X: Widget,
 Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, O::Delta>, HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>,
     HashMap<Id, R::Delta>, HashMap<Id, S::Delta>, HashMap<Id, T::Delta>,
     HashMap<Id, U::Delta>, HashMap<Id, V::Delta>, HashMap<Id, W::Delta>,
     HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, O>, HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>,
     HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>,
     HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, P, Q, R, S, T, U, V, W, X, Y, Z> WidgetData<Id> for
 (P, Q, R, S, T, U, V, W, X, Y, Z) where P: Widget, Q: Widget, R: Widget,
 S: Widget, T: Widget, U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget,
 Z: Widget {
    type
    Delta
    =
    (HashMap<Id, P::Delta>, HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>,
     HashMap<Id, S::Delta>, HashMap<Id, T::Delta>, HashMap<Id, U::Delta>,
     HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, P>, HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>,
     HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>,
     HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, Q, R, S, T, U, V, W, X, Y, Z> WidgetData<Id> for
 (Q, R, S, T, U, V, W, X, Y, Z) where Q: Widget, R: Widget, S: Widget,
 T: Widget, U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, Q::Delta>, HashMap<Id, R::Delta>, HashMap<Id, S::Delta>,
     HashMap<Id, T::Delta>, HashMap<Id, U::Delta>, HashMap<Id, V::Delta>,
     HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, Q>, HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>,
     HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>,
     HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, R, S, T, U, V, W, X, Y, Z> WidgetData<Id> for
 (R, S, T, U, V, W, X, Y, Z) where R: Widget, S: Widget, T: Widget, U: Widget,
 V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, R::Delta>, HashMap<Id, S::Delta>, HashMap<Id, T::Delta>,
     HashMap<Id, U::Delta>, HashMap<Id, V::Delta>, HashMap<Id, W::Delta>,
     HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, R>, HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>,
     HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>,
     HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, S, T, U, V, W, X, Y, Z> WidgetData<Id> for
 (S, T, U, V, W, X, Y, Z) where S: Widget, T: Widget, U: Widget, V: Widget,
 W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, S::Delta>, HashMap<Id, T::Delta>, HashMap<Id, U::Delta>,
     HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, S>, HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>,
     HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, T, U, V, W, X, Y, Z> WidgetData<Id> for
 (T, U, V, W, X, Y, Z) where T: Widget, U: Widget, V: Widget, W: Widget,
 X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, T::Delta>, HashMap<Id, U::Delta>, HashMap<Id, V::Delta>,
     HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, T>, HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>,
     HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, U, V, W, X, Y, Z> WidgetData<Id> for (U, V, W, X, Y, Z)
 where U: Widget, V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, U::Delta>, HashMap<Id, V::Delta>, HashMap<Id, W::Delta>,
     HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, U>, HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>,
     HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, V, W, X, Y, Z> WidgetData<Id> for (V, W, X, Y, Z) where
 V: Widget, W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, V::Delta>, HashMap<Id, W::Delta>, HashMap<Id, X::Delta>,
     HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, V>, HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>,
     HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, W, X, Y, Z> WidgetData<Id> for (W, X, Y, Z) where
 W: Widget, X: Widget, Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, W::Delta>, HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>,
     HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, W>, HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, X, Y, Z> WidgetData<Id> for (X, Y, Z) where X: Widget,
 Y: Widget, Z: Widget {
    type
    Delta
    =
    (HashMap<Id, X::Delta>, HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, X>, HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, Y, Z> WidgetData<Id> for (Y, Z) where Y: Widget,
 Z: Widget {
    type
    Delta
    =
    (HashMap<Id, Y::Delta>, HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, Y>, HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
impl <Id: Eq + Hash, Z> WidgetData<Id> for (Z) where Z: Widget {
    type
    Delta
    =
    (HashMap<Id, Z::Delta>);
    type
    State
    =
    (HashMap<Id, Z>);
    fn update(state: &mut Self::State) -> Self::Delta {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("src/lib.rs", 69u32, 1u32))
        }
    }
}
pub struct Gui<Id: Eq + Hash, W: WidgetData<Id>> {
    state: W::State,
}
impl <Id: Eq + Hash, W: WidgetData<Id>> Gui<Id, W> {
    fn update(&mut self, input: &Input) -> W::Delta {
        W::update(&mut self.state)
    }
}
