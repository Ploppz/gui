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
            Widget::new(id, TextField::new(text), Placement::fixed(0.0, 0.0)));
        Button {
            children
        }
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
    fn children(&mut self) -> &mut IndexMap<String, Widget> {
        &mut self.children
    }
    fn default_size_hint(&self) -> SizeHint {
        SizeHint::Minimize {top: 2.0, bot: 2.0, left: 2.0, right: 2.0}
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
            Widget::new(id, TextField::new(text), Placement::fixed(0.0, 0.0))
            );
        ToggleButton {
            children,
            state: false,
        }

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
    fn children(&mut self) -> &mut IndexMap<String, Widget> {
        &mut self.children
    }
}
