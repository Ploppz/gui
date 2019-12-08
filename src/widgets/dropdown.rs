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
    options: Vec<DropdownOption>,
    /// map from ID to option index
    opt_map: IndexMap<Id, usize>,
    main_button_id: usize,
}
impl DropdownButton {
    pub fn new() -> DropdownButton {
        // children.insert(id.clone(), ToggleButton::new(String::from("---")).wrap(unimplemented!()));
        DropdownButton {
            options: Vec::new(),
            opt_map: IndexMap::new(),
            main_button_id: 0,
        }
    }
    pub fn option(mut self, name: String, value: String) -> Self {
        self.options.push(DropdownOption { name, value });
        self
    }
}
impl Interactive for DropdownButton {
    fn update(
        &mut self,
        events: &[(Id, WidgetEvent)],
        children: &mut IndexMap<Id, Widget>,
    ) -> Vec<(Id, WidgetEvent)> {
        let new_events = Vec::new();

        // Always ensure that all children have the same width

        for (id, event) in events {
            // Toggle dropdown list
            if *id == self.main_button_id {
                if *event == WidgetEvent::Change {
                    let toggled = children[id].downcast_ref::<ToggleButton>().unwrap().state;
                    if toggled {
                        for option in &self.options {
                            // children.insert(
                            // id,
                            // Button::new(option.name.clone()).wrap(id.clone()),
                            // );
                            // new_events.push((id, WidgetEvent::Change));
                        }
                    } else {
                        children.retain(|id, _| *id == self.main_button_id);
                    }
                }
            }

            // TODO NEXT: more logic

            /*
            if let Some(opt) = self.opt_ids.get(id) {
                // children.
                // TODO NEXT: change text
                children.get_mut("main-button")
                children.retain(|id, _| id.starts_with("main-button#"));
                self.opt_ids = IndexMap::new();
            }
            */
        }
        new_events
    }
    fn wrap(self, id: Id) -> Widget {
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
}
