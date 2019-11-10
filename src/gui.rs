use crate::*;
use uuid::Uuid;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Gui {
    root: Widget,
    screen: (f32, f32),
    paths: IndexMap<String, Vec<String>>,
}

impl Gui {
    pub fn new() -> Gui {
        Gui {
            root: Widget::new(String::new(), Container::new(), Placement::fixed(0.0, 0.0)),
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
    pub fn insert_widget<W: Interactive + 'static>(
        &mut self,
        parent_id: &str,
        id: String,
        widget: W,
        place: Placement,
    ) -> Option<()> {
        self.get_widget(parent_id)
            .map(|parent| {
                parent.children().insert(id.clone(), Widget::new(id, widget, place));
            })
    }
    pub fn insert_widget_in_root<W: Interactive + 'static>(
        &mut self,
        id: String,
        widget: W,
        place: Placement,
    ) {
        self.root.children().insert(id.clone(), Widget::new(id, widget, place));
    }
    pub fn get_widget(&mut self, id: &str) -> Option<&mut Widget> {
        if let Some(path) = self.paths.get(id) {
            let mut current = &mut self.root;
            for id in path {
                current = &mut current.children()[id];
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
    for child in w.children().values_mut() {
        paths.insert(child.get_id().to_string(), current_path.clone());

        let mut child_path = current_path.clone();
        child_path.push(child.get_id().to_string());
        update_paths_recurse(child_path, child, paths);
    }
}
