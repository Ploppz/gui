use crate::*;
use indexmap::IndexMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct DropdownButton {
    children: IndexMap<String, Widget>,
    options: Vec<String>,
    opt_ids: IndexMap<String, String>,
}
impl DropdownButton {
    pub fn new(options: Vec<String>) -> DropdownButton {
        let id = format!("main-button#{}", Uuid::new_v4());
        let mut children = IndexMap::new();
        children.insert(id.clone(), ToggleButton::new(String::from("---")).wrap(id));
        DropdownButton {
            children,
            options,
            opt_ids: IndexMap::new(),
        }
    }
}
impl Interactive for DropdownButton {
    fn update(&mut self, events: &[(String, WidgetEvent)]) -> Vec<(String, WidgetEvent)> {
        let mut new_events = Vec::new();

        // Always ensure that all children have the same width

        for (id, event) in events {
            // Toggle dropdown list
            if id.starts_with("main-button#") {
                if *event == WidgetEvent::Change {
                    let toggled = self.children[id]
                        .downcast_ref::<ToggleButton>()
                        .unwrap()
                        .state;
                    if toggled {
                        for option in &self.options {
                            let id = format!("{}#{}", option, Uuid::new_v4());
                            self.opt_ids.insert(id.clone(), option.clone());
                            self.children
                                .insert(id.clone(), Button::new(option.clone()).wrap(id.clone()));
                            // new_events.push((id, WidgetEvent::Change));
                        }
                    } else {
                        self.children.retain(|id, _| id.starts_with("main-button#"));
                        self.opt_ids = IndexMap::new();
                    }
                }
            }

            // TODO NEXT: more logic

            /*
            if let Some(opt) = self.opt_ids.get(id) {
                // self.children.
                // TODO NEXT: change text
                self.children.get_mut("main-button")
                self.children.retain(|id, _| id.starts_with("main-button#"));
                self.opt_ids = IndexMap::new();
            }
            */
        }
        new_events
    }
    fn wrap(self, id: String) -> Widget {
        Widget::new(id, self)
            .padding(4.0, 4.0, 6.0, 6.0)
            .layout(Axis::Y, false, Anchor::Min, 2.0)
    }
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
