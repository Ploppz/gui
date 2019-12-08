use crate::*;
use indexmap::IndexMap;
use uuid::Uuid;

#[derive(Debug)]
struct DropdownOption {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct DropdownButton {
    children: IndexMap<String, Widget>,
    options: Vec<DropdownOption>,
    /// map from ID to option index
    opt_map: IndexMap<String, usize>,
}
impl DropdownButton {
    pub fn new() -> DropdownButton {
        let id = format!("main-button#{}", Uuid::new_v4());
        let mut children = IndexMap::new();
        children.insert(id.clone(), ToggleButton::new(String::from("---")).wrap(id));
        DropdownButton {
            children,
            options: Vec::new(),
            opt_map: IndexMap::new(),
        }
    }
    pub fn option(mut self, name: String, value: String) -> Self {
        self.options.push(DropdownOption { name, value });
        self
    }
}
impl Interactive for DropdownButton {
    fn update(&mut self, events: &[(String, WidgetEvent)]) -> Vec<(String, WidgetEvent)> {
        let new_events = Vec::new();

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
                            let id = Uuid::new_v4().to_string();
                            self.children.insert(
                                id.clone(),
                                Button::new(option.name.clone()).wrap(id.clone()),
                            );
                            // new_events.push((id, WidgetEvent::Change));
                        }
                    } else {
                        self.children.retain(|id, _| id.starts_with("main-button#"));
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
    fn children<'a>(&'a self) -> &IndexMap<String, Widget> {
        &self.children
    }
    fn children_mut<'a>(&'a mut self) -> &mut IndexMap<String, Widget> {
        &mut self.children
    }
}
