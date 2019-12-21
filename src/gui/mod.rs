use crate::lens::Lens;
use crate::*;
use indexmap::IndexMap;
use slog::{info, Logger};
mod drawer;
use std::{cell::RefCell, rc::Rc};

pub use drawer::*;

pub const ROOT: usize = 1;

pub trait AsId<D: GuiDrawer>: Clone {
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

/// Just some fields needed to create widgets..
#[derive(Debug, Clone)]
pub struct ChildService {
    pub paths: IndexMap<Id, Vec<Id>>,
    id_cnt: usize,
    to_remove: Vec<Id>,
}
impl ChildService {
    pub fn new_id(&mut self) -> Id {
        self.id_cnt += 1;
        self.id_cnt
    }
    pub fn remove(&mut self, id: Id) {
        self.to_remove.push(id);
    }
}

pub struct OperationService {
    operations: Vec<(Id, Box<dyn FnOnce(&mut Widget)>)>,
}
impl OperationService {
    pub fn push<O: FnOnce(&mut Widget) + 'static>(&mut self, id: Id, op: O) {
        self.operations.push(Box::new(op));
    }
}

#[derive(Debug)]
pub struct Gui<D: GuiDrawer> {
    pub root: Widget,
    screen: (f32, f32),
    drawer: D,
    pub aliases: IndexMap<String, Id>,
    pub child_service: Rc<RefCell<ChildService>>,
    /// Allows operations on widgets to be added from anywhere (in the same thread)
    pub op_service: Rc<RefCell<OperationService>>,
    /// Events collected outside update function, consumed when update is called
    events: Vec<(Id, WidgetEvent)>,
}

/// Provides access to a widget
pub struct WidgetProxy {
    op_service: Rc<RefCell<OperationService>>,
}
// WidgetLens resolves Gui -> WidgetProxy.
// From there, you should chain it with e.g. `Button::text`, which goes WidgetProxy

pub struct WidgetLens<D, I> {
    id: I,
    _phantom: std::marker::PhantomData<D>,
}
impl<D: GuiDrawer, I: AsId<D>> WidgetLens<D, I> {
    pub fn get(id: I) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<D: GuiDrawer, I: AsId<D>> Lens<Gui<D>, Widget> for WidgetLens<D, I> {
    fn with<V, F: FnOnce(&Widget) -> V>(&self, gui: &Gui<D>, f: F) -> V {
        let w = gui.get(self.id.clone());
        f(w)
    }
    fn with_mut<V, F: FnOnce(&mut Widget) -> V>(&self, gui: &mut Gui<D>, f: F) -> V {
        let w = gui.get_mut(self.id.clone());
        f(w)
    }
}

impl<D: GuiDrawer> Gui<D> {
    pub fn new(drawer: D) -> Gui<D> {
        let child_service = Rc::new(RefCell::new(ChildService {
            paths: IndexMap::new(),
            id_cnt: ROOT,
            to_remove: Vec::new(),
        }));
        let mut root = Widget::new(ROOT, Box::new(Container::new()), child_service.clone());
        root.config = root.config.placement(Placement::fixed(0.0, 0.0));
        Gui {
            root,
            drawer,
            screen: (0.0, 0.0),
            child_service: child_service,
            events: Vec::new(),
            aliases: IndexMap::new(),
        }
    }

    /// # Removal of widgets
    /// Removal of widgets is done through the `ChildService` struct given in the update function
    /// of widgets. This only maintains a list of `Id`s of widget to be deleted. At the very start of
    /// `Gui::update`, these widgets are deleted. Thus, for all `Id`s found in the events returned by
    /// `Gui::update`, it is guaranteed that the widget exists - until the next call to
    /// `Gui::update`.
    pub fn update(
        &mut self,
        input: &Input,
        log: Logger,
        ctx: &mut D::Context,
    ) -> (Vec<(Id, WidgetEvent)>, Capture) {
        let mouse = self.drawer.transform_mouse(input.get_mouse_position(), ctx);
        let (sw, sh) = self.drawer.window_size(ctx);
        self.root.config.set_size(sw, sh);

        // Delete widgets that were marked for deletion last frame
        let to_remove =
            std::mem::replace(&mut self.child_service.borrow_mut().to_remove, Vec::new());
        for id_to_remove in to_remove {
            let parent_id = self.parent(id_to_remove);
            let parent = self.get_mut(parent_id);
            parent.remove(id_to_remove);
        }

        // Update sizes of text fields that have changed
        // - temporary solution - silly to require a while traversal only for that (TODO)
        {
            let mut changed_texts = Vec::new();
            for child in self.root.recursive_children_iter() {
                if child.changed {
                    if child.inner.is::<TextField>() {
                        let text = child
                            .inner
                            .downcast_ref::<TextField>()
                            .unwrap()
                            .text
                            .clone();
                        changed_texts.push((child.get_id(), text));
                    }
                }
            }
            for (id, text) in changed_texts {
                let text_size = self.drawer.text_size(&text, ctx);
                info!(log, "EARLY TEXT UPDATE ({}) -> {:?}", text, text_size);
                WidgetLens::get(id).with_mut(self, |w| w.config.set_size(text_size.0, text_size.1))
            }
        }

        // Update logic recursively
        let (mut events, capture) = self.root.update(input, sw, sh, mouse, log.clone());
        events.extend(std::mem::replace(&mut self.events, Vec::new()));

        // Update parent relations
        let mut old_paths =
            std::mem::replace(&mut self.child_service.borrow_mut().paths, IndexMap::new());
        update_paths_recurse(
            vec![],
            &mut self.root,
            &mut old_paths,
            &mut self.child_service.borrow_mut().paths,
            &mut events,
        );
        // TODO remove old_paths - not needed (?) (removal is now marked through ChildService

        // Signal removal to renderer
        for to_remove_id in &self.child_service.borrow().to_remove {
            for descendant in self.get(*to_remove_id).recursive_children_iter() {
                events.push((descendant.get_id(), WidgetEvent::Removed));
            }
            events.push((*to_remove_id, WidgetEvent::Removed));
        }

        let ops = self.drawer.update(self, &events, log, ctx);
        for op in ops {
            match op {
                WidgetOp::Resize { id, size } => {
                    self.get_mut(id).config.set_size(size.0, size.1);
                    events.push((id, WidgetEvent::ChangeSize));
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
            let child_service = self.child_service.clone();
            // Create Widget and insert
            if let Some(parent) = self.try_get_mut(parent_id) {
                let mut children = ChildrenProxy {
                    self_id: parent_id,
                    children: &mut parent.children,
                    child_service,
                };
                Some(children.insert(widget))
            // TODO ???
            // self.events.push((id, WidgetEvent::Change));
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
    /// Panics if widget does not exist, or if alias does not exist (if I = String)
    pub fn get<I: AsId<D>>(&self, id: I) -> &Widget {
        self.try_get(id).unwrap()
    }
    pub fn try_get<I: AsId<D>>(&self, id: I) -> Option<&Widget> {
        if let Some(id) = id.resolve(self) {
            if id == ROOT {
                return Some(&self.root);
            }
            if let Some(path) = self.child_service.borrow().paths.get(&id) {
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
        self.child_service
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
            if let Some(path) = self.child_service.borrow().paths.get(&id) {
                let mut current = &mut self.root;
                for id in path {
                    if let Some(child) = current.children.get_mut(id) {
                        current = child;
                    } else {
                        panic!(
                            "Incorrect path (gui programming error?).
                            {} not a child of {} on path {:?}",
                            id, current.id, path
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
    events: &mut Vec<(Id, WidgetEvent)>,
) {
    for child in w.children.values_mut() {
        if !old_paths.contains_key(&child.get_id()) {
            // If not known, issue an event
            events.push((child.get_id(), WidgetEvent::Change));
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
