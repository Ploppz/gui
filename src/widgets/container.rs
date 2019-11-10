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
    fn children<'a>(&'a mut self) -> Box<dyn Iterator<Item=&mut Widget> + 'a> {
        Box::new(self.children.values_mut())
    }
    fn get_child(&mut self, id: &str) -> Option<&mut Widget> {
        self.children.get_mut(id)
    }
    fn insert_child(&mut self, id: String, w: Widget) -> Option<()> {
        self.children.insert(id, w);
        Some(())
    }
    fn default_size_hint(&self) -> SizeHint {
        SizeHint::Minimize {top: 2.0, bot: 2.0, left: 2.0, right: 2.0}
    }
}
