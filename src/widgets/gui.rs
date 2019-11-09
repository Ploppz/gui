use crate::*;

#[derive(Default, Debug)]
pub struct Gui {
    pub widgets: HashMap<String, Widget>,
    screen: (f32, f32),
}

impl Gui {
    pub fn insert<W: Interactive + 'static>(
        &mut self,
        id: String,
        widget: W,
        place: Placement,
    ) {
        self.widgets.insert(id, Widget::new(widget, place));
    }
}

impl Interactive for Gui {
    fn inside(&self, _pos: (f32, f32), _size: (f32, f32), _mouse: (f32, f32)) -> bool {
        true
    }
    fn handle_event(&mut self, _event: WidgetEvent) -> bool {
        panic!("There is no reason to use this on the root")
    }
    fn captures(&self) -> Capture {
        panic!("There is no reason to use this on the root")
    }
    fn children(&mut self) -> Vec<(&str, &mut Widget)> {
        self.widgets.iter_mut().map(|(id, w)| (id.as_str(), w)).collect()
    }

}
