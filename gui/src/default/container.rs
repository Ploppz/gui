use super::*;
use crate::*;
use interactive::*;

pub trait ContainerStyle: StyleBound {}

#[derive(Debug, Default)]
pub struct Container<Style> {
    pub style: Style,
}
impl<Style: ContainerStyle> Container<Style> {
    pub fn new() -> Container<Style> {
        Container {
            style: Style::default(),
        }
    }
}
impl<Style: ContainerStyle> Interactive for Container<Style> {
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}
