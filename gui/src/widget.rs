use crate::{
    lens::{LeafLens, Lens},
    *,
};
use indexmap::IndexMap;
use slog::Logger;
use std::ops::Deref;
use winput::Input;

mod layout;
pub mod lenses;
pub use layout::WidgetConfig;

/// Macro is needed rather than a member function, in order to preserve borrow information:
/// so that the compiler knows that only `self.children` is borrowed.
macro_rules! widget_context {
    ($self:ident) => {
        WidgetContext {
            self_id: $self.id,
            children: &mut $self.children,
            gui: &$self.gui,
        }
    };
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
    pub(crate) fn new<I: Interactive>(id: Id, mut widget: I, gui: GuiShared) -> Widget {
        let mut children = IndexMap::new();
        let mut ctx = WidgetContext {
            self_id: id,
            children: &mut children,
            gui: &gui,
        };
        let config = widget.init(&mut ctx);
        Widget {
            inner: Box::new(widget),
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
    pub fn access(&mut self) -> LensRoot {
        LensRoot::new(self, self.gui.clone())
    }
    pub fn children(&self) -> &IndexMap<Id, Widget> {
        &self.children
    }
    pub fn insert_child<I: Interactive>(&mut self, child: I) -> Id {
        self.widget_context().insert_child(child)
    }
    pub fn get_child_mut(&mut self, id: Id) -> &mut Widget {
        self.children.get_mut(&id).unwrap()
    }
    /// Get iterator over widgets and their ids
    pub fn children_mut(&mut self) -> indexmap::map::ValuesMut<usize, Widget> {
        self.children.values_mut()
    }
    pub fn remove_child(&mut self, id: Id) {
        self.gui.borrow_mut().remove(id);
    }
    /// Needed only when access to children are needed without access to the `Widget`: for example
    /// in `Interactive::update` and `Interactive::init`, which cannot possibly know the `Widget`
    pub(crate) fn widget_context(&mut self) -> WidgetContext {
        widget_context!(self)
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
        log: Logger,
    ) -> Capture {
        let prev_events_len = self.gui.borrow().events().len();
        let mut capture = Capture::default();

        // Update children
        for child in self.children.values_mut() {
            let child_capture = child.update_bottom_up(input, sw, sh, mouse, log.clone());
            capture |= child_capture;
        }

        if !capture.mouse {
            let mut gui = self.gui.borrow_mut();
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
        let local_events = self.gui.borrow().events()[prev_events_len..].to_vec();
        self.inner
            .update(self.id, local_events, &mut widget_context!(self));

        capture
    }
    /// Calculates absolute positions
    pub(crate) fn update_top_down(&mut self) {
        let pos = self.pos;
        for child in self.children.values_mut() {
            let new_pos = pos + child.rel_pos;
            if new_pos != child.pos {
                self.gui
                    .borrow_mut()
                    .push_event(Event::change(child.id, Widget::pos));
                child.pos = new_pos;
            }
            child.update_top_down();
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
/// provided `WidgetContext::insert` function.
/// NOTE: If you need to get a widget in the widget tree that is not immediate, look to
/// [gui::LensRoot], [gui::Gui::access] or [gui::Widget::access]
///
pub struct WidgetContext<'a, 'b> {
    /// children of a widget
    children: &'a mut IndexMap<Id, Widget>,
    pub self_id: Id,
    pub gui: &'b GuiShared,
}
impl<'a, 'b> Deref for WidgetContext<'a, 'b> {
    type Target = IndexMap<Id, Widget>;
    fn deref(&self) -> &Self::Target {
        self.children
    }
}
impl<'a, 'b> WidgetContext<'a, 'b> {
    /// Insert child
    pub fn insert_child<I: Interactive>(&mut self, child: I) -> Id {
        let id = {
            let mut gui = self.gui.borrow_mut();
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

        let widget = Widget::new(id, child, self.gui.clone());
        self.children.insert(id, widget);
        id
    }
    /// Remove child
    pub fn remove_child(&mut self, id: Id) {
        self.gui.borrow_mut().remove(id);
    }
    /// Get child
    pub fn get_child_mut(&mut self, id: Id) -> &mut Widget {
        self.children.get_mut(&id).unwrap()
    }
    /// Get iterator over widgets and their ids
    pub fn children_mut(&mut self) -> indexmap::map::ValuesMut<usize, Widget> {
        self.children.values_mut()
    }
    pub fn access_child(&mut self, id: Id) -> LensRoot {
        LensRoot::new(&mut self.children[&id], self.gui.clone())
    }
}
