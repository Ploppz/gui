use crate::*;

#[derive(Debug, Default)]
pub struct Container {}
impl Container {
    pub fn new() -> Container {
        Container {}
    }
}
impl Interactive for Container {
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
