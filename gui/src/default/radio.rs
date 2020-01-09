
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
impl Interactive for Radio {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
    fn children(&mut self) -> Vec<&mut WidgetInternal> {
        vec![] // TODO radio buttons
    }
}

#[derive(Debug, Clone)]
pub struct RadioButton {
    pub text: String,
    pub state: bool,
}
impl RadioButton {
    pub fn new(text: String) -> RadioButton {
        RadioButton {
            text,
            state: false,
        }
    }
}
impl Interactive for RadioButton {
    fn handle_event(&mut self, event: WidgetEvent) -> bool {
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
    fn children(&mut self) -> Vec<&mut WidgetInternal> {
        vec![] // TODO text field
    }
}
