use crate::*;
use gui_derive::Lenses;

#[derive(Lenses, Debug)]
pub struct TextField {
    pub text: String,
}
impl TextField {
    pub fn new(text: String) -> TextField {
        TextField { text }
    }
}
impl Interactive for TextField {
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}
