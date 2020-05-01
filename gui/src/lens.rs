//! Traits and types to support lenses specialized for use on widgets and their fields.
//!
//! Usage always starts with a `LensRoot`, and lenses can be chained together.
//!
//! The most important reason why lenses exist in this crate, is that when mutating any field of
//! any widget, we want to generate an event that signifies that that particular field was changed.
//! Lenses are used to automatically generate events (they hold a reference to the event buffer),
//! and simultaneously provides a way to encode a field as a value (look to [gui::FieldId]).
//!
//! Lenses can be used to get any descendant of a widget, but also to access fields - this is done
//! with a `LeafLens`, which is always the very last lens (if present).
//!
//! If we already have a `Widget`, we can use `Widget::access`.
//! The following example creates a child button on `widget`, then sets the text of the button
//! (`Button` always has one child which is a `TextField`).
//! ```
//! # use gui::{*, interactive::*, lens::*, default::*};
//! # use indexmap::IndexMap;
//! # let mut gui = Gui::new(NoDrawer, &mut ());
//! # let gui_shared = gui.shared();
//! # let mut parent_id = gui.insert_in_root(Container::new());
//! # let mut widget = gui.get_mut(parent_id);
//! let id = widget.insert_child(Button::<()>::new());
//!
//! widget.access()
//!     .chain(Widget::child(id)) // <- get Button
//!     .chain(Widget::first_child)    // <- get TextField of Button
//!     .chain(TextField::<()>::text)  // <- this is a `LeafLens`
//!     .put("Click me!".to_string()); // <- mutates text and pushes event internally
//! ```
//!
//!
//! The same can be achieved with `Gui::access` to access any widget:
//! ```
//! # use gui::{*, lens::*, default::*};
//! # let mut gui = Gui::new(NoDrawer, &mut ());
//! gui.insert_in_root_with_alias(Button::<()>::new(), "my-button-id".to_string());
//! // This is how an application would use WidgetLens
//! gui.access("my-button-id")
//!     .chain(Widget::first_child)
//!     .chain(TextField::<()>::text)
//!     .put("Click me!".to_string());
//!
//! assert_eq!("Click me!",
//!     gui.access("my-button-id")
//!         .chain(Widget::first_child)
//!         .chain(TextField::<()>::text)
//!         .get());
//! ```
//!
//! Note that in place of `"my-button-id"`, `&str`, `String` or `Id` can be used
//! - any identification you have handy.
//!
//! New widgets that implement Interactive should `#[derive(Lens)]`.

use crate::gui::*;
use crate::*;
pub use gui_derive::Lens;

/// The Lens trait provides the basic lens functionality. In `gui` it should _always_ be used in
/// conjunction with a [LensDriver].
///
/// TODO: how to restrict usage?
pub trait Lens: 'static {
    type Source;
    type Target;
    fn get<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
    fn get_mut<'a>(&self, source: &'a mut Self::Source) -> &'a mut Self::Target;
}

/// A lens to accesses a certain field on a widget.
/// It should exclusively be used as a step in a [lens::Chain].
/// For more examples, look to the implementation of widgets like [gui::default::Select].
pub trait LeafLens: Lens<Source = Widget> + Clone
where
    Self::Target: PartialEq,
{
    /// Make a string that describes the target field. e.g. `TextField::text`
    fn target(&self) -> String;
}

pub trait WidgetLens: LensDriver {
    fn configure<F: FnOnce(&mut WidgetConfig)>(&mut self, f: F);
}
impl<T> WidgetLens for T
where
    T: LensDriver,
{
    fn configure<F: FnOnce(&mut WidgetConfig)>(&mut self, f: F) {
        f(&mut self.get_widget_mut().config)
    }
}

/// Support operations on widgets. Implementors will store Id or &mut Widget internally
pub trait LensDriver {
    fn get_widget(&self) -> &Widget;
    fn get_widget_mut(&mut self) -> &mut Widget;
    fn push_event<F: LeafLens>(&mut self, id: Id, lens: F)
    where
        F::Target: PartialEq;

    fn chain<L: Lens>(self, lens: L) -> Chain<Self, L>
    where
        Self: Sized,
        L: Sized,
    {
        Chain {
            driver: self,
            child_lens: lens,
        }
    }
}

pub struct Chain<A, B> {
    driver: A,
    child_lens: B,
}

impl<A, B> Chain<A, B>
where
    A: LensDriver,
    B: Lens<Source = Widget>,
{
    pub fn get(&self) -> &B::Target {
        let w = self.driver.get_widget();
        self.child_lens.get(w)
    }
}

impl<A, B> Chain<A, B>
where
    A: LensDriver,
    B: LeafLens,
    B::Target: PartialEq,
{
    pub fn put(&mut self, value: B::Target) -> &mut Self {
        let (id, target) = {
            let widget = self.driver.get_widget_mut();
            (widget.get_id(), self.child_lens.get_mut(widget))
        };
        let equal = *target == value;
        if !equal {
            *target = value;
            self.driver.push_event(id, self.child_lens.clone());
        }
        self
    }
}

impl<A, B> LensDriver for Chain<A, B>
where
    A: LensDriver,
    B: Lens<Source = Widget, Target = Widget>,
{
    fn get_widget(&self) -> &Widget {
        self.child_lens.get(self.driver.get_widget())
    }
    fn get_widget_mut(&mut self) -> &mut Widget {
        self.child_lens.get_mut(self.driver.get_widget_mut())
    }
    fn push_event<F: LeafLens>(&mut self, id: Id, lens: F)
    where
        F::Target: PartialEq,
    {
        self.driver.push_event(id, lens)
    }
}

/// The starting point of all lenses. To start a lens, look to `Widget::access` and `Gui::access`
pub struct LensRoot<'a> {
    widget: &'a mut Widget,
    gui: GuiShared,
}
impl<'a> LensRoot<'a> {
    pub fn new(widget: &'a mut Widget, gui: GuiShared) -> Self {
        Self { widget, gui }
    }
}

impl<'a> LensDriver for LensRoot<'a> {
    fn get_widget(&self) -> &Widget {
        &self.widget
    }
    fn get_widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }
    fn push_event<F: LeafLens>(&mut self, id: Id, lens: F)
    where
        F::Target: PartialEq,
    {
        self.gui.borrow_mut().push_event(Event::change(id, lens))
    }
}
