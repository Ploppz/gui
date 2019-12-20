use crate::*;

#[derive(Debug)]
pub struct TextField {
    pub text: String,
}
impl TextField {
    pub const text: TextFieldLens = TextFieldLens;
    pub fn new(text: String) -> TextField {
        TextField { text }
    }
}
impl Interactive for TextField {
    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}

pub struct TextFieldLens;
impl Lens<Widget, String> for TextFieldLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, w: &Widget, f: F) -> V {
        let text = &w.downcast_ref::<TextField>().unwrap().text;
        f(text)
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, w: &mut Widget, f: F) -> V {
        let text = &mut w
            .downcast_mut::<TextField>()
            .expect("Expected TextField")
            .text;
        let old_text = text.clone();
        let result = f(text);
        if old_text != *text {
            w.mark_change();
        }
        result
    }
}
