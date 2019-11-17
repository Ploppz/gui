use crate::*;
use winit::event::{ElementState, ModifiersState, MouseButton};
use winput::{Input, MouseInput};

pub fn press_left_mouse(s: &mut Input) {
    s.register_mouse_input(
        MouseInput {
            state: ElementState::Pressed,
            modifiers: ModifiersState::default(),
        },
        MouseButton::Left,
    );
}
pub fn release_left_mouse(s: &mut Input) {
    s.register_mouse_input(
        MouseInput {
            state: ElementState::Released,
            modifiers: ModifiersState::default(),
        },
        MouseButton::Left,
    );
}
pub fn mouse_pressed() -> MouseInput {
    MouseInput {
        state: ElementState::Pressed,
        modifiers: ModifiersState::default(),
    }
}
pub fn single_button() -> Gui {
    let mut gui = Gui::new();
    gui.insert_widget_in_root(
        Button::new("B1".to_string())
            .wrap("B1".to_string())
            .placement(Placement::fixed(100.0, 100.0)),
    );
    // NOTE: maybe a bad solution right now but size is (0.0, 0.0) by default because it depends on rendering
    gui.get_widget("B1").unwrap().size = (50.0, 50.0);
    gui
}
pub fn single_toggle_button() -> Gui {
    let mut gui = Gui::new();
    gui.insert_widget_in_root(
        ToggleButton::new("B1".to_string())
            .wrap("B1".to_string())
            .placement(Placement::fixed(100.0, 100.0)),
    );
    gui.get_widget("B1").unwrap().size = (50.0, 50.0);
    gui
}
#[macro_export]
macro_rules! assert_events {
    ($events:expr, $expected:expr) => {
        let mut events = $events.clone();
        let expected = $expected;
        let events_freeze = $events.clone();
        for expected_event in expected.iter() {
            if let Some(idx) = events.iter().enumerate().find_map(|(i, (_, e))| {
                if *e == *expected_event {
                    Some(i)
                } else {
                    None
                }
            }) {
                events.remove(idx);
            } else {
                panic!(
                    "\nAssertion failed: Event\n{:#?}\n\nnot in\n{:#?}",
                    expected_event, events_freeze
                );
            }
        }
    };
}
fn event_exists(events: &Vec<(String, WidgetEvent)>, target: WidgetEvent) -> bool {
    events.iter().find(|(_, event)| *event == target).is_some()
}

#[test]
fn test_button_press_capture_and_events() {
    let mut gui = single_button();
    let mut input = Input::default();
    press_left_mouse(&mut input);
    // NOTE: gui.update() ignores `input`'s mouse position, as a transformed one is passed:
    let (events, capture) = gui.update(&input, 0.0, 0.0, (101.0, 101.0));
    let relevant_events = events
        .into_iter()
        .filter(|event| event.0 == "B1")
        .collect::<Vec<_>>();
    assert!(capture.mouse);
    assert_eq!(relevant_events.len(), 4);
    assert_events!(
        relevant_events,
        vec![
            WidgetEvent::Press,
            WidgetEvent::Hover,
            WidgetEvent::ChangePos,
            WidgetEvent::ChangeSize
        ]
    );
}

#[test]
fn test_mark_change() {
    let mut gui = single_toggle_button();

    // Manually change the toggle button
    println!("{:?}", gui.get_widget("B1").unwrap());
    gui.get_widget("B1")
        .unwrap()
        .downcast_mut::<ToggleButton>()
        .unwrap()
        .state = true;

    let button = gui.get_widget("B1").unwrap();
    button.mark_change();
    button.downcast_mut::<ToggleButton>().unwrap().state = true;

    let (events, capture) = gui.update(&Input::default(), 0.0, 0.0, (0.0, 0.0));
    let relevant_events = events
        .into_iter()
        .filter(|event| event.0 == "B1")
        .collect::<Vec<_>>();
    println!("{:?}", relevant_events);
    assert_eq!(relevant_events.len(), 3);
    assert_events!(
        relevant_events,
        vec![
            WidgetEvent::Change,
            WidgetEvent::ChangePos,
            WidgetEvent::ChangeSize,
        ]
    );
    // Extra test:
    assert!(!capture.mouse);
}

#[test]
fn test_gui_change_pos() {
    let mut gui = single_toggle_button();
    let (events, capture) = gui.update(&Input::default(), 0.0, 0.0, (0.0, 0.0));
    let relevant_events = events
        .into_iter()
        .filter(|event| event.0 == "B1")
        .collect::<Vec<_>>();
    assert_events!(relevant_events, vec![WidgetEvent::ChangePos]);
}

#[test]
fn test_button_inside() {
    // TODO
}

#[test]
fn test_gui_paths() {
    // Test that gui updates paths correctly and that get_widget() which uses said paths, works
    // correctly.
    let mut gui = Gui::new();
    gui.insert_widget_in_root(
        ToggleButton::new("B1".to_string())
            .wrap("B1".to_string())
            .placement(Placement::fixed(100.0, 100.0)),
    );

    gui.get_widget("B1").unwrap();
    gui.get_widget("B1")
        .unwrap()
        .downcast_mut::<ToggleButton>()
        .unwrap();

    // See if `update` updates paths correctly
    gui.update(&Input::default(), 0.0, 0.0, (101.0, 101.0));
    gui.get_widget("B1").unwrap();
    gui.get_widget("B1")
        .unwrap()
        .downcast_mut::<ToggleButton>()
        .unwrap();
}
