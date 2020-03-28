use crate::*;
use mopa::Any;

/// An interactive component/node in the tree of widgets that defines a GUI. This is the trait that
/// all different widgets, such as buttons, checkboxes, containers, `Gui` itself, healthbars, ...,
/// implement.
pub trait Interactive: Any + std::fmt::Debug + Send + Sync {
    /// Exists to make it possible for a widget to create children - Gui and Widget
    /// are required for that.
    /// `init` will be called once while the widget is being added to Gui.
    /// `children` provides an interface to add/delete/get children of this widget.
    /// That is, it is basically a wrapper around the owning Widget's `children`

    fn init(&mut self, _ctx: &mut WidgetContext) -> WidgetConfig {
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
    fn update(&mut self, _id: Id, _local_events: Vec<Event>, _ctx: &mut WidgetContext) {}

    /// Returns information whether this widget will stop mouse events and state
    /// from reaching other parts of the application.
    fn captures(&self) -> Capture;

    /// Defines an area which is considered "inside" a widget - for checking mouse hover etc.
    /// Provided implementation simply checks whether mouse is inside the boundaries, where `pos`
    /// is the very center of the widget. However, this is configurable in case a finer shape is
    /// desired (e.g. round things).
    fn inside(&self, pos: Vec2, size: Vec2, mouse: Vec2) -> bool {
        let min = pos;
        let max = pos + size;
        mouse.y < max.y && mouse.y > min.y && mouse.x > min.x && mouse.x < max.x
    }

    /// If the widget has some sort of intrinsic size, returns Some.
    /// Anything whose real size depends on the drawer (text, sprites, ..).
    /// NOTE: Only basic 'leaf' widgets like text need to implement this - it's not like it must be
    /// implemented on
    /// Default returns None.
    fn determine_size(&self, _drawer: &mut dyn TextCalculator) -> Option<Vec2> {
        None
    }
}
mopafy!(Interactive);
