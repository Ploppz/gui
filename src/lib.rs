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
pub use Placement::*;
pub use AbsPlacement::*;

#[cfg(test)]
mod test;


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
    /// Returns true if some internal state has changed
    fn handle_event(&mut self, event: WidgetEvent) -> bool;

    /// Returns information whether this widget will stop mouse events and state
    /// to reach other parts of the application.
    fn captures(&self) -> Capture;

    fn children(&mut self) -> Vec<(&str, &mut WidgetInternal)>;

}
mopafy!(Widget);


#[derive(Default)]
pub struct Gui {
    pub widgets: HashMap<String, WidgetInternal>,
    screen: (f32, f32),
    events: Vec<(String, WidgetEventState)>,
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
    pub fn mark_change(&mut self, id: String) {
        let widget = &self.widgets[&id];
        self.events.push((
            id,
            WidgetEventState {
                pressed: widget.pressed,
                hover: widget.inside,
                event: WidgetEvent::Change,
            },
        ));
    }
    pub fn update(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: (f32, f32),
    ) -> (Vec<(String, WidgetEventState)>, Capture) {
        self.screen = (sw, sh);

        // Update positions
        update_position(self.widgets.values_mut().collect(), self.screen);

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

        // Update each widget
        let mut capture = Capture::default();
        for (id, w) in self.widgets.iter_mut() {
            let now_inside = w.inside(w.pos, w.size, mouse);
            let prev_inside = w.inside;
            w.inside = now_inside;

            if now_inside && !prev_inside {
                event!(WidgetEvent::Hover, (w, id, self.events));
            } else if prev_inside && !now_inside {
                event!(WidgetEvent::Unhover, (w, id, self.events));
            }

            if now_inside {
                capture |= w.widget.captures();
            }

            if now_inside && input.is_mouse_button_toggled_down(winit::event::MouseButton::Left) {
                w.pressed = true;
                event!(WidgetEvent::Press, (w, id, self.events));
            }
            if w.pressed && input.is_mouse_button_toggled_up(winit::event::MouseButton::Left) {
                w.pressed = false;
                event!(WidgetEvent::Release, (w, id, self.events));
            }
        }

        let events = std::mem::replace(&mut self.events, vec![]);
        (events, capture)
    }

}
fn update_position(widgets: Vec<&mut WidgetInternal>, screen: (f32, f32)) {
    // TODO: look at width, height for relative positions
    let mut _pos = 0;
    for widget in widgets {
        // TODO relative/float placements
        let pos = match widget.place {
            Abs (x, y) => {
                let x = match x {
                    Pos (offset) => offset,
                    Neg (offset) => screen.0 - offset,
                };
                let y = match y {
                    Pos (offset) => offset,
                    Neg (offset) => screen.1 - offset,
                };
                (x, y)
            }
            _ => unimplemented!(),
        };
        update_position(widget.widget.children().into_iter().map(|(id, w)| w).collect(), screen);
        widget.pos = pos;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Placement {
    /// Absolute position
    Abs(AbsPlacement, AbsPlacement),
    FloatVertical,
    FloatHorizontal,
}
#[derive(Copy, Clone, Debug)]
pub enum AbsPlacement {
    Pos(f32),
    Neg(f32),
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

#[derive(Deref, DerefMut, Debug)]
pub struct WidgetInternal {
    #[deref_target]
    pub widget: Box<dyn Widget>,
    pub pos: (f32, f32),
    pub size: (f32, f32),

    /// Declarative placement (used to calculate position)
    pub place: Placement,

    /// Keeps track of hover state in order to generate the right WidgetEvents
    inside: bool,
    pressed: bool,
}
impl WidgetInternal {
    pub fn new<W: Widget>(widget: W, place: Placement) -> WidgetInternal {
        WidgetInternal {
            widget: Box::new(widget),
            pos: (0.0, 0.0),
            size: (10.0, 10.0), // TODO Widget::default_size()?
            place,
            inside: false,
            pressed: false,
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
