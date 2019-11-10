use crate::*;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Gui {
    root: Widget,
    screen: (f32, f32),
    paths: IndexMap<String, Vec<String>>,
}

impl Gui {
    pub fn new() -> Gui {
        let root = Widget::new(
            String::new(),
            Container::new()
        ).placement(Placement::fixed(0.0, 0.0));
        Gui {
            root,
            screen: (0.0, 0.0),
            paths: IndexMap::new(),
        }
    }
    /// Used when `Gui` is the root of the project.
    pub fn update(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: (f32, f32),
    ) -> (Vec<(String, WidgetEventState)>, Capture) {
        // update parent relations
        self.paths = IndexMap::new();
        update_paths_recurse(vec![], &mut self.root, &mut self.paths);
        self.root.update(input, sw, sh, mouse)
    }
    pub fn insert_widget(&mut self, parent_id: &str, id: String, mut widget: Widget) -> Option<()> {
        widget.id = id.clone();
        self.get_widget(parent_id)
            .map(|parent| {
                parent.insert_child(id.clone(), widget);
            })
    }
    pub fn insert_widget_in_root(&mut self, id: String, mut widget: Widget) {
        widget.id = id.clone();
        self.root.insert_child(id.clone(), widget);
    }
    pub fn get_widget(&mut self, id: &str) -> Option<&mut Widget> {
        if let Some(path) = self.paths.get(id) {
            let mut current = &mut self.root;
            for id in path {
                if let Some(child) = current.get_child(id) {
                    current = child;
                } else {
                    panic!("Incorrect path (panicking to be sure to catch this error)");
                }
            }
            Some(current)
        } else {
            None
        }
    }
    pub fn widgets_iter<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Widget> + 'a> {
        self.root.recursive_children_iter()
    }
}
fn update_paths_recurse(current_path: Vec<String>, w: &mut Widget, paths: &mut IndexMap<String, Vec<String>>) {
    for child in w.children() {
        paths.insert(child.get_id().to_string(), current_path.clone());

        let mut child_path = current_path.clone();
        child_path.push(child.get_id().to_string());
        update_paths_recurse(child_path, child, paths);
    }
}
