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

mod widgets;

pub use widgets::*;
pub use Placement::*;

#[cfg(test)]
mod test;

#[derive(Copy, Clone)]
pub enum Placement<Id> {
    /// Relative from top left
    Pos(f32),
    /// Relative to screen from right or bottom
    Neg(f32),
    /// Relative to another widget
    FromWidget(Id, f32),
}

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

}
mopafy!(Widget);

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WidgetEvent {
    Press, // TODO
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

#[derive(Deref, DerefMut)]
pub struct WidgetInternal<Id> {
    #[deref_target]
    pub widget: Box<dyn Widget>,
    pub pos: (f32, f32),
    pub size: (f32, f32),

    /// Relative x position as declared
    place_x: Placement<Id>,
    /// Relative y position as declared
    place_y: Placement<Id>,

    /// Keeps track of hover state in order to generate the right WidgetEvents
    inside: bool,
    pressed: bool,
}

#[derive(Default)]
pub struct Gui<Id: Eq + Hash> {
    pub widgets: HashMap<Id, WidgetInternal<Id>>,
    screen: (f32, f32),
    events: Vec<(Id, WidgetEventState)>,
}

impl<Id: Eq + Hash + Clone> Gui<Id> {
    pub fn insert<W: Widget + 'static>(
        &mut self,
        id: Id,
        widget: W,
        place_x: Placement<Id>,
        place_y: Placement<Id>,
    ) {
        self.widgets.insert(
            id,
            WidgetInternal {
                widget: Box::new(widget),
                pos: (0.0, 0.0),
                size: (0.0, 0.0), // TODO Widget::default_size()?
                place_x,
                place_y,
                inside: false,
                pressed: false,
            },
        );
    }
    pub fn mark_change(&mut self, id: Id) {
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
    ) -> (Vec<(Id, WidgetEventState)>, Capture) {
        self.screen = (sw, sh);

        // Update positions
        let mut updated_positions = HashMap::new();
        let ids: Vec<Id> = self.widgets.keys().cloned().collect();
        for id in &ids {
            let pos = self.update_position(id.clone(), &mut updated_positions);
            self.widgets.get_mut(&id).unwrap().pos = pos;
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
            // TODO release
        }

        let events = std::mem::replace(&mut self.events, vec![]);
        (events, capture)
    }

    fn update_position(&mut self, id: Id, positions: &mut HashMap<Id, (f32, f32)>) -> (f32, f32) {
        // TODO: look at width, height for relative positions
        if let Some(pos) = positions.get(&id) {
            return *pos;
        }

        let WidgetInternal {
            ref place_x,
            ref place_y,
            ..
        } = self.widgets[&id];
        let (place_x, place_y) = (place_x.clone(), place_y.clone());
        let x = match place_x {
            Placement::Pos(offset) => offset,
            Placement::Neg(offset) => self.screen.0 - offset,
            Placement::FromWidget(other_id, offset) => {
                if let Some(pos) = positions.get(&other_id) {
                    offset + pos.0
                } else {
                    offset + self.update_position(other_id.clone(), positions).0
                }
            }
        };
        let y = match place_y {
            Placement::Pos(offset) => offset,
            Placement::Neg(offset) => self.screen.1 - offset,
            Placement::FromWidget(other_id, offset) => {
                if let Some(pos) = positions.get(&other_id) {
                    offset + pos.1
                } else {
                    offset + self.update_position(other_id.clone(), positions).1
                }
            }
        };
        positions.insert(id, (x, y));
        (x, y)
    }
}
