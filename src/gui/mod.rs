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
    pub paths: IndexMap<String, Vec<String>>,

    /// Events collected outside update function, consumed when update is called
    events: Vec<(String, WidgetEvent)>,
}

impl<D: GuiDrawer> Gui<D> {
    pub fn new(drawer: D) -> Gui<D> {
        let root = Widget::new(String::new(), Container::new())
            .placement(Placement::fixed(0.0, 0.0))
            .size_hint(SizeHint::External, SizeHint::External);
        Gui {
            root,
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
    ) -> (Vec<(String, WidgetEvent)>, Capture) {
        let mouse = self.drawer.transform_mouse(input.get_mouse_position(), ctx);
        let (sw, sh) = self.drawer.window_size(ctx);
        self.root.size = (sw, sh);
        let (mut events, capture) = self.root.update(input, sw, sh, mouse, log.clone());

        // update parent relations
        let old_paths = std::mem::replace(&mut self.paths, IndexMap::new());
        update_paths_recurse(
            vec![],
            &mut self.root,
            &old_paths,
            &mut self.paths,
            &mut events,
        );
        events.extend(std::mem::replace(&mut self.events, Vec::new()));
        let ops = self.drawer.update(self, &events, log, ctx);
        for op in ops {
            match op {
                WidgetOp::Resize { id, size } => {
                    self.get_widget_mut(&id).unwrap().size = size;
                    events.push((id, WidgetEvent::ChangeSize));
                }
            }
        }
        (events, capture)
    }
    pub fn insert_widget(&mut self, parent_id: &str, widget: Widget) -> Option<()> {
        let id = widget.get_id().to_string();
        if let Some(parent) = self.get_widget_mut(parent_id) {
            // Insert
            parent.insert_child(widget);
            // Update paths
            let mut path = self.paths[parent_id].clone();
            path.push(parent_id.to_string());
            self.paths.insert(id.clone(), path);
            self.events.push((id, WidgetEvent::Change));
            Some(())
        } else {
            None
        }
    }
    pub fn insert_widget_in_root(&mut self, widget: Widget) {
        let id = widget.get_id().to_string();
        self.root.insert_child(widget);
        self.paths.insert(id.clone(), vec![]);
        self.events.push((id, WidgetEvent::Change));
    }
    pub fn get_widget(&self, id: &str) -> Option<&Widget> {
        if id.is_empty() {
            return Some(&self.root);
        }
        if let Some(path) = self.paths.get(id) {
            let mut current = &self.root;
            for id in path {
                if let Some(child) = current.get_child(id) {
                    current = child;
                } else {
                    panic!("Incorrect path (panicking to be sure to catch this error)");
                }
            }
            if let Some(child) = current.get_child(id) {
                Some(child)
            } else {
                panic!("Path is wrong - child not found.  child {}", id);
            }
        } else {
            None
        }
    }
    pub fn get_widget_mut(&mut self, id: &str) -> Option<&mut Widget> {
        if id.is_empty() {
            return Some(&mut self.root);
        }
        if let Some(path) = self.paths.get(id) {
            let mut current = &mut self.root;
            for id in path {
                if let Some(child) = current.get_child_mut(id) {
                    current = child;
                } else {
                    panic!("Incorrect path (panicking to be sure to catch this error)");
                }
            }
            if let Some(child) = current.get_child_mut(id) {
                Some(child)
            } else {
                panic!("Path is wrong - child not found.  child {}", id);
            }
        } else {
            None
        }
    }
    /// Recursive iterator of all widgets in the tree
    pub fn widgets_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Widget> + 'a> {
        self.root.recursive_children_iter()
    }
    /// Recursively process all widgets (mutably) in the tree
    pub fn widgets_mut(&mut self, f: &mut dyn FnMut(&mut Widget)) {
        recursive_children_mut(&mut self.root, f)
    }
}

// NOTE: can't be in `Interactive` because of F
fn recursive_children_mut(w: &mut Widget, f: &mut dyn FnMut(&mut Widget)) {
    for child in w.children_mut() {
        f(child);
    }
    for child in w.children_mut() {
        recursive_children_mut(child, f);
    }
}
fn update_paths_recurse(
    current_path: Vec<String>,
    w: &mut Widget,
    old_paths: &IndexMap<String, Vec<String>>,
    paths: &mut IndexMap<String, Vec<String>>,
    events: &mut Vec<(String, WidgetEvent)>,
) {
    for child in w.children_mut() {
        if !old_paths.contains_key(child.get_id()) {
            // If not known, issue an event
            events.push((child.get_id().to_string(), WidgetEvent::Change));
            println!(
                "[gui] Found new widget {:?} - Change event sent",
                child.get_id()
            );
        }
        paths.insert(child.get_id().to_string(), current_path.clone());

        let mut child_path = current_path.clone();
        child_path.push(child.get_id().to_string());
        update_paths_recurse(child_path, child, old_paths, paths, events);
    }
}
