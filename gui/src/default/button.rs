use super::*;
use crate::*;
use interactive::*;

pub trait ButtonStyle: StyleBound {
    /// Style of contained text field
    type TextField: TextFieldStyle;
}

#[derive(Debug)]
pub struct Button<Style> {
    pub style: Style,
}
impl<Style: ButtonStyle> Button<Style> {
    pub fn new() -> Button<Style> {
        Button {
            style: Style::default(),
        }
    }
}
impl<Style: ButtonStyle> Interactive for Button<Style> {
    fn init(&mut self, children: &mut ChildrenProxy, gui: &GuiShared) -> WidgetConfig {
        children.insert(
            Box::new(TextField::<Style::TextField>::new(String::new())),
            gui,
        );

        // Layout the one child in X direction with vertical centering
        WidgetConfig::default()
            .size_hint(SizeHint::Minimize, SizeHint::Minimize)
            .layout_direction(Axis::X)
            .layout_align(Anchor::Center)
            .padding(4.0, 4.0, 6.0, 6.0)
            .height(DEFAULT_BUTTON_HEIGHT)
    }

    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}

#[derive(LensInternal, Debug)]
pub struct ToggleButton<Style> {
    #[lens]
    pub state: bool,
    pub style: Style,
}
impl<Style: ButtonStyle> ToggleButton<Style> {
    pub fn new() -> ToggleButton<Style> {
        ToggleButton {
            state: false,
            style: Style::default(),
        }
    }
}
impl<Style: ButtonStyle> Interactive for ToggleButton<Style> {
    fn init(&mut self, children: &mut ChildrenProxy, gui: &GuiShared) -> WidgetConfig {
        children.insert(
            Box::new(TextField::<Style::TextField>::new(String::new())),
            gui,
        );
        // Layout the one child in X direction with vertical centering
        WidgetConfig::default()
            .size_hint(SizeHint::Minimize, SizeHint::Minimize)
            .layout_direction(Axis::X)
            .layout_align(Anchor::Center)
            .padding(4.0, 4.0, 6.0, 6.0)
            .height(DEFAULT_BUTTON_HEIGHT)
    }
    fn update(
        &mut self,
        id: Id,
        local_events: Vec<Event>,
        _children: &mut ChildrenProxy,
        gui: &GuiShared,
    ) {
        for event in local_events {
            if id == event.id {
                if let EventKind::Press = event.kind {
                    self.state = !self.state;
                    gui.borrow_mut()
                        .push_event(Event::change(event.id, Self::state));
                }
            }
        }
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test_common::*;
    use crate::*;
    #[test]
    fn test_toggle_button_state() {
        let mut fix = TestFixture::fixture();
        fix.update();

        let ((_, _), (events, _)) = fix.click_widget("ToggleButton 0");

        assert_events!(events, vec![EventKind::Release]);

        let btn = fix.gui.get("ToggleButton 0");
        let btn = btn.downcast_ref::<ToggleButton>().unwrap();
        assert_eq!(btn.state, true);
    }
}
