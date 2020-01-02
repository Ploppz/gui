#![feature(type_alias_impl_trait)]
//! # Gui
//!
//! ## Layout
//! The *main axis* is the axis along which widgets are stacked. The other axis is called the
//! *cross axis*.
//!
#[macro_use]
extern crate mopa;
#[macro_use]
extern crate derive_deref;

use indexmap::IndexMap;
use winput::Input;

mod gui;
pub mod interactive;
pub mod lens;
pub mod placement;
pub mod widget;

pub use self::gui::*;
pub use interactive::Interactive;
pub use placement::*;
pub use widget::*;

use interactive::*;
use lens::*;

pub mod test_common;

pub type Id = usize;

use std::any::TypeId;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldId(TypeId);

impl FieldId {
    /// Construct a new FieldId, which contains the TypeId of T
    pub fn of<T: 'static + LeafLens>(_: T) -> FieldId
    where
        T::Target: PartialEq,
    {
        FieldId(TypeId::of::<T>())
    }
    pub fn is<T: 'static + LeafLens>(&self, _: T) -> bool
    where
        T::Target: PartialEq,
    {
        self.0 == TypeId::of::<T>()
    }
    pub fn is_pos(&self) -> bool {
        self.is(Widget::pos)
    }
    pub fn is_size(&self) -> bool {
        self.is(Widget::size)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: Id,
    pub kind: EventKind,
}
impl Event {
    pub fn new(id: Id, kind: EventKind) -> Event {
        Event { id, kind }
    }
    pub fn change<T: LeafLens + 'static>(id: Id, t: T) -> Event
    where
        T::Target: PartialEq,
    {
        Event {
            id,
            kind: EventKind::change(t),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventKind {
    Press,
    Release,
    Hover,
    Unhover,
    /// Change to any field of Widget or Interactive
    Change {
        field: FieldId,
    },
    New,
    // TODO: perhaps something to notify that position has changed
    Removed,
}
impl EventKind {
    pub fn change<T: LeafLens + 'static>(t: T) -> EventKind
    where
        T::Target: PartialEq,
    {
        EventKind::Change {
            field: FieldId::of::<T>(t),
        }
    }
    pub fn is_change<T: LeafLens>(&self, t: T) -> bool
    where
        T::Target: PartialEq,
    {
        if let EventKind::Change { field } = self {
            return field.is(t);
        } else {
            return false;
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Capture {
    pub mouse: bool,
    pub keyboard: bool,
}
impl std::ops::BitOrAssign for Capture {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mouse |= rhs.mouse;
        self.keyboard |= rhs.keyboard;
    }
}

impl<T> std::ops::Index<Axis> for (T, T) {
    type Output = T;
    fn index(&self, idx: Axis) -> &T {
        match idx {
            Axis::X => &self.0,
            Axis::Y => &self.1,
        }
    }
}

impl<T> std::ops::IndexMut<Axis> for (T, T) {
    fn index_mut(&mut self, idx: Axis) -> &mut T {
        match idx {
            Axis::X => &mut self.0,
            Axis::Y => &mut self.1,
        }
    }
}
