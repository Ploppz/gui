use crate::*;

#[derive(Debug, Clone)]
pub struct TextField {
    pub text: String,
}
impl TextField {
    pub fn new(text: String) -> TextField {
        TextField { text }
    }
    /// Wrap in a `Widget` 
    pub fn wrap(self) -> Widget {
        Widget::new(String::new(), self)
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
    fn children<'a>(&'a mut self) -> Box<dyn Iterator<Item=&mut Widget> + 'a> {
        Box::new(std::iter::empty())
    }
    fn get_child(&mut self, _id: &str) -> Option<&mut Widget> {
        None
    }
    fn insert_child(&mut self, _id: String, _w: Widget) -> Option<()> {
        None
    }
}
