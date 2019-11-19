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
    fn children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Widget> + 'a> {
        Box::new(std::iter::empty())
    }
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &Widget> + 'a> {
        Box::new(std::iter::empty())
    }
    fn get_child(&mut self, _id: &str) -> Option<&mut Widget> {
        None
    }
    fn insert_child(&mut self, _w: Widget) -> Option<()> {
        None
    }

    fn default_size_hint(&self) -> (SizeHint, SizeHint) {
        (SizeHint::External, SizeHint::External)
    }
}
