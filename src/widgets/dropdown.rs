use crate::*;
use indexmap::IndexMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct DropdownButton {
    children: IndexMap<String, Widget>,
}
impl DropdownButton {
    pub fn new(options: Vec<String>) -> DropdownButton {
        let id = Uuid::new_v4().to_string();
        let mut children = IndexMap::new();
        children.insert(id.clone(), ToggleButton::new(String::new()).wrap(id));
        DropdownButton { children }
    }
}
impl Interactive for DropdownButton {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
    fn children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Widget> + 'a> {
        Box::new(self.children.values_mut())
    }
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &Widget> + 'a> {
        Box::new(self.children.values())
    }
    fn get_child(&self, id: &str) -> Option<&Widget> {
        self.children.get(id)
    }
    fn get_child_mut(&mut self, id: &str) -> Option<&mut Widget> {
        self.children.get_mut(id)
    }
    fn insert_child(&mut self, w: Widget) -> Option<()> {
        self.children.insert(w.get_id().to_string(), w);
        Some(())
    }
}
