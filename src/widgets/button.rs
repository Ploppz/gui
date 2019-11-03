use crate::*;

#[derive(Debug)]
pub struct Button {
    pub text: WidgetInternal,
}
impl Button {
    pub fn new(text: String) -> Button {
        Button {
            text: WidgetInternal::new(TextField::new(text), Abs (Pos(0.0), Pos(0.0)))
        }
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
    fn children(&mut self) -> Vec<&mut WidgetInternal> {
        vec![&mut self.text]
    }
}

#[derive(Debug)]
pub struct ToggleButton {
    pub text: WidgetInternal,
    pub state: bool,
}
impl ToggleButton {
    pub fn new(text: String) -> ToggleButton {
        ToggleButton {
            text: WidgetInternal::new(TextField::new(text), Abs (Pos(0.0), Pos(0.0))),
            state: false,
        }

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
    fn children(&mut self) -> Vec<&mut WidgetInternal> {
        vec![&mut self.text]
    }
}
