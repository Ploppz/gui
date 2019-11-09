use crate::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Button {
    text_id: String,
    pub text: WidgetInternal,
}
impl Button {
    pub fn new(text: String) -> Button {
        Button {
            text_id: Uuid::new_v4().to_string(),
            text: WidgetInternal::new(TextField::new(text), Placement::fixed(0.0, 0.0))
        }
    }
}
impl Interactive for Button {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
    fn children(&mut self) -> Vec<(&str, &mut WidgetInternal)> {
        vec![(&self.text_id, &mut self.text)]
    }
    fn default_size_hint(&self) -> SizeHint {
        SizeHint::Minimize {top: 2.0, bot: 2.0, left: 2.0, right: 2.0}
    }
}

#[derive(Debug)]
pub struct ToggleButton {
    text_id: String,
    pub text: WidgetInternal,
    pub state: bool,
}
impl ToggleButton {
    pub fn new(text: String) -> ToggleButton {
        ToggleButton {
            text_id: Uuid::new_v4().to_string(),
            text: WidgetInternal::new(TextField::new(text), Placement::fixed(0.0, 0.0)),
            state: false,
        }

    }
}
impl Interactive for ToggleButton {
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
    fn children(&mut self) -> Vec<(&str, &mut WidgetInternal)> {
        vec![(&self.text_id, &mut self.text)]
    }
}
