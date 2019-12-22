#![allow(non_upper_case_globals)]
use crate::lens2::FieldLens;
use crate::*;

pub struct ButtonTextLens;
impl FieldLens for ButtonTextLens {
    type Target = String;
    fn get(&self, source: &Widget) -> &String {
        let text_widget = &source.children().values().next().unwrap();
        TextField::text.get(text_widget)
    }
    fn put(&self, value: Self::Target) -> Box<dyn FnOnce(&mut Widget)> {
        let mut proxy = w.children_proxy();
        let text_widget = proxy.values_mut().next().unwrap();
        TextField::text.with_mut(text_widget, f)
    }
}

#[derive(Debug)]
pub struct Button {}
impl Button {
    pub const text: ButtonTextLens = ButtonTextLens;
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

pub struct ToggleButtonTextLens;
impl Lens<Widget, String> for ToggleButtonTextLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, w: &Widget, f: F) -> V {
        let text = &w
            .children()
            .values()
            .next()
            .unwrap()
            .downcast_ref::<TextField>()
            .unwrap()
            .text;
        f(text)
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, w: &mut Widget, f: F) -> V {
        let mut proxy = w.children_proxy();
        let text_widget = proxy.values_mut().next().unwrap();
        let text = &mut text_widget.downcast_mut::<TextField>().unwrap().text;
        let old_text = text.clone();
        let result = f(text);
        if old_text != *text {
            text_widget.mark_change();
            println!("MARKED CHANGE IN TEXT");
        }
        result
    }
}
pub struct ToggleButtonStateLens;
impl Lens<Widget, bool> for ToggleButtonStateLens {
    fn with<V, F: FnOnce(&bool) -> V>(&self, w: &Widget, f: F) -> V {
        let state = &w.downcast_ref::<ToggleButton>().unwrap().state;
        f(state)
    }
    fn with_mut<V, F: FnOnce(&mut bool) -> V>(&self, w: &mut Widget, f: F) -> V {
        let mut proxy = w.children_proxy();
        let state = &mut w.downcast_mut::<ToggleButton>().unwrap().state;
        let old_state = state.clone();
        let result = f(state);
        if old_state != *state {
            w.mark_change();
        }
        result
    }
}

#[derive(Debug)]
pub struct ToggleButton {
    pub state: bool,
}
impl ToggleButton {
    /// Lens to access and modify the text of the button
    pub const text: ToggleButtonTextLens = ToggleButtonTextLens;
    /// Lens to access and modify the state of the button
    pub const state: ToggleButtonStateLens = ToggleButtonStateLens;
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

        let ((_, _), (events, _)) = fix.click_widget("ToggleButton 0");

        assert_events!(events, vec![WidgetEvent::Release]);

        let btn = fix.gui.get("ToggleButton 0");
        let btn = btn.downcast_ref::<ToggleButton>().unwrap();
        assert_eq!(btn.state, true);
    }
}
