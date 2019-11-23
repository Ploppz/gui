#[macro_use]
extern crate mopa;
#[macro_use]
extern crate derive_deref;
use mopa::Any;
use winput::Input;

mod gui;
mod placement;
mod widgets;

pub use crate::gui::*;
pub use placement::*;
pub use widgets::*;

#[cfg(test)]
mod test;

#[derive(Deref, DerefMut, Debug)]
pub struct Widget {
    #[deref_target]
    pub inner: Box<dyn Interactive>,
    pub pos: (f32, f32),
    pub size: (f32, f32),

    /// Declarative placement (used to calculate position)
    pub place: Placement,
    pub anchor: (Anchor, Anchor),

    // padding
    pub padding_top: f32,
    pub padding_left: f32,
    pub padding_right: f32,
    pub padding_bot: f32,

    // size hints
    pub size_hint_x: SizeHint,
    pub size_hint_y: SizeHint,

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
    id: String,
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
    pub fn new<W: Interactive>(id: String, widget: W) -> Widget {
        let (size_hint_x, size_hint_y) = widget.default_size_hint();
        Widget {
            inner: Box::new(widget),
            pos: (0.0, 0.0),
            size: (10.0, 10.0), // TODO Interactive::default_size()?
            place: Placement::float(),
            anchor: (Anchor::Min, Anchor::Min),
            padding_top: 0.0,
            padding_bot: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            size_hint_x,
            size_hint_y,
            inside: false,
            pressed: false,
            changed: false,
            id,
        }
    }
    pub fn placement(mut self, place: Placement) -> Self {
        self.place = place;
        self
    }
    pub fn padding(mut self, top: f32, bot: f32, left: f32, right: f32) -> Self {
        self.padding_top = top;
        self.padding_bot = bot;
        self.padding_left = left;
        self.padding_right = right;
        self
    }
    pub fn get_id(&self) -> &str {
        &self.id
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
    /// Update this widget tree recursively, returning accumulated events from all nodes
    pub fn update(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: (f32, f32),
    ) -> (Vec<(String, WidgetEvent)>, Capture) {
        let mut events = Vec::new();
        let mut capture = Capture::default();

        // Update positions of children (and possibly size of self)
        let pos_events = self.update_positions();
        events.extend(pos_events.into_iter());

        // Update children
        for child in self.children_mut() {
            let (child_events, child_capture) = child.update(input, sw, sh, mouse);
            capture |= child_capture;
            events.extend(child_events.into_iter());
        }

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

    /// Not recursive - only updates the position of children.
    /// (and updates size of `self` if applicable)
    fn update_positions(&mut self) -> Vec<(String, WidgetEvent)> {
        // let id = self.id.clone();
        let mut events = Vec::new();
        let mut float_progress_x = self.padding_left;
        let mut float_progress_y = self.padding_top;
        let (pos, size) = (self.pos, self.size);
        let children = self.children_mut();

        for child in children {
            // println!(" [{}] size = {:?}", child.id, child.size);
            let child_relative_pos = (
                match child.place.x {
                    PlacementAxis::Fixed(x) => match child.place.x_anchor {
                        Anchor::Min => x,
                        Anchor::Max => size.0 - x,
                    },
                    PlacementAxis::Float => match child.place.x_anchor {
                        Anchor::Min => {
                            let x = float_progress_x;
                            float_progress_x += child.size.0;
                            // println!(" ({}) Progress for {}: {}", id, child.id, float_progress_x);
                            x
                        }
                        _ => unimplemented!(),
                    },
                    PlacementAxis::Percentage(_x) => unimplemented!(),
                },
                match child.place.y {
                    PlacementAxis::Fixed(y) => match child.place.y_anchor {
                        Anchor::Min => y,
                        Anchor::Max => size.1 - y,
                    },
                    PlacementAxis::Float => match child.place.y_anchor {
                        Anchor::Min => {
                            let y = float_progress_y;
                            float_progress_y += child.size.1;
                            y
                        }
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                },
            );

            // println!(" ({}) Rel pos for  {}: {:?}", id, child.id, child_relative_pos);
            let new_pos = (child_relative_pos.0 + pos.0, child_relative_pos.1 + pos.1);
            if new_pos != child.pos {
                event!(WidgetEvent::ChangePos, (child, events));
            }
            child.pos = new_pos;
            child.pos.0 = child_relative_pos.0 + pos.0;
            child.pos.1 = child_relative_pos.1 + pos.1;
        }
        float_progress_x += self.padding_right;
        float_progress_y += self.padding_bot;

        let mut new_size = self.size;
        if self.size_hint_x == SizeHint::Minimize {
            new_size.0 = float_progress_x;
        }
        if self.size_hint_y == SizeHint::Minimize {
            new_size.1 = float_progress_y;
        }
        if new_size != self.size {
            self.size = new_size;
            event!(WidgetEvent::ChangeSize, (self, events));
        }

        events
    }
}

// TODO move to its own module. Problem with MOPA
/// An interactive component/node in the tree of widgets that defines a GUI. This is the trait that
/// all different widgets, such as buttons, checkboxes, containers, `Gui` itself, healthbars, ...,
/// implement.
pub trait Interactive: Any + std::fmt::Debug + Send + Sync {
    /// Create a Widget with this Interactive.
    fn wrap(self, id: String) -> Widget
    where
        Self: Sized,
    {
        Widget::new(id, self)
    }
    /// Defines an area which is considered "inside" a widget - for checking mouse hover etc.
    /// Provided implementation simply checks whether mouse is inside the boundaries, where `pos`
    /// is the very center of the widget. However, this is configurable in case a finer shape is
    /// desired (e.g. round things).
    fn inside(&self, pos: (f32, f32), size: (f32, f32), mouse: (f32, f32)) -> bool {
        let (x, y, w, h) = (pos.0, pos.1, size.0, size.1);
        let (top, bot, right, left) = (y, y + h, x + w, x);
        mouse.1 < bot && mouse.1 > top && mouse.0 > left && mouse.0 < right
    }
    /// Returns true if some internal state has changed in this widget (not in children)
    fn handle_event(&mut self, event: WidgetEvent) -> bool;

    /// Returns information whether this widget will stop mouse events and state
    /// to reach other parts of the application.
    fn captures(&self) -> Capture;

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &Widget> + 'a>;
    fn children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Widget> + 'a>;
    fn get_child(&self, id: &str) -> Option<&Widget>;
    fn get_child_mut(&mut self, id: &str) -> Option<&mut Widget>;
    fn insert_child(&mut self, w: Widget) -> Option<()>;

    fn recursive_children_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Widget> + 'a> {
        Box::new(
            self.children().chain(
                self.children()
                    .map(|child| child.recursive_children_iter())
                    .flatten(),
            ),
        )
    }
    fn default_size_hint(&self) -> (SizeHint, SizeHint) {
        (SizeHint::Minimize, SizeHint::Minimize)
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
    /// Change to any internal state
    Change,
    // TODO: perhaps something to notify that position has changed
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
