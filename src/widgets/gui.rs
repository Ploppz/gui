use crate::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Gui {
    widgets: HashMap<String, Widget>,
    screen: (f32, f32),
    parents: HashMap<String, Option<String>,
}

impl Gui {
    /// Used when `Gui` is the root of the project.
    pub fn update(
        &mut self,
        input: &Input,
        sw: f32,
        sh: f32,
        mouse: (f32, f32),
    ) -> (Vec<(String, WidgetEventState)>, Capture) {
        // update parent relations
        self.parents = HashMap::new();
        for w in self.widgets.values() {
            self.update_parents()
        }
        // wrap in a Widget to use its update function
        Widget::new(Uuid::new_v4().to_string(), *self, Placement::fixed(0.0, 0.0))
            .update(input, sw, sh, mouse)
    }
    fn update_parents(&mut self, w: &Widget) {
        for 
    }
    pub fn insert<W: Interactive + 'static>(
        &mut self,
        id: String,
        widget: W,
        place: Placement,
    ) {
        self.widgets.insert(id.clone(), Widget::new(id, widget, place));
    }
    pub fn get_widget(&self, ) -> Option<Widget> {
    }

}

impl Interactive for Gui {
    fn inside(&self, _pos: (f32, f32), _size: (f32, f32), _mouse: (f32, f32)) -> bool {
        true
    }
    fn handle_event(&mut self, _event: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture::default()
    }
    fn children(&mut self) -> Vec<&mut Widget> {
        self.widgets.iter_mut().map(|(_id, w)| w).collect()
    }
}
