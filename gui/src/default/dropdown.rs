use super::*;
use crate::*;
use indexmap::IndexMap;

pub trait SelectStyle: Default + Send + Sync + Clone + std::fmt::Debug + 'static {
    type TextField: TextFieldStyle;
    type Button: ButtonStyle;
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropdownOption {
    pub name: String,
    pub value: String,
}

#[derive(LensInternal, Debug)]
pub struct Select<Style> {
    options: Vec<DropdownOption>,
    value: Option<String>,
    /// map from ID to option index
    opt_map: IndexMap<Id, usize>,
    main_button_id: usize,
    pub style: Style,
}
impl<Style: SelectStyle> Select<Style> {
    pub fn new() -> Select<Style> {
        Select {
            options: Vec::new(),
            value: None,
            opt_map: IndexMap::new(),
            main_button_id: 0,
            style: Style::default(),
        }
    }
    pub fn option(mut self, name: String, value: String) -> Self {
        self.options.push(DropdownOption { name, value });
        self
    }
    pub fn close(&mut self, children: &mut ChildrenProxy, gui: &GuiShared) {
        let to_remove = children.keys().cloned().collect::<Vec<_>>();
        for id in to_remove {
            if id != self.main_button_id {
                children.remove(id, gui);
            }
        }
        self.opt_map = IndexMap::new();
    }
}

impl<Style: SelectStyle> Interactive for Select<Style> {
    fn init(&mut self, children: &mut ChildrenProxy, gui: &GuiShared) -> WidgetConfig {
        let main_id = children.insert(
            Box::new(ToggleButton::<Style::Button>::new()) as Box<dyn Interactive>,
            gui,
        );
        children.get_mut(main_id).config.set_height(24.0);
        self.main_button_id = main_id;
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
            if id == self.main_button_id {
                if kind.is_change(ToggleButton::<Style::Button>::state) {
                    let toggled = *children
                        .get_mut(id)
                        .access(gui.clone())
                        .chain(ToggleButton::<Style::Button>::state)
                        .get();
                    if toggled {
                        for (i, option) in self.options.iter().enumerate() {
                            let id =
                                children.insert(Box::new(Button::<Style::Button>::new()), &gui);

                            children
                                .get_mut(id)
                                .access(gui.clone())
                                .chain(Widget::first_child)
                                .chain(TextField::<Style::TextField>::text)
                                .put(option.name.clone());

                            // Button::text.put(children.get_mut(id), option.name.clone());
                            self.opt_map.insert(id, i);
                        }
                    } else {
                        self.close(children, gui);
                    }
                }
            }

            if let Some(opt_idx) = self.opt_map.get(&id) {
                if kind == EventKind::Press {
                    let opt = self.options[*opt_idx].clone();
                    let btn = children.get_mut(self.main_button_id);
                    btn.access(gui.clone())
                        .chain(Widget::first_child)
                        .chain(TextField::<Style::TextField>::text)
                        .put(opt.name.clone());
                    btn.access(gui.clone())
                        .chain(ToggleButton::<Style::Button>::state)
                        .put(false);

                    self.value = Some(opt.value.clone());

                    self.close(children, gui);
                }
            }
        }
    }

    /*
    fn determine_size(&self, drawer: &mut dyn ContextFreeGuiDrawer) -> Option<Vec2> {
        let mut max_x = None;
        let mut max_y = None;
        for DropdownOption { name, value: _ } in &self.options {
            let (x, y) = drawer.text_size(&name);
            max_x = max_x.or(Some(x)).map(|max_x| max_x.max(x));
            max_y = max_y.or(Some(y)).map(|max_y| max_y.max(y));
        }
        max_y.and_then(|max_y| max_x.and_then(|max_x| Some((max_x, max_y))))
    }
    */

    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}
