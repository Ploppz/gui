use crate::*;
use indexmap::IndexMap;
use slog::Logger;
use std::{cell::RefCell, rc::Rc};

mod drawer;
pub use drawer::*;

/// Holds a shared reference to the internal struct of `Gui`,
/// for operations such as emitting events, adding and removing widgets, etc.
///
/// `clone()` liberally, as it amounts to `Rc::clone`.
///
/// NOTE: By virtue of holding an `Rc<RefCell<_>>`, any operations will `borrow()` or `borrow_mut()`.

pub type GuiShared = Rc<RefCell<GuiInternal>>;

pub const ROOT: usize = 1;

pub trait AsId<D: GuiDrawer>: Clone + std::fmt::Display {
    fn resolve(&self, gui: &Gui<D>) -> Option<Id>;
}
impl<D: GuiDrawer> AsId<D> for Id {
    fn resolve(&self, _gui: &Gui<D>) -> Option<Id> {
        Some(*self)
    }
}
impl<D: GuiDrawer> AsId<D> for String {
    fn resolve(&self, gui: &Gui<D>) -> Option<Id> {
        gui.aliases.get(self).map(|x| *x)
    }
}
impl<D: GuiDrawer> AsId<D> for &String {
    fn resolve(&self, gui: &Gui<D>) -> Option<Id> {
        gui.aliases.get(*self).map(|x| *x)
    }
}
impl<D: GuiDrawer> AsId<D> for &str {
    fn resolve(&self, gui: &Gui<D>) -> Option<Id> {
        gui.aliases.get(*self).map(|x| *x)
    }
}

/// Access to `Gui`'s internals
#[derive(Debug)]
pub struct GuiInternal {
    paths: IndexMap<Id, Vec<Id>>,
    id_cnt: usize,
    to_remove: Vec<Id>,
    /// Events collected outside update function, consumed when update is called.
    events: Vec<Event>,
    pub text_calc: Box<dyn TextCalculator>,
}
impl GuiInternal {
    pub fn new<T: TextCalculator>(text_calc: T) -> Self {
        GuiInternal {
            paths: IndexMap::new(),
            id_cnt: ROOT,
            to_remove: Vec::new(),
            events: Vec::new(),
            text_calc: Box::new(text_calc),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    pub fn remove(&mut self, id: Id) {
        self.to_remove.push(id);
    }
    pub(crate) fn new_id(&mut self) -> Id {
        self.id_cnt += 1;
        self.id_cnt
    }
    pub(crate) fn insert_path(&mut self, id: Id, path: Vec<Id>) {
        self.paths.insert(id, path);
    }
    pub(crate) fn get_path(&self, id: Id) -> &[Id] {
        &self.paths[&id]
    }
}

#[derive(Debug)]
pub struct Gui<D> {
    pub root: Widget,
    screen: (f32, f32),
    // Why option: need to take it out of Gui when we call GuiDrawer::update
    drawer: Option<D>,
    pub aliases: IndexMap<String, Id>,
    pub internal: Rc<RefCell<GuiInternal>>,
}

impl<D: GuiDrawer> Gui<D> {
    pub fn new(drawer: D, ctx: &mut D::Context) -> Gui<D> {
        let internal = Rc::new(RefCell::new(GuiInternal::new(drawer.text_calc(ctx))));
        let mut root = Widget::new(ROOT, Root, internal.clone());
        root.config = root.config.placement(Placement::fixed(0.0, 0.0));
        Gui {
            root,
            drawer: Some(drawer),
            screen: (0.0, 0.0),
            internal,
            aliases: IndexMap::new(),
        }
    }
    pub fn shared(&self) -> GuiShared {
        self.internal.clone()
    }
    pub fn shared_ref(&self) -> &GuiShared {
        &self.internal
    }
    /// Constructs a [`LensDriver`] to access a widget given by `id`
    pub fn access<I: AsId<D>>(&mut self, id: I) -> LensRoot {
        let internal = self.internal.clone();
        LensRoot::new(self.get_mut(id), internal)
    }
    pub fn insert<I: AsId<D>, W: Interactive>(&mut self, parent_id: I, widget: W) -> Option<Id> {
        self.insert_internal(parent_id, widget)
    }
    /// Returns None if widget referred to by parent_id does not exist
    fn insert_internal<I: AsId<D>, W: Interactive>(
        &mut self,
        parent_id: I,
        widget: W,
    ) -> Option<Id> {
        if let Some(parent_id) = parent_id.resolve(self) {
            if let Some(parent) = self.try_get_mut(parent_id) {
                Some(parent.insert_child(widget))
            } else {
                return None;
            }
        } else {
            None
        }
    }
    pub fn insert_in_root<W: Interactive>(&mut self, widget: W) -> Id {
        self.insert(ROOT, widget).unwrap()
    }
    pub fn insert_in_root_with_alias<W: Interactive>(&mut self, widget: W, alias: String) {
        let id = self.insert(ROOT, widget).unwrap();
        self.aliases.insert(alias, id);
    }
    /// Panics if widget does not exist, or (only if I = String) if alias does not exist
    pub fn get<I: AsId<D>>(&self, id: I) -> &Widget {
        self.try_get(id).unwrap()
    }
    pub fn try_get<I: AsId<D>>(&self, id: I) -> Option<&Widget> {
        if let Some(id) = id.resolve(self) {
            if id == ROOT {
                return Some(&self.root);
            }
            if let Some(path) = self.internal.borrow().paths.get(&id) {
                let mut current = &self.root;
                for id in path {
                    if let Some(child) = current.children.get(id) {
                        current = child;
                    } else {
                        panic!("Incorrect path (gui programming error?)");
                    }
                }
                if let Some(child) = current.children.get(&id) {
                    Some(child)
                } else {
                    panic!(
                        "Incorrect path (gui programming error?): reached destination but no child"
                    );
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn parent(&self, id: Id) -> Id {
        self.try_parent(id).unwrap()
    }
    pub fn try_parent(&self, id: Id) -> Option<Id> {
        self.internal
            .borrow()
            .paths
            .get(&id)
            .and_then(|path| path.last().map(|x| *x))
    }

    pub fn get_mut<I: AsId<D>>(&mut self, id: I) -> &mut Widget {
        self.try_get_mut(id).unwrap()
    }

    pub fn try_get_mut<I: AsId<D>>(&mut self, id: I) -> Option<&mut Widget> {
        if let Some(id) = id.resolve(self) {
            if id == ROOT {
                return Some(&mut self.root);
            }
            if let Some(path) = self.internal.borrow().paths.get(&id) {
                let mut current = &mut self.root;
                let current_id = current.get_id();
                for id in path {
                    if let Some(child) = current.children.get_mut(id) {
                        current = child;
                    } else {
                        panic!(
                            "Incorrect path (gui programming error?).
                            {} not a child of {} on path {:?}",
                            id, current_id, path
                        );
                    }
                }
                if let Some(child) = current.children.get_mut(&id) {
                    Some(child)
                } else {
                    panic!(
                        "Incorrect path (gui programming error?): reached destination but no child"
                    );
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn id_eq<I: AsId<D>, J: AsId<D>>(&self, i: I, j: J) -> bool {
        i.resolve(self) == j.resolve(self)
    }

    /// Recursively process all widgets (mutably) in the tree
    // TODO immutable version
    pub fn widgets_mut(&mut self, f: &mut dyn FnMut(&mut Widget)) {
        recursive_children_mut(&mut self.root, f)
    }

    /// # Removal of widgets
    /// Removal of widgets is done through the `GuiInternal` struct given in the update function
    /// of widgets. This only maintains a list of `Id`s of widget to be deleted. At the very start of
    /// `Gui::update`, these widgets are deleted. Thus, for all `Id`s found in the events returned by
    /// `Gui::update`, it is guaranteed that the widget exists - until the next call to
    /// `Gui::update`.
    pub fn update(
        &mut self,
        input: &Input,
        log: Logger,
        ctx: &mut D::Context,
    ) -> (Vec<Event>, Capture) {
        let mouse = self
            .drawer
            .as_mut()
            .unwrap()
            .transform_mouse(input.get_mouse_position().into(), ctx);
        let Vec2 { x: sw, y: sh } = self.drawer.as_mut().unwrap().window_size(ctx);
        self.root.config.set_size(sw, sh);

        // Delete widgets that were marked for deletion last frame
        {
            let to_remove =
                std::mem::replace(&mut self.internal.borrow_mut().to_remove, Vec::new());
            for id_to_remove in to_remove {
                let parent_id = self.parent(id_to_remove);
                let parent = self.get_mut(parent_id);
                parent.remove(id_to_remove);
            }
        }

        // 3 traversals
        let capture = self
            .root
            .update_bottom_up(input, sw, sh, mouse, log.clone());
        self.root.layout_alg();
        self.root.update_top_down();

        // Update parent relations
        {
            let mut internal = self.internal.borrow_mut();
            internal.paths = IndexMap::new();
            update_paths_recurse(vec![], &mut self.root, &mut internal.paths);
        }

        // Emit Remove events (without removing widgets)
        {
            // (move `to_remove` out and back in again)
            let to_remove =
                std::mem::replace(&mut self.internal.borrow_mut().to_remove, Vec::new());
            to_remove
                .iter()
                .flat_map(|to_remove_id| {
                    self.get(*to_remove_id)
                        .recursive_children_iter()
                        .map(|descendant| descendant.get_id())
                        .chain(std::iter::once(*to_remove_id))
                })
                .for_each(|id| {
                    self.internal
                        .borrow_mut()
                        .push_event(Event::new(id, EventKind::Removed))
                });
            // TODO ^ will probably panic
            std::mem::replace(&mut self.internal.borrow_mut().to_remove, to_remove);
        }

        let events = std::mem::replace(&mut self.internal.borrow_mut().events, Vec::new());

        let drawer = self.drawer.take().unwrap();
        let ops = drawer.update(self, &events, log, ctx);
        self.drawer = Some(drawer);
        for op in ops {
            match op {
                WidgetOp::Resize { id, size } => {
                    self.get_mut(id).config.set_size(size.x, size.y);
                    self.internal
                        .borrow_mut()
                        .push_event(Event::change(id, Widget::size));
                }
            }
        }

        (events, capture)
    }
}

// NOTE: can't be in `Interactive` because of F
fn recursive_children_mut(w: &mut Widget, f: &mut dyn FnMut(&mut Widget)) {
    for child in w.children.values_mut() {
        f(child);
    }
    for child in w.children.values_mut() {
        recursive_children_mut(child, f);
    }
}
fn update_paths_recurse(current_path: Vec<Id>, w: &mut Widget, paths: &mut IndexMap<Id, Vec<Id>>) {
    for child in w.children.values_mut() {
        paths.insert(child.get_id(), current_path.clone());
        let mut child_path = current_path.clone();
        child_path.push(child.get_id());
        update_paths_recurse(child_path, child, paths);
    }
}

#[derive(Debug, Default)]
pub struct Root;
impl Interactive for Root {
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}
