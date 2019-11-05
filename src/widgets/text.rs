use crate::*;

#[derive(Debug, Clone)]
pub struct TextField {
    pub text: String,
}
impl TextField {
    pub fn new(text: String) -> TextField {
        TextField { text }
    }
}
impl Widget for TextField {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
    fn children(&mut self) -> Vec<(&str, &mut WidgetInternal)> {
        vec![]
    }
}
