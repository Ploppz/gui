
use crate::*;

#[derive(Debug, Clone)]
pub struct Radio {
    pub text: String,
}
impl Radio {
    pub fn new(text: String) -> Radio {
        Radio { text }
    }
}
impl Widget for Radio {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadioButton {
    pub text: String,
    pub state: bool,
}
impl RadioButton {
    pub fn new(text: String) -> RadioButton {
        RadioButton { text }
    }
}
impl Widget for RadioButton {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        if let WidgetEvent::Release = event {
            if !self.state {
                self.state = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}
