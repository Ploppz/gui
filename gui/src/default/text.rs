use super::*;
use crate::*;
use interactive::*;

pub trait TextFieldStyle: StyleBound {}

#[derive(LensInternal, Debug)]
pub struct TextField<Style> {
    #[lens]
    pub text: String,
    // TODO (idea): lens that is not a LeafLens but can be further chained with fields of Style
    pub style: Style,
}
impl<Style: TextFieldStyle> TextField<Style> {
    pub fn new(text: String) -> TextField<Style> {
        TextField {
            text,
            style: Style::default(),
        }
    }
}
impl<Style: TextFieldStyle> Interactive for TextField<Style> {
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
    fn determine_size(&self, drawer: &mut dyn ContextFreeGuiDrawer) -> Option<(f32, f32)> {
        Some(drawer.text_size(&self.text))
    }
}
