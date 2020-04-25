use super::*;
// use crate::{*, widget::lenses::ChildLens};
use crate::*;
use indexmap::IndexMap;

pub trait SelectStyle: Default + Send + Sync + Clone + std::fmt::Debug + 'static {
    type TextField: TextFieldStyle;
    type Button: ButtonStyle;
}

#[derive(Debug, Clone, PartialEq)]
struct SelectOption {
    pub name: String,
    pub value: Option<String>,
}

#[derive(LensInternal, Debug)]
pub struct Select<Style> {
    // configuration
    options: Vec<SelectOption>,
    pub style: Style,

    // runtime state
    #[lens]
    value: Option<String>,
    /// map from ID to option index. If the index is None, it means the button is the reset button.
    opt_map: IndexMap<Id, usize>,
    main_button_id: usize,
}
impl<Style: SelectStyle> Select<Style> {
    pub fn new() -> Select<Style> {
        Select {
            options: vec![SelectOption {
                name: "-".into(),
                value: None,
            }],
            value: None,
            opt_map: IndexMap::new(),
            main_button_id: 0,
            style: Style::default(),
        }
    }
    pub fn with_option(mut self, name: String, value: String) -> Self {
        self.options.push(SelectOption {
            name,
            value: Some(value),
        });
        self
    }
    pub fn open(&mut self, ctx: &mut WidgetContext) {
        let container_id = ctx.insert_child(Container::new());
        ctx.access_child(container_id).configure(|config| {
            config.set_placement(Placement::fixed(0.0, 0.0));
            config.set_layout(Axis::Y, false, Anchor::Min, 2.0);
        });
        let size = self.max_size(ctx);
        for (i, option) in self.options.iter().enumerate() {
            let id = ctx
                .get_child_mut(container_id)
                .insert_child(ToggleButton::<Style::Button>::new());

            ctx.access_child(container_id)
                .chain(Widget::child(id))
                .chain(ToggleButton::<Style::Button>::text_field)
                .chain(TextField::<Style::TextField>::text)
                .put(option.name.clone());
            ctx.access_child(container_id)
                .chain(Widget::child(id))
                .chain(ToggleButton::<Style::Button>::text_field)
                .configure(|config| {
                    config.set_size_hint(SizeHint::External(size.x), SizeHint::External(size.y));
                });

            if self.value == option.value {
                ctx.access_child(container_id)
                    .chain(Widget::child(id))
                    .chain(ToggleButton::<Style::Button>::state)
                    .put(true);
            }

            self.opt_map.insert(id, i);
        }
    }
    pub fn close(&mut self, ctx: &mut WidgetContext) {
        let to_remove = ctx.keys().cloned().collect::<Vec<_>>();
        for id in to_remove {
            if id != self.main_button_id {
                ctx.remove_child(id);
            }
        }
        self.opt_map = IndexMap::new();
    }
    pub fn main_button_id(&self) -> Id {
        self.main_button_id
    }
    fn max_size(&self, ctx: &mut WidgetContext) -> Vec2 {
        self.options
            .iter()
            .fold(None, |max: Option<Vec2>, option| {
                let size = ctx.gui.borrow_mut().text_calc.text_size(&option.name);
                if let Some(max) = max {
                    Some(Vec2::new(max.x.max(size.x), max.y.max(size.y)))
                } else {
                    Some(size)
                }
            })
            .unwrap()
    }
    // #[cfg(test)]
    pub fn get_widget_for_option(&self, value: &str) -> Option<Id> {
        let target_opt_idx = self
            .options
            .iter()
            .enumerate()
            .find(|o| o.1.value == self.value)
            .unwrap()
            .0;
        self.opt_map
            .iter()
            .find(|(id, opt_idx)| **opt_idx == target_opt_idx)
            .map(|(id, _)| *id)
    }
}

impl<Style: SelectStyle> Interactive for Select<Style> {
    fn init(&mut self, ctx: &mut WidgetContext) -> WidgetConfig {
        let main_id = ctx.insert_child(ToggleButton::<Style::Button>::new());
        self.main_button_id = main_id;
        let size = self.max_size(ctx);
        ctx.access_child(main_id)
            .chain(Button::<Style::Button>::text_field)
            .configure(|config| {
                config.set_size_hint(SizeHint::External(size.x), SizeHint::External(size.y));
                config.set_padding(2.0, 2.0, 2.0, 2.0);
            });

        WidgetConfig::default()
            // .padding(4.0, 4.0, 6.0, 6.0)
            .layout(Axis::Y, false, Anchor::Min, 0.0)
    }
    fn update(&mut self, _id: Id, local_events: Vec<Event>, ctx: &mut WidgetContext) {
        // Always ensure that all children have the same width
        let is_open = !*ctx
            .get_child_mut(self.main_button_id)
            .access()
            .chain(ToggleButton::<Style::Button>::state)
            .get();
        for Event { id, kind } in local_events.iter().cloned() {
            // Toggle dropdown list
            if id == self.main_button_id {
                if kind.is_change(ToggleButton::<Style::Button>::state) {
                    if is_open {
                        self.close(ctx);
                    } else {
                        self.open(ctx);
                    }
                }
            }

            // Handle any option buttons
            if let Some(opt_idx) = self.opt_map.get(&id) {
                if kind == EventKind::Press {
                    let opt = self.options[*opt_idx].clone();
                    let btn = ctx.get_child_mut(self.main_button_id);
                    btn.access()
                        .chain(Widget::first_child)
                        .chain(TextField::<Style::TextField>::text)
                        .put(opt.name.clone());
                    btn.access()
                        .chain(ToggleButton::<Style::Button>::state)
                        .put(false);

                    // TODO: Also somehow automatically emit events on change of fields?
                    // Somehow force the use of lenses?
                    if opt.value != self.value {
                        ctx.push_event(EventKind::change(Self::value));
                    }
                    self.value = opt.value.clone();

                    self.close(ctx);
                }
            }
        }
        // Toggle button with current option if applicable
        if is_open {
            for (id, opt_idx) in self.opt_map.iter() {
                let option_value = &self.options[*opt_idx].value;
                ctx.get_child_mut(*id)
                    .access()
                    .chain(ToggleButton::<Style::Button>::state)
                    .put(*option_value == self.value);
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

// --------
// Lenses
// --------

#[derive(Clone)]
pub struct MainButtonLens<Style> {
    _marker: std::marker::PhantomData<Style>,
}
impl<Style: SelectStyle> Lens for MainButtonLens<Style> {
    type Source = Widget;
    type Target = Widget;
    fn get<'a>(&self, w: &'a Widget) -> &'a Widget {
        let id = w.downcast_ref::<Select<Style>>().unwrap().main_button_id();
        &w.children()[&id]
    }
    fn get_mut<'a>(&self, w: &'a mut Widget) -> &'a mut Widget {
        let id = w.downcast_mut::<Select<Style>>().unwrap().main_button_id();
        w.get_child_mut(id)
    }
}
#[derive(Clone)]
pub struct OptionLens<Style> {
    value: Option<String>,
    _marker: std::marker::PhantomData<Style>,
}
impl<Style> Select<Style> {
    pub const main_button: MainButtonLens<Style> = MainButtonLens {
        _marker: std::marker::PhantomData,
    };
}
