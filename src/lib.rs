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
use mopa::Any;
use slog::Logger;
use std::{cell::RefCell, ops::Deref, rc::Rc};
use winput::Input;

mod gui;
mod lens;
mod lens2;
mod placement;
mod widgets;

pub use crate::gui::*;
pub use lens::*;
pub use placement::*;
pub use widgets::*;

pub mod test_common;

pub type Id = usize;

/// Macro is needed rather than a member function, in order to preserve borrow information:
/// so that the compiler knows that only `self.children` is borrowed.
macro_rules! children_proxy {
    ($self:ident) => {
        ChildrenProxy {
            self_id: $self.id,
            children: &mut $self.children,
            child_service: $self.child_service.clone(),
        }
    };
}

#[derive(Deref, DerefMut, Debug)]
pub struct Widget {
    #[deref_target]
    pub inner: Box<dyn Interactive>,
    /// Children of this node in the widget tree.
    children: IndexMap<Id, Widget>,
    /// Current absolute position as calculated by layout algorithm.
    /// Any mutation to `pos` has no effect except possibly generating spurious `ChangeSize` events.
    /// (should be read-only outside `gui`)
    pub pos: (f32, f32),
    /// Current relative (to parent) position as calculated by layout algorithm
    /// Any mutation to `rel_pos` has no effect except possibly generating spurious `ChangeSize` events.
    /// (should be read-only outside `gui`)
    pub rel_pos: (f32, f32),
    /// Current size as calculated by layout algorithm
    /// Any mutation to `size` has no effect except possibly generating spurious `ChangeSize` events.
    /// (should be read-only outside `gui`)
    pub size: (f32, f32),

    pub config: WidgetConfig,

    child_service: Rc<RefCell<ChildService>>,

    /// Keeps track of hover state in order to generate the right WidgetEvents
    inside: bool,
    /// Keeps track of mouse press state in order to generate the right WidgetEvents
    pressed: bool,

    /// 'Buffer' - when `true` it is set to `false` by the parent, and the
    changed: bool,

    /// For internal use; mirrors the id that is the key in the HashMap that this Widget is
    /// likely a part of.
    /// NOTE: It's important to always ensure that `self.id` corresponds to the ID as registered in
    /// the gui system.
    id: Id,
}
macro_rules! event {
    ($event:expr, ($widget:expr, $events:expr)) => {{
        let change = $widget.inner.handle_event($event);
        if change {
            $events.push(($widget.id.clone(), WidgetEvent::Change));
        }
        $events.push(($widget.id.clone(), $event));
    }};
}

impl Widget {
    pub fn new(
        id: Id,
        mut widget: Box<dyn Interactive>,
        child_service: Rc<RefCell<ChildService>>,
    ) -> Widget {
        let mut children = IndexMap::new();
        let mut proxy = ChildrenProxy {
            self_id: id,
            children: &mut children,
            child_service: child_service.clone(),
        };
        let config = widget.init(&mut proxy);
        Widget {
            inner: widget,
            children,
            pos: (0.0, 0.0),
            rel_pos: (0.0, 0.0),
            size: (10.0, 10.0),
            config,
            child_service,

            inside: false,
            pressed: false,
            changed: false,
            id,
        }
    }
    /// Remove child for real - only for internal use.
    pub(crate) fn remove(&mut self, id: Id) -> Option<()> {
        self.children.remove(&id).map(drop)
    }
    pub fn children(&self) -> &IndexMap<Id, Widget> {
        &self.children
    }
    pub fn children_proxy(&mut self) -> ChildrenProxy {
        children_proxy!(self)
    }
    pub fn get_id(&self) -> Id {
        self.id
    }
    pub fn hover(&self) -> bool {
        self.inside
    }
    pub fn pressed(&self) -> bool {
        self.pressed
    }
    /// Mark that some internal state has changed in this Widget.
    /// For use when an application itself wants to change state of a Widget - for example toggle a
    /// button in response to a key press. A `Change` event has to be registered so that the drawer
    /// knows to redraw the widget.
    pub fn mark_change(&mut self) {
        self.changed = true;
    }
    /// Update this widget tree recursively, returning accumulated events from all nodes.
    /// Will perform one bottom-up pass and one top-down pass.
    pub fn update(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: (f32, f32),
        log: Logger,
    ) -> (Vec<(Id, WidgetEvent)>, Capture) {
        let (mut e, c) = self.update_bottom_up(input, sw, sh, mouse, log);
        self.update_top_down(&mut e);
        (e, c)
    }
    /// Main update work happens here.
    /// NOTE: Due to recursion order, during update, position of `self` is not yet known.
    /// That's why calculating the absolute positions of widgets has to happen in a second pass.
    fn update_bottom_up(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: (f32, f32),
        log: Logger,
    ) -> (Vec<(Id, WidgetEvent)>, Capture) {
        let mut events = Vec::new();
        let mut capture = Capture::default();

        // Update children
        for child in self.children.values_mut() {
            let (child_events, child_capture) =
                child.update_bottom_up(input, sw, sh, mouse, log.clone());
            capture |= child_capture;
            events.extend(child_events.into_iter());
        }
        // Execute widget-specific logic
        let events2 = self.inner.update(&events, &mut children_proxy!(self));
        // If there are any events pertaining any children, we need to recurse children again
        let re_recurse = events2.iter().any(|(id, _)| *id != self.id);
        if re_recurse {
            // TODO code duplication
            for child in self.children.values_mut() {
                let (child_events, child_capture) =
                    child.update_bottom_up(input, sw, sh, mouse, log.clone());
                capture |= child_capture;
                events.extend(child_events.into_iter());
            }
        }

        events.extend(events2);

        // Update positions of children (and possibly size of self)
        let pos_events = self.layout_alg(log.clone());
        events.extend(pos_events.into_iter());

        if !capture.mouse {
            let now_inside = self.inside(self.pos, self.size, mouse);
            let prev_inside = self.inside;
            self.inside = now_inside;

            if now_inside && !prev_inside {
                event!(WidgetEvent::Hover, (self, events));
            } else if prev_inside && !now_inside {
                event!(WidgetEvent::Unhover, (self, events));
            }

            if now_inside {
                capture |= self.inner.captures();
            }

            if now_inside && input.is_mouse_button_toggled_down(winit::event::MouseButton::Left) {
                self.pressed = true;
                event!(WidgetEvent::Press, (self, events));
            }
            if self.pressed && input.is_mouse_button_toggled_up(winit::event::MouseButton::Left) {
                self.pressed = false;
                event!(WidgetEvent::Release, (self, events));
            }
        }

        if self.changed {
            events.push((self.id.clone(), WidgetEvent::Change));
            self.changed = false;
        }

        (events, capture)
    }
    /// Calculates absolute positions
    fn update_top_down(&mut self, events: &mut Vec<(Id, WidgetEvent)>) {
        let pos = self.pos;
        for child in self.children.values_mut() {
            let new_pos = (pos.0 + child.rel_pos.0, pos.1 + child.rel_pos.1);
            if new_pos != child.pos {
                event!(WidgetEvent::ChangePos, (child, events));
                child.pos = new_pos;
            }
            child.update_top_down(events);
        }
    }

    /// Not recursive - only updates the position of children.
    /// (and updates size of `self` if applicable)
    fn layout_alg(&mut self, _log: Logger) -> Vec<(Id, WidgetEvent)> {
        // println!("Positioning Parent [{}]", self.id);
        if self.config.layout_wrap {
            unimplemented!()
        }
        // let id = self.id.clone();
        let mut events = Vec::new();
        let size = self.size;
        let layout_align = self.config.layout_align;
        let layout_main_margin = self.config.layout_main_margin;
        let padding_min = self.config.padding_min;

        let (main_axis, cross_axis) = (
            self.config.layout_direction,
            self.config.layout_direction.other(),
        );

        let mut layout_progress = self.config.padding_min[main_axis];
        // max width/height along cross axis
        let mut cross_size = 0.0;

        for child in self.children.values_mut() {
            let mut child_relative_pos = (0.0, 0.0);
            if let Some(place) = child.config.place {
                // Child does not participate in layout
                child_relative_pos = (
                    match place.x {
                        PlacementAxis::Fixed(x) => match place.x_anchor {
                            Anchor::Min => x,
                            Anchor::Center => (size.0 - child.size.0) / 2.0 + x,
                            Anchor::Max => size.0 - child.size.0 - x,
                        },
                    },
                    match place.y {
                        PlacementAxis::Fixed(y) => match place.y_anchor {
                            Anchor::Min => y,
                            Anchor::Center => (size.1 - child.size.1) / 2.0 + y,
                            Anchor::Max => size.1 - child.size.1 - y,
                        },
                    },
                );
            } else {
                // Layout algorithm
                child_relative_pos[main_axis] = layout_progress;
                layout_progress += child.size[main_axis] + layout_main_margin;
                child_relative_pos[cross_axis] = match layout_align {
                    Anchor::Min => padding_min[cross_axis],
                    Anchor::Center => (size[cross_axis] - child.size[cross_axis]) / 2.0,
                    Anchor::Max => unimplemented!(),
                };
                if child.size[cross_axis] > cross_size {
                    cross_size = child.size[cross_axis]
                }
            };

            // println!("Positioning Child [{}] relative_pos={:?}", child.id, child_relative_pos);
            child.rel_pos = child_relative_pos;
        }
        // because it should only be _between_ children - not after the last one
        layout_progress -= layout_main_margin;
        layout_progress += self.config.padding_max[main_axis];

        let mut new_size = self.size;
        // println!("[positioning {}] pre size {:?}", self.id, new_size);
        let size_hint = (self.config.size_hint_x, self.config.size_hint_y);
        match size_hint[main_axis] {
            SizeHint::Minimize => new_size[main_axis] = layout_progress,
            SizeHint::External(s) => new_size[main_axis] = s,
        }
        match size_hint[cross_axis] {
            SizeHint::Minimize => {
                new_size[cross_axis] = cross_size
                    + self.config.padding_min[cross_axis]
                    + self.config.padding_max[cross_axis]
            }
            SizeHint::External(s) => new_size[cross_axis] = s,
        }
        if new_size != self.size {
            self.size = new_size;
            event!(WidgetEvent::ChangeSize, (self, events));
        }

        events
    }
    pub fn recursive_children_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Widget> + 'a> {
        Box::new(
            self.children.values().chain(
                self.children
                    .values()
                    .map(|child| child.recursive_children_iter())
                    .flatten(),
            ),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WidgetConfig {
    /// Optional positioning; makes this widget not participate in its siblings' layout
    pub place: Option<Placement>,
    /// The axis along which to stack children
    pub layout_direction: Axis,
    /// If true, children are stacked in the cross axis when the main axis fills up.
    pub layout_wrap: bool,
    /// Alignment of children along the cross axis (the axis which is not the direction).
    pub layout_align: Anchor,
    /// Space between widgets in the main axis.
    /// TODO: should maybe be a "justify" enum where you can choose to space them evenly etc
    pub layout_main_margin: f32,

    // padding
    /// left and top padding respectively
    pub padding_min: (f32, f32),
    /// right and bot padding respectively
    pub padding_max: (f32, f32),

    // size hints
    pub size_hint_x: SizeHint,
    pub size_hint_y: SizeHint,
}
impl Default for WidgetConfig {
    fn default() -> Self {
        WidgetConfig {
            place: None,
            layout_direction: Axis::X,
            layout_wrap: false,
            layout_align: Anchor::Min,
            layout_main_margin: 0.0,

            padding_min: (0.0, 0.0),
            padding_max: (0.0, 0.0),

            size_hint_x: SizeHint::default(),
            size_hint_y: SizeHint::default(),
        }
    }
}
impl WidgetConfig {
    pub fn layout(
        mut self,
        layout_direction: Axis,
        layout_wrap: bool,
        layout_align: Anchor,
        _layout_main_margin: f32,
    ) -> Self {
        self.layout_direction = layout_direction;
        self.layout_wrap = layout_wrap;
        self.layout_align = layout_align;
        self.layout_main_margin = self.layout_main_margin;
        self
    }
    pub fn placement(mut self, place: Placement) -> Self {
        self.place = Some(place);
        self
    }
    pub fn size_hint(mut self, x: SizeHint, y: SizeHint) -> Self {
        self.size_hint_x = x;
        self.size_hint_y = y;
        self
    }
    /// Fixed width
    pub fn width(mut self, w: f32) -> Self {
        self.size_hint_x = SizeHint::External(w);
        self
    }
    /// Fixed height
    pub fn height(mut self, h: f32) -> Self {
        self.size_hint_y = SizeHint::External(h);
        self
    }
    pub fn set_size(&mut self, w: f32, h: f32) {
        self.size_hint_x = SizeHint::External(w);
        self.size_hint_y = SizeHint::External(h);
    }
    pub fn set_width(&mut self, w: f32) {
        self.size_hint_x = SizeHint::External(w);
    }
    pub fn set_height(&mut self, h: f32) {
        self.size_hint_y = SizeHint::External(h);
    }
    pub fn padding(mut self, top: f32, bot: f32, left: f32, right: f32) -> Self {
        self.padding_min = (left, top);
        self.padding_max = (right, bot);
        self
    }
}

/// Provides an interface to insert, delete and get immediate children.
/// Through Deref, we can get the immediate children immutably.
/// DerefMut is not implemented, because it is forbidden to insert children without using the
/// provided `ChildrenProxy::insert` function.
/// NOTE: If you need to get a widget in the widget tree that is not immediate, look to
/// [gui::WidgetLens] or the getters of [Gui]
///
pub struct ChildrenProxy<'a> {
    self_id: Id,
    /// children of a widget
    children: &'a mut IndexMap<Id, Widget>,
    child_service: Rc<RefCell<ChildService>>,
}
impl<'a> Deref for ChildrenProxy<'a> {
    type Target = IndexMap<Id, Widget>;
    fn deref(&self) -> &Self::Target {
        self.children
    }
}
impl<'a> ChildrenProxy<'a> {
    pub fn insert(&mut self, widget: Box<dyn Interactive>) -> Id {
        let id = self.child_service.borrow_mut().new_id();

        // Update paths
        let path = if self.self_id == 1 {
            vec![]
        } else {
            let mut p = self.child_service.borrow().paths[&self.self_id].clone();
            p.push(self.self_id);
            p
        };
        self.child_service.borrow_mut().paths.insert(id, path);
        let widget = Widget::new(id, widget, self.child_service.clone());
        self.children.insert(id, widget);
        id
    }
    pub fn remove(&mut self, id: Id) {
        self.child_service.borrow_mut().remove(id);
        // self.children.shift_remove(&id)
    }
    pub fn get_mut(&mut self, id: Id) -> &mut Widget {
        self.children.get_mut(&id).unwrap()
    }
    pub fn values_mut(&mut self) -> indexmap::map::ValuesMut<usize, Widget> {
        self.children.values_mut()
    }
}

// TODO move to its own module. Problem with MOPA
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
    /// Optional additional logic specific to this widget type, with events from children.
    /// Returns events resulting from this update. For example, if children are added, it should
    /// return Change events for those children.
    fn update(
        &mut self,
        _events: &[(Id, WidgetEvent)],
        _children: &mut ChildrenProxy,
    ) -> Vec<(Id, WidgetEvent)> {
        Vec::new()
    }
    /// Returns true if some internal state has changed in this widget (not in children)
    fn handle_event(&mut self, event: WidgetEvent) -> bool;

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WidgetEvent {
    Press,
    Release,
    Hover,
    Unhover,
    ChangePos,
    ChangeSize,
    /// Change to any internal state.
    /// Also issued upon first discovery of widget.
    Change,
    // TODO: perhaps something to notify that position has changed
    Removed,
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
