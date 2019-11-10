use crate::*;
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct TextField {
    pub text: String,
}
impl TextField {
    pub fn new(text: String) -> TextField {
        TextField { text }
    }
}
impl Interactive for TextField {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
    fn children(&mut self) -> &mut IndexMap<String, Widget> {
        panic!("Text field cannot have children")
    }
}
