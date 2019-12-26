use crate::*;
use mopa::Any;

mod button;
mod container;
mod dropdown;
mod text;

pub use button::*;
pub use container::*;
pub use dropdown::*;
pub use text::*;
// pub use radio::*;

/// An interactive component/node in the tree of widgets that defines a GUI. This is the trait that
/// all different widgets, such as buttons, checkboxes, containers, `Gui` itself, healthbars, ...,
/// implement.
pub trait Interactive: Any + std::fmt::Debug + Send + Sync {
    /// Exists to make it possible for a widget to create children - Gui and Widget
    /// are required for that.
    /// `init` will be called once while the widget is being added to Gui.
    /// `children` provides an interface to add/delete/get children of this widget.
    /// That is, it is basically a wrapper around the owning Widget's `children`

    fn init(&mut self, _children: &mut ChildrenProxy) -> WidgetConfig {
        WidgetConfig::default()
    }
    /// Optional additional logic specific to this widget type, called in the bottom-up phase, and
    /// thus `_local_events` is the accumulated events of all descendants of `self` and `self`
    /// itself.
    /// Any logic handling interactive events in `self`, such as mouse press/release/hover/unhover
    /// should thus be implemented here.
    /// *Make sure that if any fields of `self` are changed, to generate emit a
    /// `EventKind::Change {..}` for that field.*
    /// `_children` is a proxy to the `Widget` which owns `self`.
    /// `_events` is the collection of events which should be pushed to.
    ///
    /// Returns events resulting from this update. For example, if children are added, it should
    /// return Change events for those children.
    fn update(
        &mut self,
        _id: Id,
        _local_events: &[Event],
        _children: &mut ChildrenProxy,
        _events: &mut Vec<Event>,
    ) {
    }

    /// Returns information whether this widget will stop mouse events and state
    /// from reaching other parts of the application.
    fn captures(&self) -> Capture;

    /// Defines an area which is considered "inside" a widget - for checking mouse hover etc.
    /// Provided implementation simply checks whether mouse is inside the boundaries, where `pos`
    /// is the very center of the widget. However, this is configurable in case a finer shape is
    /// desired (e.g. round things).
    fn inside(&self, pos: (f32, f32), size: (f32, f32), mouse: (f32, f32)) -> bool {
        let (x, y, w, h) = (pos.0, pos.1, size.0, size.1);
        let (top, bot, right, left) = (y, y + h, x + w, x);
        mouse.1 < bot && mouse.1 > top && mouse.0 > left && mouse.0 < right
    }
}
mopafy!(Interactive);
