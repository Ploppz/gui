use crate::*;
use indexmap::IndexMap;
use slog::Logger;
mod drawer;
pub use drawer::*;
use std::{cell::RefCell, rc::Rc};

pub const ROOT: usize = 1;

pub trait AsId<D: GuiDrawer> {
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
}
impl ChildService {
    pub fn new_id(&mut self) -> Id {
        self.id_cnt += 1;
        self.id_cnt
    }
}

#[derive(Debug)]
pub struct Gui<D: GuiDrawer> {
    pub root: Widget,
    screen: (f32, f32),
    drawer: D,
    pub aliases: IndexMap<String, Id>,
    pub child_service: Rc<RefCell<ChildService>>,
    /// Events collected outside update function, consumed when update is called
    events: Vec<(Id, WidgetEvent)>,
}

impl<D: GuiDrawer> Gui<D> {
    pub fn new(drawer: D) -> Gui<D> {
        let child_service = Rc::new(RefCell::new(ChildService {
            paths: IndexMap::new(),
            id_cnt: ROOT,
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
    pub fn update(
        &mut self,
        input: &Input,
        log: Logger,
        ctx: &mut D::Context,
    ) -> (Vec<(Id, WidgetEvent)>, Capture) {
        let mouse = self.drawer.transform_mouse(input.get_mouse_position(), ctx);
        let (sw, sh) = self.drawer.window_size(ctx);
        self.root.config.set_size(sw, sh);
        let (mut events, capture) = self.root.update(input, sw, sh, mouse, log.clone());
        events.extend(std::mem::replace(&mut self.events, Vec::new()));

        // update parent relations
        let mut old_paths =
            std::mem::replace(&mut self.child_service.borrow_mut().paths, IndexMap::new());
        update_paths_recurse(
            vec![],
            &mut self.root,
            &mut old_paths,
            &mut self.child_service.borrow_mut().paths,
            &mut events,
        );
        // entries left in `old_paths` are deleted widget ids
        for deleted_id in old_paths.keys() {
            events.push((*deleted_id, WidgetEvent::Removed));
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
