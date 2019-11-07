//! # Notes and thoughts
//!
//! - In the future, we can have a `trait Event: Any` and let `Widget::handle_event` return
//! `Box<dyn Event>` to support more custom/complex events than just the basic mouse events.

#[macro_use]
extern crate mopa;
#[macro_use]
extern crate derive_deref;
use mopa::Any;
use std::collections::HashMap;
use std::hash::Hash;
use winput::Input;
use uuid::Uuid;

mod widgets;

pub use widgets::*;

#[cfg(test)]
mod test;

#[derive(Deref, DerefMut, Debug)]
pub struct WidgetInternal {
    #[deref_target]
    pub widget: Box<dyn Widget>,
    pub pos: (f32, f32),
    pub size: (f32, f32),

    /// Declarative placement (used to calculate position)
    pub place: Placement,
    pub anchor: (Anchor, Anchor),

    /// Keeps track of hover state in order to generate the right WidgetEvents
    inside: bool,
    /// Keeps track of mouse press state in order to generate the right WidgetEvents
    pressed: bool,

    /// 'Buffer' - when `true` it is set to `false` by the parent, and the 
    changed: bool,
}

impl WidgetInternal {
    pub fn new<W: Widget>(widget: W, place: Placement) -> WidgetInternal {
        WidgetInternal {
            widget: Box::new(widget),
            pos: (0.0, 0.0),
            size: (10.0, 10.0), // TODO Widget::default_size()?
            place,
            anchor: (Anchor::Min, Anchor::Min),
            inside: false,
            pressed: false,
            changed: false,
        }
    }
    /// Mark that some internal state has changed in this Widget.
    /// For use when an application itself wants to change state of a Widget - for example toggle a
    /// button in response to a key press. A `Change` event has to be registered so that the drawer
    /// knows to redraw the widget.
    pub fn mark_change(&mut self) {
        self.changed = true;
    }

}

macro_rules! event {
    ($event:expr, ($widget:expr, $id:expr, $events:expr)) => {{
        let change = $widget.widget.handle_event($event);
        if change {
            $events.push((
                $id.clone(),
                WidgetEventState {
                    pressed: $widget.pressed,
                    hover: $widget.inside,
                    event: WidgetEvent::Change,
                },
            ));
        }
        $events.push((
            $id.clone(),
            WidgetEventState {
                pressed: $widget.pressed,
                hover: $widget.inside,
                event: $event,
            },
        ));
    }};
}

// TODO move to `widget` module. Problem with MOPA
pub trait Widget: Any + std::fmt::Debug + Send + Sync {
    // fn update(&mut self, _: &Input, x: f32, y: f32, mx: f32, my: f32) -> Option<Box<dyn Event>>;
    // PROTOTYPING:
    /// Defines an area which is considered "inside" a widget - for checking mouse hover etc.
    /// Provided implementation simply checks whether mouse is inside the boundaries, where `pos`
    /// is the very center of the widget. However, this is configurable in case a finer shape is
    /// desired (e.g. round things).
    fn inside(&self, pos: (f32, f32), size: (f32, f32), mouse: (f32, f32)) -> bool {
        let (x, y, w, h) = (pos.0, pos.1, size.0, size.1);
        let (top, bot, right, left) = (y + h / 2.0, y - h / 2.0, x + w / 2.0, x - w / 2.0);
        mouse.1 > bot && mouse.1 < top && mouse.0 > left && mouse.0 < right
    }
    /// Returns true if some internal state has changed in this widget (not in children)
    fn handle_event(&mut self, event: WidgetEvent) -> bool;

    /// Returns information whether this widget will stop mouse events and state
    /// to reach other parts of the application.
    fn captures(&self) -> Capture;

    fn children(&mut self) -> Vec<(&str, &mut WidgetInternal)>;

    fn update(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: (f32, f32),
    ) -> (Vec<(String, WidgetEventState)>, Capture) {
        // Update positions
        update_position(self.children(), (sw, sh));

        let mut events = Vec::new();
        let children = self.children();

        // Update each widget
        let mut capture = Capture::default();
        for (id, w) in children {
            let id = id.to_string();
            let now_inside = w.inside(w.pos, w.size, mouse);
            let prev_inside = w.inside;
            w.inside = now_inside;

            if now_inside && !prev_inside {
                event!(WidgetEvent::Hover, (w, id, events));
            } else if prev_inside && !now_inside {
                event!(WidgetEvent::Unhover, (w, id, events));
            }

            if now_inside {
                capture |= w.widget.captures();
            }

            if now_inside && input.is_mouse_button_toggled_down(winit::event::MouseButton::Left) {
                w.pressed = true;
                event!(WidgetEvent::Press, (w, id, events));
            }
            if w.pressed && input.is_mouse_button_toggled_up(winit::event::MouseButton::Left) {
                w.pressed = false;
                event!(WidgetEvent::Release, (w, id, events));
            }

            if w.changed {

                events.push((
                    id.clone(),
                    WidgetEventState {
                        pressed: w.pressed,
                        hover: w.inside,
                        event: WidgetEvent::Change,
                    },
                ));
                w.changed = false;
            }
        }

        let events = std::mem::replace(&mut events, vec![]);
        (events, capture)
    }

}
mopafy!(Widget);


#[derive(Default, Debug)]
pub struct Gui {
    pub widgets: HashMap<String, WidgetInternal>,
    screen: (f32, f32),
}

impl Gui {
    pub fn insert<W: Widget + 'static>(
        &mut self,
        id: String,
        widget: W,
        place: Placement,
    ) {
        self.widgets.insert(id, WidgetInternal::new(widget, place));
    }
}

impl Widget for Gui {
    fn inside(&self, pos: (f32, f32), size: (f32, f32), mouse: (f32, f32)) -> bool {
        true
    }
    fn handle_event(&mut self, event: WidgetEvent) -> bool {
        panic!("There is no reason to use this on the root")
    }
    fn captures(&self) -> Capture {
        panic!("There is no reason to use this on the root")
    }
    fn children(&mut self) -> Vec<(&str, &mut WidgetInternal)> {
        self.widgets.iter_mut().map(|(id, w)| (id.as_str(), w)).collect()
    }

}

fn update_position(widgets: Vec<(&str, &mut WidgetInternal)>, screen: (f32, f32)) {
    let mut float_progress = 0.0;
    // TODO: look at width, height for relative positions
    for (_id, widget) in widgets {
        // TODO relative/float placements
        let pos = match widget.place {
            Placement::Fixed (Position {x, y, x_anchor, y_anchor}) => (
                match x_anchor {
                    Anchor::Min => x,
                    Anchor::Max => screen.0 - x,
                    Anchor::Center => unimplemented!(),
                },

                match y_anchor {
                    Anchor::Min => y,
                    Anchor::Max => screen.1 - y,
                    Anchor::Center => unimplemented!(),
                }),
            Placement::Float (axis, anchor) => {
                if let (Axis::X, Anchor::Min) = (axis, anchor) {
                    float_progress += widget.size.0;
                    (float_progress - widget.size.0 / 2.0, 0.0)
                } else {
                    unimplemented!();
                }
            }
            Placement::Percentage (x, y) => unimplemented!(),
        };
        update_position(widget.widget.children(), screen);
        widget.pos = pos;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Placement {
    Percentage (f32, f32),
    Fixed (Position),
    Float (Axis, Anchor),
}
impl Placement {
    pub fn fixed(x: f32, y: f32) -> Self {
        Self::Fixed (Position {x, y, x_anchor: Anchor::Min, y_anchor: Anchor::Min})
    }
    pub fn x_anchor(self, a: Anchor) -> Self {
        if let Self::Fixed(Position {mut x_anchor, ..}) =  self {
            x_anchor = a
        }
        self
    }
    pub fn y_anchor(self, a: Anchor) -> Self {
        if let Self::Fixed(Position {mut y_anchor, ..}) =  self {
            y_anchor = a
        }
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: f32,
    y: f32,
    x_anchor: Anchor,
    y_anchor: Anchor,
}
#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    Min,
    Max,
    Center,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WidgetEvent {
    Press,
    Release,
    Hover,
    Unhover,
    /// Change to any internal state
    Change,
    // TODO: perhaps something to notify that position has changed
}

#[derive(Clone, Debug)]
pub struct WidgetEventState {
    pub hover: bool,
    pub pressed: bool,
    pub event: WidgetEvent,
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
