use crate::*;
use indexmap::IndexMap;

pub trait DropdownButtonTrait: Default {
    fn internal(&mut self) -> &mut DropdownButtonInternal;

    fn option(mut self, name: String, value: String) -> Self {
        self.internal().options.push(DropdownOption { name, value });
        self
    }
    fn close(&mut self, children: &mut ChildrenProxy, gui: &GuiShared) {
        let internal = self.internal();
        let to_remove = children.keys().cloned().collect::<Vec<_>>();
        for id in to_remove {
            if id != internal.main_button_id {
                children.remove(id, gui);
            }
        }
        internal.opt_map = IndexMap::new();
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DropdownOption {
    pub name: String,
    pub value: String,
}

#[derive(Lens, Debug)]
pub struct DropdownButtonInternal {
    options: Vec<DropdownOption>,
    value: Option<String>,
    /// map from ID to option index
    opt_map: IndexMap<Id, usize>,
    main_button_id: usize,
}

impl<T: DropdownButtonTrait> Interactive for T {
    fn init(&mut self, children: &mut ChildrenProxy, gui: &GuiShared) -> WidgetConfig {
        let s = self.internal();
        let main_id = children.insert(Box::new(ToggleButton::new()) as Box<dyn Interactive>, gui);
        children.get_mut(main_id).config.set_height(24.0);
        s.main_button_id = main_id;
        WidgetConfig::default()
            .padding(4.0, 4.0, 6.0, 6.0)
            .layout(Axis::Y, false, Anchor::Min, 2.0)
    }
    fn update(
        &mut self,
        _id: Id,
        local_events: Vec<Event>,
        children: &mut ChildrenProxy,
        gui: &GuiShared,
    ) {
        // Always ensure that all children have the same width

        for Event { id, kind } in local_events.iter().cloned() {
            // Toggle dropdown list
            if id == self.internal().main_button_id {
                if kind.is_change(ToggleButton::state) {
                    let toggled = *children[&id].access(gui.clone())
                        .chain(ToggleButton::state)
                        .get();
                    if toggled {
                        for (i, option) in self.internal().options.iter().enumerate() {
                            let id = children.insert(Box::new(Button::new()), &gui);

                            children
                                .get_mut(id)
                                .access(gui.clone())
                                .chain(Widget::first_child)
                                .chain(TextField::text)
                                .put(option.name.clone());

                            // Button::text.put(children.get_mut(id), option.name.clone());
                            self.internal().opt_map.insert(id, i);
                        }
                    } else {
                        self.internal().close(children, gui);
                    }
                }
            }

            if let Some(opt_idx) = self.internal().opt_map.get(&id) {
                if kind == EventKind::Press {
                    let opt = self.internal().options[*opt_idx].clone();
                    let btn = children.get_mut(self.internal().main_button_id);
                    btn.access(gui.clone())
                        .chain(Widget::first_child)
                        .chain(TextField::text)
                        .put(opt.name.clone());
                    btn.access(gui.clone())
                        .chain(ToggleButton::state)
                        .put(false);

                    self.internal().value = Some(opt.value.clone());

                    self.close(children, gui);
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
