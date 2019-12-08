use crate::*;
use indexmap::IndexMap;
use slog::Logger;
mod drawer;
pub use drawer::*;

#[derive(Debug)]
pub struct Gui<D: GuiDrawer> {
    pub root: Widget,
    screen: (f32, f32),
    drawer: D,
    pub paths: IndexMap<Id, Vec<Id>>,

    id_cnt: usize,
    /// Events collected outside update function, consumed when update is called
    events: Vec<(Id, WidgetEvent)>,
}

impl<D: GuiDrawer> Gui<D> {
    pub fn new(drawer: D) -> Gui<D> {
        let root = Widget::new(1, Container::new())
            .placement(Placement::fixed(0.0, 0.0))
            .size_hint(SizeHint::External, SizeHint::External);
        Gui {
            root,
            id_cnt: 1,
            drawer,
            screen: (0.0, 0.0),
            paths: IndexMap::new(),
            events: Vec::new(),
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
        self.root.size = (sw, sh);
        let (mut events, capture) = self.root.update(input, sw, sh, mouse, log.clone());
        events.extend(std::mem::replace(&mut self.events, Vec::new()));

        // update parent relations
        let mut old_paths = std::mem::replace(&mut self.paths, IndexMap::new());
        update_paths_recurse(
            vec![],
            &mut self.root,
            &mut old_paths,
            &mut self.paths,
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
                    self.get_widget_mut(id).unwrap().size = size;
                    events.push((id, WidgetEvent::ChangeSize));
                }
            }
        }
        (events, capture)
    }
    pub fn insert_widget(&mut self, parent_id: Id, widget: Widget) -> Option<()> {
        let id = widget.get_id();
        if let Some(parent) = self.get_widget_mut(parent_id) {
            // Insert
            parent.children.insert(widget.id.clone(), widget);
            // Update paths
            let mut path = self.paths[&parent_id].clone();
            path.push(parent_id);
            self.paths.insert(id, path);
            self.events.push((id, WidgetEvent::Change));
            Some(())
        } else {
            None
        }
    }
    pub fn insert_widget_in_root(&mut self, widget: Widget) {
        let id = widget.get_id();
        self.root.children.insert(widget.id, widget);
        self.paths.insert(id, vec![]);
        self.events.push((id, WidgetEvent::Change));
    }
    pub fn get_widget(&self, id: Id) -> Option<&Widget> {
        if id == 1 {
            return Some(&self.root);
        }
        if let Some(path) = self.paths.get(&id) {
            let mut current = &self.root;
            for id in path {
                if let Some(child) = current.children.get(id) {
                    current = child;
                } else {
                    panic!("Incorrect path (panicking to be sure to catch this error)");
                }
            }
            if let Some(child) = current.children.get(&id) {
                Some(child)
            } else {
                panic!("Path is wrong - child not found.  child {}", id);
            }
        } else {
            None
        }
    }
    pub fn get_widget_mut(&mut self, id: Id) -> Option<&mut Widget> {
        if id == 1 {
            return Some(&mut self.root);
        }
        if let Some(path) = self.paths.get(&id) {
            let mut current = &mut self.root;
            for id in path {
                if let Some(child) = current.children.get_mut(id) {
                    current = child;
                } else {
                    panic!("Incorrect path (panicking to be sure to catch this error)");
                }
            }
            if let Some(child) = current.children.get_mut(&id) {
                Some(child)
            } else {
                panic!("Path is wrong - child not found.  child {}", id);
            }
        } else {
            None
        }
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
