use crate::*;

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub w: f32,
    pub h: f32,
    pub state: ButtonState,
}
impl Button {
    pub fn new(text: String, w: f32, h: f32) -> Button {
        Button {
            text,
            w,
            h,
            state: ButtonState::None,
        }
    }
}
impl Widget for Button {
    fn update(&mut self, input: &Input, x: f32, y: f32, mx: f32, my: f32) -> Option<Box<dyn Event>> {
        let (top, bot, right, left) = (y + self.h/2.0, y - self.h/2.0, x + self.w/2.0, x - self.w/2.0);
        let inside = my > bot && my < top && mx > left && mx < right;
        if inside {
            self.state = ButtonState::Hover;
            if input.is_mouse_button_toggled_up(winit::event::MouseButton::Left) {
                Some(Box::new(ButtonPress) as Box<dyn Event>)
            } else {
                None
            }
        } else {
            self.state = ButtonState::None;
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Hover,
    None,
}

#[derive(Debug)]
pub struct ButtonPress;
impl Event for ButtonPress {}
