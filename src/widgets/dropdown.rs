use crate::*;
use indexmap::IndexMap;

pub struct DropdownValueLens;
impl Lens<Widget, String> for DropdownValueLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, w: &Widget, f: F) -> V {
        let text = &w
            .children()
            .values()
            .next()
            .unwrap()
            .downcast_ref::<TextField>()
            .unwrap()
            .text;
        f(text)
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, w: &mut Widget, f: F) -> V {
        let mut proxy = w.children_proxy();
        let text = &mut proxy
            .values_mut()
            .next()
            .unwrap()
            .downcast_mut::<TextField>()
            .unwrap()
            .text;
        let old_text = text.clone();
        let result = f(text);
        if old_text != *text {
            w.mark_change();
        }
        result
    }
}

#[derive(Debug, Clone)]
struct DropdownOption {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
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
        events: &[(Id, WidgetEvent)],
        children: &mut ChildrenProxy,
    ) -> Vec<(Id, WidgetEvent)> {
        let new_events = Vec::new();

        // Always ensure that all children have the same width

        for (id, event) in events {
            // Toggle dropdown list
            if *id == self.main_button_id {
                if *event == WidgetEvent::Change {
                    let toggled = children[id].downcast_ref::<ToggleButton>().unwrap().state;
                    if toggled {
                        for (i, option) in self.options.iter().enumerate() {
                            let mut btn = Button::new();
                            btn.text = option.name.clone();
                            let id = children.insert(Box::new());
                            // Button::text.put(children.get_mut(id), option.name.clone());
                            self.opt_map.insert(id, i);
                        }
                    } else {
                        self.close(children);
                    }
                }
            }

            // TODO NEXT: more logic

            if let Some(opt_idx) = self.opt_map.get(id) {
                if *event == WidgetEvent::Press {
                    let opt = self.options[*opt_idx].clone();
                    let btn = children.get_mut(self.main_button_id);
                    ToggleButton::text.with_mut(btn, |text| {
                        *text = opt.name.clone();
                        println!("SEt text to {}", *text);
                    });
                    ToggleButton::state.with_mut(btn, |state| *state = false);
                    self.value = Some(opt.value.clone());

                    self.close(children);
                }
            }
        }
        new_events
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
