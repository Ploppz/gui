use crate::*;
use indexmap::IndexMap;

#[derive(Debug, Default)]
pub struct Container {
    children: IndexMap<String, Widget>,
}
impl Container {
    pub fn new() -> Container {
        Container {
            children: IndexMap::new(),
        }
    }
}
impl Interactive for Container {
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
}
