//! Traits and types to support lenses specialized for use on widgets and their fields.
//!
//! Usage always starts with a `LensDriver`: widgets use `InternalLens` because they have direct
//! access to `events` etc.
//!
//! Users of the `gui` library use `WidgetLens`, which has the capability of pushing events to a
//! buffer internal to `Gui`, and getting widgets from `Gui` based on widget id.
//!
//! Next, a `LensDriver` must be chained with any number of lenses always ending in a `LeafLens`
//! which provides access to a field of a widget.
//!
//! Example from a widget implementation follows. The widget inserts a button as gets its id. Then
//! in changes the text of said button by first getting its first child (which is a TextField), and
//! then getting the `text` field of the `TextField`.
//! ```
//! use gui::{*, interactive::*, lens::*};
//! use indexmap::IndexMap;
//! # let mut gui = Gui::new(NoDrawer);
//! # let mut parent_id = gui.insert_in_root(Container::new());
//! # let mut children = gui.get_mut(parent_id).children_proxy();
//! # let mut events = &mut Vec::new();
//! let id = children.insert(Box::new(Button::new()));
//! InternalLens::new(children.get_mut(id), events)
//!     .chain(Widget::first_child)
//!     .chain(TextField::text)
//!     .put("Click me!".to_string());
//! ```
//!
//! The same can be achieved in an application using `gui` in a similar way:
//! ```
//! use gui::{*, interactive::*, lens::*};
//! # let mut gui = Gui::new(NoDrawer);
//! WidgetLens::new(&mut gui, "my-button-id")
//!     .chain(Widget::first_child)
//!     .chain(TextField::text)
//!     .put("Click me!".to_string());
//! ```
//!
//! Note that in place of `"my-button-id"`, `&str`, `String` or `Id` can be used
//! - anything identification you have handy.
//! The last lens in the chain must be
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
/// It should exclusively be used as a step in a [lens2::Chain].
/// For more examples, look to the implementation of widgets like [DropdownButton].
pub trait LeafLens: Lens<Source = Widget> + Clone
where
    Self::Target: PartialEq,
{
}

/// NOTE: with `Chain` its only implementor, it's not necessary to have a trait for this, but I
/// thoguht about extending it in the future to let other things than Chain access fields.
pub trait FieldLens<Target: PartialEq> {
    fn get(&self) -> &Target;
    fn put(&mut self, value: Target) -> &mut Self;
}

pub struct Chain<A, B> {
    driver: A,
    child_lens: B,
}
impl<A, B> FieldLens<B::Target> for Chain<A, B>
where
    A: LensDriver,
    B: LeafLens,
    B::Target: PartialEq,
{
    fn get(&self) -> &B::Target {
        // need to get the child through the child_lens
        // but the 'parent' is never exposed - `LensDriver` doesn't expose it in any way
        self.child_lens.get(self.get_widget())
    }
    fn put(&mut self, value: B::Target) -> &mut Self {
        let target = self.child_lens.get_mut(self.driver.get_widget_mut());
        let equal = *target == value;
        if !equal {
            *target = value;
            self.driver.push_event(self.child_lens.clone());
        }
        self
    }
}

impl<A, B> LensDriver for Chain<A, B>
where
    A: LensDriver,
{
    fn get_widget(&self) -> &Widget {
        self.driver.get_widget()
    }
    fn get_widget_mut(&mut self) -> &mut Widget {
        self.driver.get_widget_mut()
    }
    fn push_event<F: LeafLens>(&mut self, lens: F)
    where
        F::Target: PartialEq,
    {
        self.driver.push_event(lens)
    }
}

/// Support operations on widgets. Implementors will store Id or &mut Widget internally
pub trait LensDriver {
    fn get_widget(&self) -> &Widget;
    fn get_widget_mut(&mut self) -> &mut Widget;
    fn push_event<F: LeafLens>(&mut self, lens: F)
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

/// Used for looking up widgets globally
pub struct WidgetLens<'a, D, I> {
    id: I,
    gui: &'a mut Gui<D>,
}
impl<'a, D, I> WidgetLens<'a, D, I> {
    pub fn new(gui: &'a mut Gui<D>, id: I) -> Self {
        Self { id, gui }
    }
}

impl<'a, D: GuiDrawer, I: AsId<D>> LensDriver for WidgetLens<'a, D, I> {
    fn get_widget(&self) -> &Widget {
        self.gui.get(self.id.clone())
    }
    fn get_widget_mut(&mut self) -> &mut Widget {
        self.gui.get_mut(self.id.clone())
    }
    fn push_event<F: LeafLens>(&mut self, lens: F)
    where
        F::Target: PartialEq,
    {
        let id = self.id.resolve(&self.gui).unwrap();
        self.gui
            .internal
            .borrow_mut()
            .push_event(Event::change(id, lens))
    }
}

/// Used only internally in `Interactive`
pub struct InternalLens<'a, 'b> {
    widget: &'a mut Widget,
    events: &'b mut Vec<Event>,
}
impl<'a, 'b> InternalLens<'a, 'b> {
    pub fn new(widget: &'a mut Widget, events: &'b mut Vec<Event>) -> Self {
        Self { widget, events }
    }
}

impl<'a, 'b> LensDriver for InternalLens<'a, 'b> {
    fn get_widget(&self) -> &Widget {
        &self.widget
    }
    fn get_widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }
    fn push_event<F: LeafLens>(&mut self, lens: F)
    where
        F::Target: PartialEq,
    {
        self.events.push(Event::change(self.widget.get_id(), lens))
    }
}
