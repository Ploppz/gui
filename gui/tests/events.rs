use gui::{interactive::*, test_common::*, *};

#[test]
fn test_idempotent_positioning() {
    // Verify that updating once is enough to complete positioning/sizing/layouting
    let mut fix = TestFixture::fixture();
    fix.update();
    for _ in 0..4 {
        let (e, _) = fix.update();
        assert_eq!(e.len(), 0);
    }
}

#[test]
fn test_button_click_capture_and_events() {
    let mut fix = TestFixture::fixture();
    fix.update();

    let ((press_events, press_capture), (release_events, release_capture)) =
        fix.click_widget("ToggleButton 0");

    let relevant_events = press_events
        .into_iter()
        .filter(|event| fix.gui.id_eq(event.id, "ToggleButton 0"))
        .chain(
            release_events
                .into_iter()
                .filter(|event| fix.gui.id_eq(event.id, "ToggleButton 0")),
        )
        .collect::<Vec<_>>();
    assert!(press_capture.mouse);
    assert!(release_capture.mouse);
    assert_eq!(relevant_events.len(), 4);
    assert_events!(
        relevant_events,
        vec![
            EventKind::Hover,
            EventKind::Press,
            EventKind::change(ToggleButton::state),
            EventKind::Release,
        ]
    );
}

#[test]
fn test_mark_change() {
    let mut fix = TestFixture::fixture();
    fix.update();

    // Manually change the toggle button
    WidgetLens::new(&mut fix.gui, "ToggleButton 0")
        .chain(ToggleButton::state)
        .put(true);

    let (events, capture) = fix.update();
    let relevant_events = events
        .into_iter()
        .filter(|event| fix.gui.id_eq(event.id, "ToggleButton 0"))
        .collect::<Vec<_>>();
    println!("{:?}", relevant_events);
    assert_eq!(relevant_events.len(), 1);
    assert_events!(
        relevant_events,
        vec![EventKind::change(ToggleButton::state),]
    );
    // Extra test:
    assert!(!capture.mouse);
}

#[test]
fn test_gui_change_pos() {
    let mut fix = TestFixture::fixture();
    let (events, _) = fix.update();
    let relevant_events = events
        .into_iter()
        .filter(|event| fix.gui.id_eq(event.id, "Button 1"))
        .collect::<Vec<_>>();
    assert_events!(
        relevant_events,
        vec![
            EventKind::change(Widget::pos),
            EventKind::change(Widget::size)
        ]
    );
}

#[test]
fn test_button_inside() {
    // TODO
}

#[test]
fn test_gui_paths() {
    // Test that gui updates paths correctly and that get_widget() which uses said paths, works
    // correctly.
    let mut fix = TestFixture::fixture();

    for (id, _) in fix.expected.iter() {
        fix.gui.get(id);
    }

    // See if `update` updates paths correctly
    fix.update();
    for (id, _) in fix.expected.iter() {
        fix.gui.get(id);

        if id.starts_with("ToggleButton ") {
            fix.gui.get_mut(id).downcast_mut::<ToggleButton>().unwrap();
        } else if id.starts_with("Button ") {
            fix.gui.get_mut(id).downcast_mut::<Button>().unwrap();
        }
    }
}
