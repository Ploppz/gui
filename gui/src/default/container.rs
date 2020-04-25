use super::*;
use crate::*;
use interactive::*;

#[derive(Debug, Default)]
pub struct Container;
impl Container {
    pub fn new() -> Container {
        Container
    }
}
impl Interactive for Container {
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}
