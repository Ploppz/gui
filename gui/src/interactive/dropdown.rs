use crate::*;
use gui_derive::Lenses;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
struct DropdownOption {
    pub name: String,
    pub value: String,
}

#[derive(Lenses, Debug)]
pub struct DropdownButton {
    options: Vec<DropdownOption>,
    value: Option<String>,
    /// map from ID to option index
    opt_map: IndexMap<Id, usize>,
    main_button_id: usize,
}
impl DropdownButton {
    pub fn new() -> DropdownButton {
        DropdownButton {
            options: Vec::new(),
            value: None,
            opt_map: IndexMap::new(),
            main_button_id: 0,
        }
    }
    pub fn option(mut self, name: String, value: String) -> Self {
        self.options.push(DropdownOption { name, value });
        self
    }
    pub fn close(&mut self, children: &mut ChildrenProxy) {
        let to_remove = children.keys().cloned().collect::<Vec<_>>();
        for id in to_remove {
            if id != self.main_button_id {
                children.remove(id);
            }
        }
        self.opt_map = IndexMap::new();
    }
}
impl Interactive for DropdownButton {
    fn init(&mut self, children: &mut ChildrenProxy) -> WidgetConfig {
        let main_id = children.insert(Box::new(ToggleButton::new()) as Box<dyn Interactive>);
        children.get_mut(main_id).config.set_height(24.0);
        self.main_button_id = main_id;
        WidgetConfig::default()
            .padding(4.0, 4.0, 6.0, 6.0)
            .layout(Axis::Y, false, Anchor::Min, 2.0)
    }
    fn update(
        &mut self,
        id: Id,
        local_events: &[Event],
        children: &mut ChildrenProxy,
        events: &mut Vec<Event>,
    ) {
        // Always ensure that all children have the same width

        for Event { id, kind } in events {
            // Toggle dropdown list
            if *id == self.main_button_id {
                if kind.is_change(ToggleButton::state) {
                    let toggled = children[id].downcast_ref::<ToggleButton>().unwrap().state;
                    if toggled {
                        for (i, option) in self.options.iter().enumerate() {
                            let mut btn = Button::new();
                            let id = children.insert(Box::new(btn));
                            // TODO: set text
                            //  - figure out lenses that go further..
                            /*
                            InternalLens::new(children.get_mut(id), events)
                                .put(
                            */
                            // Button::text.put(children.get_mut(id), option.name.clone());
                            self.opt_map.insert(id, i);
                        }
                    } else {
                        self.close(children);
                    }
                }
            }

            if let Some(opt_idx) = self.opt_map.get(id) {
                if *kind == EventKind::Press {
                    let opt = self.options[*opt_idx].clone();
                    let btn = children.get_mut(self.main_button_id);
                    InternalLens::new(btn, events)
                        .put(ToggleButton::text, opt.name.clone())
                        .put(ToggleButton::state, false);
                    self.value = Some(opt.value.clone());

                    self.close(children);
                }
            }
        }
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}
