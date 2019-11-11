use crate::*;
use uuid::Uuid;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Button {
    children: IndexMap<String, Widget>,
}
impl Button {
    pub fn new(text: String) -> Button {
        let id = Uuid::new_v4().to_string();
        let mut children = IndexMap::new();
        children.insert(id.clone(),
            Widget::new(id, TextField::new(text)).placement(Placement::fixed(0.0, 0.0)));
        Button {
            children
        }
    }
    /// Wrap in a `Widget` 
    pub fn wrap(self) -> Widget {
        Widget::new(String::new(), self)
    }
}
impl Interactive for Button {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
    fn children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&mut Widget> + 'a> {
        Box::new(self.children.values_mut())
    }
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item=&Widget> + 'a> {
        Box::new(self.children.values())
    }
    fn get_child(&mut self, id: &str) -> Option<&mut Widget> {
        self.children.get_mut(id)
    }
    fn insert_child(&mut self, id: String, w: Widget) -> Option<()> {
        self.children.insert(id, w);
        Some(())
    }
    fn default_size_hint(&self) -> SizeHint {
        SizeHint::Minimize {top: 5.0, bot: 5.0, left: 8.0, right: 8.0}
    }
}

#[derive(Debug)]
pub struct ToggleButton {
    pub state: bool,
    children: IndexMap<String, Widget>,
}
impl ToggleButton {
    pub fn new(text: String) -> ToggleButton {
        let id = Uuid::new_v4().to_string();
        let mut children = IndexMap::new();
        children.insert(id.clone(),
            TextField::new(text)
                .wrap()
                .placement(Placement::fixed(0.0, 0.0)));
        ToggleButton {
            children,
            state: false,
        }
    }
    /// Wrap in a `Widget` 
    pub fn wrap(self) -> Widget {
        Widget::new(String::new(), self)
    }
}
impl Interactive for ToggleButton {
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
    fn children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&mut Widget> + 'a> {
        Box::new(self.children.values_mut())
    }
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item=&Widget> + 'a> {
        Box::new(self.children.values())
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
