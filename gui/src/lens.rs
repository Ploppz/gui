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
//! # use gui::{*, interactive::*, lens::*, default::*};
//! # use indexmap::IndexMap;
//! # let mut gui = Gui::new(NoDrawer);
//! # let gui_shared = gui.shared();
//! # let mut parent_id = gui.insert_in_root(Container::<()>::new());
//! # let mut children = gui.get_mut(parent_id).children_proxy();
//! // This example simulates an implementation of `Interactive`, where `children` and
//! // `gui_shared` are both arguments passed to init/update
//! let id = children.insert(Box::new(Button::<()>::new()), &gui_shared);
//! InternalLens::new(children.get_mut(id), gui_shared)
//!     .chain(Widget::first_child)
//!     .chain(TextField::<()>::text)
//!     .put("Click me!".to_string());
//! ```
//!
//!
//! The same can be achieved in an application using `gui` in a similar way:
//! ```
//! # use gui::{*, lens::*, default::*};
//! let mut gui = Gui::new(NoDrawer);
//! gui.insert_in_root_with_alias(Button::<()>::new(), "my-button-id".to_string());
//! // This is how an application would use WidgetLens
//! WidgetLens::new(&mut gui, "my-button-id")
//!     .chain(Widget::first_child)
//!     .chain(TextField::<()>::text)
//!     .put("Click me!".to_string());
//!
//! assert_eq!("Click me!",
//!     WidgetLens::new(&mut gui, "my-button-id")
//!         .chain(Widget::first_child)
//!         .chain(TextField::<()>::text)
//!         .get());
//! ```
//!
//! Note that in place of `"my-button-id"`, `&str`, `String` or `Id` can be used
//! - anything identification you have handy.
//!
//! It is also conceivable to use `InternalLens` in an application whenever you have a `&Widget` or
//! `&mut Widget` rather than just an `Id` and thus do not require `Gui` for resolving the ID:
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
/// For more examples, look to the implementation of widgets like [DropdownButton].
pub trait LeafLens: Lens<Source = Widget> + Clone
where
    Self::Target: PartialEq,
{
    /// Make a string that describes the target field. e.g. `TextField::text`
    fn target(&self) -> String;
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
    fn push_event<F: LeafLens>(&mut self, id: Id, lens: F)
    where
        F::Target: PartialEq,
    {
        self.gui
            .internal
            .borrow_mut()
            .push_event(Event::change(id, lens))
    }
}

/// Used only internally in when implementing `Interactive`
pub struct InternalLens<'a> {
    widget: &'a mut Widget,
    gui: GuiShared,
}
impl<'a> InternalLens<'a> {
    pub fn new(widget: &'a mut Widget, gui: GuiShared) -> Self {
        Self { widget, gui }
    }
}

impl<'a> LensDriver for InternalLens<'a> {
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
