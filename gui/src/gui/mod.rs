use crate::*;
use indexmap::IndexMap;
use slog::Logger;
use std::{cell::RefCell, rc::Rc};

mod drawer;
pub use drawer::*;

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
    pub paths: IndexMap<Id, Vec<Id>>,
    id_cnt: usize,
    to_remove: Vec<Id>,
    /// Events collected outside update function, consumed when update is called
    events: Vec<Event>,
}
impl GuiInternal {
    pub fn new() -> Self {
        GuiInternal {
            paths: IndexMap::new(),
            id_cnt: ROOT,
            to_remove: Vec::new(),
            events: Vec::new(),
        }
    }
    pub fn new_id(&mut self) -> Id {
        self.id_cnt += 1;
        self.id_cnt
    }
    pub fn remove(&mut self, id: Id) {
        self.to_remove.push(id);
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

#[derive(Debug)]
pub struct Gui<D> {
    pub root: Widget,
    screen: (f32, f32),
    drawer: D,
    pub aliases: IndexMap<String, Id>,
    pub internal: Rc<RefCell<GuiInternal>>,
}

impl<D: GuiDrawer> Gui<D> {
    pub fn new(drawer: D) -> Gui<D> {
        let internal = Rc::new(RefCell::new(GuiInternal::new()));
        let mut root = Widget::new(ROOT, Box::new(Container::new()), internal.clone());
        root.config = root.config.placement(Placement::fixed(0.0, 0.0));
        Gui {
            root,
            drawer,
            screen: (0.0, 0.0),
            internal,
            aliases: IndexMap::new(),
        }
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
        let mouse = self.drawer.transform_mouse(input.get_mouse_position(), ctx);
        let (sw, sh) = self.drawer.window_size(ctx);
        self.root.config.set_size(sw, sh);

        // Delete widgets that were marked for deletion last frame
        let to_remove = std::mem::replace(&mut self.internal.borrow_mut().to_remove, Vec::new());
        for id_to_remove in to_remove {
            let parent_id = self.parent(id_to_remove);
            let parent = self.get_mut(parent_id);
            parent.remove(id_to_remove);
        }

        let mut events = std::mem::replace(&mut self.internal.borrow_mut().events, Vec::new());

        // 3 traversals
        let capture = self
            .root
            .update_bottom_up(input, sw, sh, mouse, &mut events, log.clone());
        self.root.layout_alg(&mut events, &self.drawer, ctx);
        self.root.update_top_down(&mut events);

        // Update parent relations
        let mut old_paths =
            std::mem::replace(&mut self.internal.borrow_mut().paths, IndexMap::new());
        update_paths_recurse(
            vec![],
            &mut self.root,
            &mut old_paths,
            &mut self.internal.borrow_mut().paths,
            &mut events,
        );
        // TODO remove old_paths - not needed (?) (removal is now marked through GuiInternal

        // Signal removal to renderer
        for to_remove_id in &self.internal.borrow().to_remove {
            for descendant in self.get(*to_remove_id).recursive_children_iter() {
                events.push(Event::new(descendant.get_id(), EventKind::Removed));
            }
            events.push(Event::new(*to_remove_id, EventKind::Removed));
        }

        let ops = self.drawer.update(self, &events, log, ctx);
        for op in ops {
            match op {
                WidgetOp::Resize { id, size } => {
                    self.get_mut(id).config.set_size(size.0, size.1);
                    events.push(Event::change(id, Widget::size));
                }
            }
        }

        (events, capture)
    }
    pub fn insert<I: AsId<D>, W: Interactive>(&mut self, parent_id: I, widget: W) -> Option<Id> {
        self.insert_internal(parent_id, Box::new(widget))
    }
    /// Returns None if widget referred to by parent_id does not exist
    fn insert_internal<I: AsId<D>>(
        &mut self,
        parent_id: I,
        widget: Box<dyn Interactive>,
    ) -> Option<Id> {
        if let Some(parent_id) = parent_id.resolve(self) {
            let internal = self.internal.clone();
            // Create Widget and insert
            if let Some(parent) = self.try_get_mut(parent_id) {
                let mut children = ChildrenProxy::new(parent_id, &mut parent.children, internal);
                Some(children.insert(widget))
            // TODO ???
            // self.events.push(Event::new(id, EventKind::Change));
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
fn update_paths_recurse(
    current_path: Vec<Id>,
    w: &mut Widget,
    old_paths: &mut IndexMap<Id, Vec<Id>>,
    paths: &mut IndexMap<Id, Vec<Id>>,
    events: &mut Vec<Event>,
) {
    for child in w.children.values_mut() {
        if !old_paths.contains_key(&child.get_id()) {
            // If not known, issue an event
            events.push(Event::new(child.get_id(), EventKind::New));
            println!(
                "[gui] Found new widget {:?} - Change event sent",
                child.get_id()
            );
        }
        old_paths.remove(&child.get_id());
        paths.insert(child.get_id(), current_path.clone());

        let mut child_path = current_path.clone();
        child_path.push(child.get_id());
        update_paths_recurse(child_path, child, old_paths, paths, events);
    }
}