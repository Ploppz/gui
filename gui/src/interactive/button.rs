#![allow(non_upper_case_globals)]
use crate::*;
use interactive::*;

#[derive(Debug)]
pub struct Button {}
impl Button {
    pub fn new() -> Button {
        Button {}
    }
}
impl Interactive for Button {
    fn init(&mut self, children: &mut ChildrenProxy) -> WidgetConfig {
        children.insert(Box::new(TextField::new(String::new())));
        WidgetConfig::default()
            .size_hint(SizeHint::Minimize, SizeHint::Minimize)
            .padding(4.0, 4.0, 6.0, 6.0)
    }

    fn captures(&self) -> Capture {
        Capture {
            mouse: true,
            keyboard: false,
        }
    }
}

#[derive(Lens, Debug)]
pub struct ToggleButton {
    pub state: bool,
}
impl ToggleButton {
    pub fn new() -> ToggleButton {
        ToggleButton { state: false }
    }
}
impl Interactive for ToggleButton {
    fn init(&mut self, children: &mut ChildrenProxy) -> WidgetConfig {
        children.insert(Box::new(TextField::new(String::new())));
        WidgetConfig::default()
            .size_hint(SizeHint::Minimize, SizeHint::Minimize)
            .padding(4.0, 4.0, 6.0, 6.0)
    }
    fn update(
        &mut self,
        id: Id,
        local_events: &[Event],
        _children: &mut ChildrenProxy,
        events: &mut Vec<Event>,
    ) {
        for event in local_events {
            if id == event.id {
                if let EventKind::Release = event.kind {
                    self.state = !self.state;
                    events.push(Event::change(event.id, Self::state));
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
