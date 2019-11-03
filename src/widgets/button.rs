use crate::*;

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
}
impl Button {
    pub fn new(text: String) -> Button {
        Button { text }
    }
}
impl Widget for Button {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToggleButton {
    pub text: String,
    pub state: bool,
}
impl ToggleButton {
    pub fn new(text: String) -> ToggleButton {
        ToggleButton { text, state: false }
    }
}
impl Widget for ToggleButton {
    fn handle_event(&mut self, event: WidgetEvent) -> bool {
        if let WidgetEvent::Release = event {
            self.state = !self.state;
            true
        } else {
            false
        }
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}
