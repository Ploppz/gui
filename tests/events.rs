use gui::test_common::*;
use gui::*;
use slog::o;
use winput::Input;

#[test]
fn test_idempotent_positioning() {
    // Verify that updating once is enough to complete positioning/sizing/layouting
    let mut fix = TestFixture::fixture();
    fix.update();
    for i in 0..4 {
        let (e, _) = fix.update();
        assert_eq!(e.len(), 0);
    }
}

#[test]
fn test_button_press_capture_and_events() {
    let log = slog::Logger::root(slog::Discard, o!());
    let mut fix = TestFixture::fixture();
    fix.update();

    let ((events, capture), (_, _)) = fix.click_widget("ToggleButton 0");

    let relevant_events = events
        .into_iter()
        .filter(|event| event.0 == "ToggleButton 0")
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
    let log = slog::Logger::root(slog::Discard, o!());
    let mut gui = single_toggle_button();

    // Manually change the toggle button
    println!("{:?}", gui.get_widget("B1").unwrap());
    gui.get_widget_mut("B1")
        .unwrap()
        .downcast_mut::<ToggleButton>()
        .unwrap()
        .state = true;

    let button = gui.get_widget_mut("B1").unwrap();
    button.mark_change();
    button.downcast_mut::<ToggleButton>().unwrap().state = true;

    let (events, capture) = gui.update(&Input::default(), log, &mut ());
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
    let log = slog::Logger::root(slog::Discard, o!());
    let mut gui = single_toggle_button();
    let (events, _capture) = gui.update(&Input::default(), log, &mut ());
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
    let log = slog::Logger::root(slog::Discard, o!());
    let mut gui = Gui::new(NoDrawer);
    gui.insert_widget_in_root(
        ToggleButton::new("B1".to_string())
            .wrap("B1".to_string())
            .placement(Placement::fixed(100.0, 100.0)),
    );

    gui.get_widget("B1").unwrap();
    gui.get_widget_mut("B1")
        .unwrap()
        .downcast_mut::<ToggleButton>()
        .unwrap();

    // See if `update` updates paths correctly
    gui.update(&Input::default(), log, &mut ());
    gui.get_widget("B1").unwrap();
    gui.get_widget_mut("B1")
        .unwrap()
        .downcast_mut::<ToggleButton>()
        .unwrap();
}
