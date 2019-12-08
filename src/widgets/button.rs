use crate::*;

#[derive(Debug)]
pub struct Button {
    text: String,
    text_changed: bool,
}
impl Button {
    pub fn new(text: String) -> Button {
        Button {
            text,
            text_changed: false,
        }
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.text_changed = true;
    }
}
impl Interactive for Button {
    fn init(&mut self) -> Vec<Widget> {
        vec![TextField::new(self.text.clone()).wrap(0)]
    }
    fn wrap(self, id: Id) -> Widget {
        Widget::new(id, self).padding(4.0, 4.0, 6.0, 6.0)
    }

    fn handle_event(&mut self, _: WidgetEvent) -> bool {
        false
    }
    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}

#[derive(Debug)]
pub struct ToggleButton {
    pub state: bool,
    text: String,
    text_changed: bool,
}
impl ToggleButton {
    pub fn new(text: String) -> ToggleButton {
        ToggleButton {
            text,
            text_changed: false,
            state: false,
        }
    }
}
impl Interactive for ToggleButton {
    fn wrap(self, id: usize) -> Widget {
        Widget::new(id, self).padding(4.0, 4.0, 6.0, 6.0)
    }
    fn handle_event(&mut self, event: WidgetEvent) -> bool {
        if let WidgetEvent::Release = event {
            self.state = !self.state;
            true
        } else {
            false
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

        let ((_, _), (events, _)) = fix.click_widget(unimplemented!());

        assert_events!(events, vec![WidgetEvent::Release]);

        let btn = fix.gui.get_widget(unimplemented!()).unwrap();
        let btn = btn.downcast_ref::<ToggleButton>().unwrap();
        assert_eq!(btn.state, true);
    }
}
