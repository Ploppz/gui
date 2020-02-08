use crate::{
    lens::{LeafLens, Lens},
    *,
};
use indexmap::IndexMap;
use slog::Logger;
use std::ops::Deref;
use winput::Input;

mod config;
pub use config::WidgetConfig;

/// Macro is needed rather than a member function, in order to preserve borrow information:
/// so that the compiler knows that only `self.children` is borrowed.
macro_rules! children_proxy {
    ($self:ident) => {
        ChildrenProxy {
            self_id: $self.id,
            children: &mut $self.children,
        }
    };
}

// TODO: just pos, rel_pos, size initially

// TODO(PosLens): should be possible to read the value indeed but not set it
// To set a value one should go through `config`!
// Perhaps `get_mut` somehow has to do that? Idk how.
#[derive(Clone)]
pub struct PosLens;
impl Lens for PosLens {
    type Source = Widget;
    type Target = Vec2;
    fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target {
        &source.pos
    }
    fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target {
        &mut source.pos
    }
}
impl LeafLens for PosLens {
    fn target(&self) -> String {
        "Widget::pos".into()
    }
}

#[derive(Clone)]
pub struct SizeLens;
impl Lens for SizeLens {
    type Source = Widget;
    type Target = Vec2;
    fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target {
        &source.pos
    }
    fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target {
        &mut source.pos
    }
}
impl LeafLens for SizeLens {
    fn target(&self) -> String {
        "Widget::size".into()
    }
}

#[derive(Clone)]
pub struct FirstChildLens;
impl Lens for FirstChildLens {
    type Source = Widget;
    type Target = Widget;
    fn get<'a>(&self, w: &'a Widget) -> &'a Widget {
        &w.children().values().next().unwrap()
    }
    fn get_mut<'a>(&self, w: &'a mut Widget) -> &'a mut Widget {
        w.children_mut().next().unwrap()
    }
}

#[allow(non_upper_case_globals)]
impl Widget {
    pub const size: SizeLens = SizeLens;
    pub const pos: PosLens = PosLens;
    pub const first_child: FirstChildLens = FirstChildLens;
}

#[derive(Deref, DerefMut, Debug)]
pub struct Widget {
    #[deref_target]
    pub inner: Box<dyn Interactive>,
    /// Children of this node in the widget tree.
    pub(crate) children: IndexMap<Id, Widget>,
    /// Current absolute position as calculated by layout algorithm.
    /// Any mutation to `pos` has no effect except possibly generating spurious `ChangeSize` events.
    /// (should be read-only outside `gui`)
    pub pos: Vec2,
    /// Current relative (to parent) position as calculated by layout algorithm
    /// Any mutation to `rel_pos` has no effect except possibly generating spurious `ChangeSize` events.
    /// (should be read-only outside `gui`)
    pub rel_pos: Vec2,
    /// Current size as calculated by layout algorithm
    /// Any mutation to `size` has no effect except possibly generating spurious `ChangeSize` events.
    /// (should be read-only outside `gui`)
    pub size: Vec2,

    pub config: WidgetConfig,

    gui: GuiShared,

    /// Keeps track of hover state in order to generate the right WidgetEvents
    inside: bool,
    /// Keeps track of mouse press state in order to generate the right WidgetEvents
    pressed: bool,

    /// For internal use; mirrors the id that is the key in the HashMap that this Widget is
    /// likely a part of.
    /// NOTE: It's important to always ensure that `self.id` corresponds to the ID as registered in
    /// the gui system.
    id: Id,
}

impl Widget {
    pub(crate) fn new(id: Id, mut widget: Box<dyn Interactive>, gui: GuiShared) -> Widget {
        let mut children = IndexMap::new();
        let mut proxy = ChildrenProxy {
            self_id: id,
            children: &mut children,
        };
        let config = widget.init(&mut proxy, &gui);
        Widget {
            inner: widget,
            children,
            pos: Vec2::zero(),
            rel_pos: Vec2::zero(),
            size: Vec2::new(10.0, 10.0),
            config,
            gui,

            inside: false,
            pressed: false,
            id,
        }
    }
    /// Remove child for real - only for internal use.
    pub(crate) fn remove(&mut self, id: Id) -> Option<()> {
        self.children.remove(&id).map(drop)
    }

    /// Creates a lens to access this widget.
    pub fn access(&mut self, gui: GuiShared) -> InternalLens {
        InternalLens::new(self, gui)
    }
    pub fn children(&self) -> &IndexMap<Id, Widget> {
        &self.children
    }
    /// Get iterator over mutable children
    pub fn children_mut(&mut self) -> indexmap::map::ValuesMut<usize, Widget> {
        self.children.values_mut()
    }
    pub fn insert_child(&mut self, widget: Box<dyn Interactive>) -> Id {
        let gui = self.gui.clone();
        self.children_proxy().insert(widget, &gui)
    }
    pub fn remove_child(&mut self, id: Id) {
        let gui = self.gui.clone();
        self.children_proxy().remove(id, &gui)
    }
    /// Needed only when access to children are needed without access to the `Widget`: for example
    /// in `Interactive::update` and `Interactive::init`, which cannot possibly know the `Widget`
    pub fn children_proxy(&mut self) -> ChildrenProxy {
        children_proxy!(self)
    }
    pub fn get_id(&self) -> Id {
        self.id
    }
    pub fn hover(&self) -> bool {
        self.inside
    }
    pub fn pressed(&self) -> bool {
        self.pressed
    }
    /// Main update work happens here.
    /// Bottom-up means postfix
    /// NOTE: Due to recursion order, during update, position of `self` is not yet known.
    /// That's why calculating the absolute positions of widgets has to happen in a second pass.
    pub(crate) fn update_bottom_up(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: Vec2,
        gui: &GuiShared,
        log: Logger,
    ) -> Capture {
        let prev_events_len = gui.borrow().events().len();
        let mut capture = Capture::default();

        // Update children
        for child in self.children.values_mut() {
            let child_capture = child.update_bottom_up(input, sw, sh, mouse, gui, log.clone());
            capture |= child_capture;
        }

        if !capture.mouse {
            let mut gui = gui.borrow_mut();
            let now_inside = self.inside(self.pos, self.size, mouse);
            let prev_inside = self.inside;
            self.inside = now_inside;

            if now_inside && !prev_inside {
                gui.push_event(Event::new(self.id, EventKind::Hover));
            } else if prev_inside && !now_inside {
                gui.push_event(Event::new(self.id, EventKind::Unhover));
            }

            if now_inside {
                capture |= self.inner.captures();
            }

            if now_inside && input.is_mouse_button_toggled_down(winit::event::MouseButton::Left) {
                self.pressed = true;
                gui.push_event(Event::new(self.id, EventKind::Press));
            }
            if self.pressed && input.is_mouse_button_toggled_up(winit::event::MouseButton::Left) {
                self.pressed = false;
                gui.push_event(Event::new(self.id, EventKind::Release));
            }
        }
        // Execute widget-specific logic
        let local_events = gui.borrow().events()[prev_events_len..].to_vec();
        self.inner
            .update(self.id, local_events, &mut children_proxy!(self), gui);

        capture
    }
    /// Calculates absolute positions
    pub(crate) fn update_top_down(&mut self, events: &mut Vec<Event>) {
        let pos = self.pos;
        for child in self.children.values_mut() {
            let new_pos = pos + child.rel_pos;
            if new_pos != child.pos {
                events.push(Event::change(child.id, Widget::pos));
                child.pos = new_pos;
            }
            child.update_top_down(events);
        }
    }

    /// Recursively updates the position of children, and updates size of `self` if applicable.
    /// Additionally, updates sizes of text fields using `GuiDrawer`
    pub(crate) fn layout_alg<D: GuiDrawer>(
        &mut self,
        gui: GuiShared,
        drawer: &D,
        ctx: &mut D::Context,
    ) {
        for child in self.children.values_mut() {
            // Recurse
            child.layout_alg(gui.clone(), drawer, ctx);
        }

        // println!("Positioning Parent [{}]", self.id);
        if self.config.layout_wrap {
            unimplemented!()
        }
        let size = self.size;
        let layout_align = self.config.layout_align;
        let layout_main_margin = self.config.layout_main_margin;
        let padding_min = self.config.padding.min;

        let (main_axis, cross_axis) = (
            self.config.layout_direction,
            self.config.layout_direction.other(),
        );

        let mut layout_progress = self.config.padding.min[main_axis];
        // max width/height along cross axis
        let mut cross_size = 0.0;

        for child in self.children.values_mut() {
            let mut child_relative_pos = Vec2::zero();
            if let Some(place) = child.config.place {
                // Child does not participate in layout
                child_relative_pos.x = match place.x {
                    PlacementAxis::Fixed(x) => match place.x_anchor {
                        Anchor::Min => x,
                        Anchor::Center => (size.x - child.size.x) / 2.0 + x,
                        Anchor::Max => size.x - child.size.x - x,
                    },
                };
                child_relative_pos.y = match place.y {
                    PlacementAxis::Fixed(y) => match place.y_anchor {
                        Anchor::Min => y,
                        Anchor::Center => (size.y - child.size.y) / 2.0 + y,
                        Anchor::Max => size.y - child.size.y - y,
                    },
                };
            } else {
                // Layout algorithm
                child_relative_pos[main_axis] = layout_progress;
                layout_progress += child.size[main_axis] + layout_main_margin;
                child_relative_pos[cross_axis] = match layout_align {
                    Anchor::Min => padding_min[cross_axis],
                    Anchor::Center => (size[cross_axis] - child.size[cross_axis]) / 2.0,
                    Anchor::Max => unimplemented!(),
                };
                if child.size[cross_axis] > cross_size {
                    cross_size = child.size[cross_axis]
                }
            };

            // println!("Positioning Child [{}] relative_pos={:?}", child.id, child_relative_pos);
            child.rel_pos = child_relative_pos;
        }
        // because it should only be _between_ children - not after the last one
        layout_progress -= layout_main_margin;
        layout_progress += self.config.padding.max[main_axis];

        let mut new_size = self.size;
        // println!("[positioning {}] pre size {:?}", self.id, new_size);

        if let Some(intrinsic_size) = self.determine_size(&mut drawer.context_free(ctx)) {
            new_size = intrinsic_size;
        } else {
            match self.config.size_hint[main_axis] {
                SizeHint::Minimize => new_size[main_axis] = layout_progress,
                SizeHint::External(s) => new_size[main_axis] = s,
            }
            match self.config.size_hint[cross_axis] {
                SizeHint::Minimize => {
                    new_size[cross_axis] = cross_size
                        + self.config.padding.min[cross_axis]
                        + self.config.padding.max[cross_axis]
                }
                SizeHint::External(s) => new_size[cross_axis] = s,
            }
        }

        if new_size != self.size {
            self.size = new_size;
            gui.borrow_mut()
                .push_event(Event::change(self.id, Widget::size));
        }
    }

    pub fn recursive_children_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Widget> + 'a> {
        Box::new(
            self.children.values().chain(
                self.children
                    .values()
                    .map(|child| child.recursive_children_iter())
                    .flatten(),
            ),
        )
    }
}

/// Provides an interface to insert, delete and get immediate children.
/// Through Deref, we can get the immediate children immutably.
/// DerefMut is not implemented, because it is forbidden to insert children without using the
/// provided `ChildrenProxy::insert` function.
/// NOTE: If you need to get a widget in the widget tree that is not immediate, look to
/// [gui::WidgetLens] or the getters of [Gui]
///
pub struct ChildrenProxy<'a> {
    self_id: Id,
    /// children of a widget
    children: &'a mut IndexMap<Id, Widget>,
}
impl<'a> Deref for ChildrenProxy<'a> {
    type Target = IndexMap<Id, Widget>;
    fn deref(&self) -> &Self::Target {
        self.children
    }
}
impl<'a> ChildrenProxy<'a> {
    pub fn new(self_id: Id, children: &'a mut IndexMap<Id, Widget>) -> Self {
        Self { self_id, children }
    }
    pub fn insert(&mut self, widget: Box<dyn Interactive>, gui: &GuiShared) -> Id {
        let id = {
            let mut gui = gui.borrow_mut();
            let id = gui.new_id();

            // Emit event
            gui.push_event(Event::new(id, EventKind::New));
            // Update paths
            let path = if self.self_id == 1 {
                vec![]
            } else {
                let mut p = gui.get_path(self.self_id).to_vec();
                p.push(self.self_id);
                p
            };
            gui.insert_path(id, path);
            id
        };

        let widget = Widget::new(id, widget, gui.clone());
        self.children.insert(id, widget);
        id
    }
    pub fn remove(&mut self, id: Id, gui: &GuiShared) {
        gui.borrow_mut().remove(id);
    }
    pub fn get_mut(&mut self, id: Id) -> &mut Widget {
        self.children.get_mut(&id).unwrap()
    }
    pub fn values_mut(&mut self) -> indexmap::map::ValuesMut<usize, Widget> {
        self.children.values_mut()
    }
}
