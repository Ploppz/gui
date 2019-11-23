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
    fn children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Widget> + 'a> {
        Box::new(self.children.values_mut())
    }
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &Widget> + 'a> {
        Box::new(self.children.values())
    }
    fn get_child_mut(&mut self, id: &str) -> Option<&mut Widget> {
        self.children.get_mut(id)
    }
    fn get_child(&self, id: &str) -> Option<&Widget> {
        self.children.get(id)
    }
    fn insert_child(&mut self, w: Widget) -> Option<()> {
        self.children.insert(w.get_id().to_string(), w);
        Some(())
    }
}
