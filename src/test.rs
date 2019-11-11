use crate::*;
use winit::event::{ElementState, ModifiersState, MouseButton};
use winput::{Input, MouseInput};

fn mouse_pressed() -> MouseInput {
    MouseInput {
        state: ElementState::Pressed,
        modifiers: ModifiersState::default(),
    }
}
fn single_button() -> Gui {
    let mut gui = Gui::new();
    gui.insert_widget_in_root("B1".to_string(),
        Button::new("B1".to_string()).wrap().placement(Placement::fixed(100.0, 100.0)));
    // NOTE: maybe a bad solution right now but size is (0.0, 0.0) by default because it depends on rendering
    gui.get_widget("B1").unwrap().size = (50.0, 50.0);
    gui
}
fn single_toggle_button() -> Gui {
    let mut gui = Gui::new();
    gui.insert_widget_in_root("B1".to_string(), ToggleButton::new("B1".to_string())
        .wrap().placement(Placement::fixed(100.0, 100.0)));
    gui.get_widget("B1").unwrap().size = (50.0, 50.0);
    gui
}
fn event_exists(events: &Vec<(String, WidgetEventState)>, target: WidgetEvent) -> bool {
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
    let relevant_events = events.into_iter().filter(|event| event.0.len() > 0).collect::<Vec<_>>();
    assert!(capture.mouse);
    assert_eq!(relevant_events.len(), 4);
    assert!(event_exists(&relevant_events, WidgetEvent::Press));
    assert!(event_exists(&relevant_events, WidgetEvent::Hover));
}

#[test]
fn test_mark_change() {
    let mut gui = single_toggle_button();

    // Manually change the toggle button
    println!("{:?}", gui.get_widget("B1").unwrap());
    gui.get_widget("B1").unwrap().downcast_mut::<ToggleButton>().unwrap()
        .state = true;

    let button = gui.get_widget("B1").unwrap();
    button.mark_change();
    button.downcast_mut::<ToggleButton>().unwrap().state = true;

    let (events, capture) = gui.update(&Input::default(), 0.0, 0.0, (0.0, 0.0));
    let relevant_events = events.into_iter().filter(|event| event.0.len() > 0).collect::<Vec<_>>();
    assert_eq!(relevant_events.len(), 1);
    assert!(event_exists(&relevant_events, WidgetEvent::Change));
    // Extra test:
    assert!(!capture.mouse);
}

#[test]
fn test_gui_paths() {
    // Test that gui updates paths correctly and that get_widget() which uses said paths, works
    // correctly.
    let mut gui = Gui::new();
    gui.insert_widget_in_root("B1".to_string(), ToggleButton::new("B1".to_string())
        .wrap().placement(Placement::fixed(100.0, 100.0)));

    gui.get_widget("B1").unwrap();
    gui.get_widget("B1").unwrap()
        .downcast_mut::<ToggleButton>().unwrap();

    // See if `update` updates paths correctly
    gui.update(&Input::default(), 0.0, 0.0, (100.0, 100.0));
    gui.get_widget("B1").unwrap();
    gui.get_widget("B1").unwrap()
        .downcast_mut::<ToggleButton>().unwrap();
}
