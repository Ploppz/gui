use crate::*;
use indexmap::IndexMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Button {
    children: IndexMap<String, Widget>,
}
impl Button {
    pub fn new(text: String) -> Button {
        let id = Uuid::new_v4().to_string();
        let mut children = IndexMap::new();
        children.insert(id.clone(), TextField::new(text).wrap(id));
        Button { children }
    }
    pub fn set_text(&mut self, text: String) {
        let tf = self
            .children
            .values_mut()
            .next()
            .unwrap()
            .downcast_mut::<TextField>()
            .unwrap();
        tf.text = text;
    }
}
impl Interactive for Button {
    fn wrap(self, id: String) -> Widget {
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
    fn children<'a>(&'a self) -> &IndexMap<String, Widget> {
        &self.children
    }
    fn children_mut<'a>(&'a mut self) -> &mut IndexMap<String, Widget> {
        &mut self.children
    }
}

#[derive(Debug)]
pub struct ToggleButton {
    pub state: bool,
    children: IndexMap<String, Widget>,
}
impl ToggleButton {
    pub fn new(text: String) -> ToggleButton {
        let id = Uuid::new_v4().to_string();
        let mut children = IndexMap::new();
        children.insert(id.clone(), TextField::new(text).wrap(id));
        ToggleButton {
            children,
            state: false,
        }
    }
}
impl Interactive for ToggleButton {
    fn wrap(self, id: String) -> Widget {
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
    fn children<'a>(&'a self) -> &IndexMap<String, Widget> {
        &self.children
    }
    fn children_mut<'a>(&'a mut self) -> &mut IndexMap<String, Widget> {
        &mut self.children
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

        let btn = fix.gui.get_widget("ToggleButton 0").unwrap();
        let btn = btn.downcast_ref::<ToggleButton>().unwrap();
        assert_eq!(btn.state, true);
    }
}
