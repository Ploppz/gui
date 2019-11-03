use crate::*;
use winit::event::{ElementState, ModifiersState, MouseButton};
use winput::{Input, MouseInput};

fn mouse_pressed() -> MouseInput {
    MouseInput {
        state: ElementState::Pressed,
        modifiers: ModifiersState::default(),
    }
}
fn single_button() -> Gui<u8> {
    let mut gui = Gui::default();
    gui.insert(0u8, Button::new("B1".to_string()), Abs (Pos(100.0), Pos(100.0)));
    // NOTE: maybe a bad solution right now but size is (0.0, 0.0) by default because it depends on rendering
    gui.widgets.get_mut(&0).unwrap().size = (50.0, 50.0);
    gui
}
fn single_toggle_button() -> Gui<u8> {
    let mut gui = Gui::default();
    gui.insert(0u8, ToggleButton::new("B1".to_string()), Abs (Pos(100.0), Pos(100.0)));
    gui.widgets.get_mut(&0).unwrap().size = (50.0, 50.0);
    gui
}
fn event_exists(events: &Vec<(u8, WidgetEventState)>, target: WidgetEvent) -> bool {
    events
        .iter()
        .find(|(_, event)| event.event == target)
        .is_some()
}

#[test]
fn test_button_press_capture_and_events() {
    let mut gui = single_button();
    let mut input = Input::default();
    input.register_mouse_input(mouse_pressed(), MouseButton::Left);
    // NOTE: gui.update() ignores `input`'s mouse position, as a transformed one is passed:
    let (events, capture) = gui.update(&input, 0.0, 0.0, (100.0, 100.0));
    assert!(capture.mouse);
    assert_eq!(events.len(), 2);
    assert!(event_exists(&events, WidgetEvent::Press));
    assert!(event_exists(&events, WidgetEvent::Hover));
}

#[test]
fn test_mark_change() {
    let mut gui = single_toggle_button();

    // Manually change the toggle button
    gui.widgets.get_mut(&0).unwrap().downcast_mut::<ToggleButton>().unwrap()
        .state = true;
    gui.mark_change(0);

    let (events, capture) = gui.update(&Input::default(), 0.0, 0.0, (0.0, 0.0));
    assert_eq!(events.len(), 1);
    assert!(event_exists(&events, WidgetEvent::Change));
    // Extra test:
    assert!(!capture.mouse);
}
