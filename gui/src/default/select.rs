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
    pub fn option(mut self, name: String, value: String) -> Self {
        self.options.push(SelectOption {
            name,
            value: Some(value),
        });
        self
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

        let open = *ctx
            .get_child_mut(self.main_button_id)
            .access()
            .chain(ToggleButton::<Style::Button>::state)
            .get();
        let size = self.max_size(ctx);
        for Event { id, kind } in local_events.iter().cloned() {
            // Toggle dropdown list
            if id == self.main_button_id {
                if kind.is_change(ToggleButton::<Style::Button>::state) {
                    if open {
                        for (i, option) in self.options.iter().enumerate() {
                            let id = ctx.insert_child(ToggleButton::<Style::Button>::new());

                            ctx.get_child_mut(id)
                                .access()
                                .chain(Widget::first_child)
                                .chain(TextField::<Style::TextField>::text)
                                .put(option.name.clone());
                            ctx.access_child(id)
                                .chain(Widget::first_child)
                                .configure(|config| {
                                    config.set_size_hint(
                                        SizeHint::External(size.x),
                                        SizeHint::External(size.y),
                                    );
                                });

                            // Button::text.put(ctx.get_mut(id), option.name.clone());
                            self.opt_map.insert(id, i);
                        }
                    } else {
                        self.close(ctx);
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

                    if opt.value != self.value {
                        ctx.push_event(EventKind::change(Self::value));
                    }

                    self.value = opt.value.clone();

                    self.close(ctx);
                }
            }
        }
        // Toggle button with current option if applicable
        if open {
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
impl<Style> Select<Style> {
    pub const main_button: MainButtonLens<Style> = MainButtonLens {
        _marker: std::marker::PhantomData,
    };
}
