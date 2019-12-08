use crate::*;

#[derive(Debug)]
pub struct TextField {
    pub text: String,
    children: IndexMap<String, Widget>,
}
impl TextField {
    pub fn new(text: String) -> TextField {
        TextField {
            text,
            children: IndexMap::new(),
        }
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
    fn children<'a>(&'a self) -> &IndexMap<String, Widget> {
        &self.children
    }
    fn children_mut<'a>(&'a mut self) -> &mut IndexMap<String, Widget> {
        &mut self.children
    }

    fn default_size_hint(&self) -> (SizeHint, SizeHint) {
        (SizeHint::External, SizeHint::External)
    }
}
