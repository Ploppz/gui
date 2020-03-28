use gui::{lens::*, test_common::*, *};
// use slog::{o, Discard, Logger};

/// Test whether the layout alg completes in one single update.
fn test_idempotence(gui: &mut TestGui, initial_events: Option<Vec<Event>>) {
    let initial_events = initial_events.unwrap_or_else(|| gui.update().0);
    assert!(
        initial_events.len() > 0,
        "TEST ERROR: the premise of the test is that the initial update does yield some errors"
    );
    for _ in 0..4 {
        let (events, _) = gui.update();
        if !events.is_empty() {
            use std::fmt::Write;
            let mut s = "Events:\n".to_string();
            for event in events {
                let repeated = initial_events.iter().any(|e| *e == event);
                write!(s, "\t{:?}", event).unwrap();
                if repeated {
                    write!(s, " (repeated)").unwrap();
                }
                writeln!(s).unwrap();
            }
            panic!("{}", s)
        }
    }
}

#[test]
fn test_fixture_idempotence() {
    // Verify that updating once is enough to complete positioning/sizing/layouting
    let mut fix = TestFixture::fixture();
    test_idempotence(&mut fix.gui, None);
}
#[test]
fn test_select_idempotence() {
    // Test idemptence of layout algorithm with one dropdown button after clicking it.
    //
    let mut gui = TestGui::new();
    gui.insert_in_root(Button::new());
    let id = gui.insert_in_root(
        Select::new()
            .option("one".to_string(), "one".to_string())
            .option("two".to_string(), "two".to_string()),
    );
    gui.update();

    let click_pos =
        *gui.access(id).chain(Widget::pos).get() + *gui.access(id).chain(Widget::size).get() / 2.0;

    let (events, _) = gui.press(click_pos);

    let main_button_id = *gui
        .access(id)
        .chain(Widget::first_child)
        .chain(Widget::id)
        .get();
    // println!("Events 1: {:#?}", events);
    assert!(events.iter().any(|e| *e
        == Event {
            id: main_button_id,
            kind: EventKind::Press
        }));

    let mut has_made_new_widgets = false;
    for event in &events {
        has_made_new_widgets |= event.kind == EventKind::New;
    }
    assert!(has_made_new_widgets);
    // If any of the above asserts fail it might mean we failed to press the button
    // Setup is done, now test idemptence:
    println!("There should be no more events now");
    test_idempotence(&mut gui, Some(events));
}

#[test]
fn test_select_release_does_nothing() {
    // releasing the mouse button does nothing (only pressing)
    let mut gui = TestGui::new();
    let id = gui.insert_in_root(
        Select::new()
            .option("one".to_string(), "one".to_string())
            .option("two".to_string(), "two".to_string()),
    );
    gui.update();
    let click_pos =
        *gui.access(id).chain(Widget::pos).get() + *gui.access(id).chain(Widget::size).get() / 2.0;

    let (_, _) = gui.press(click_pos);
    let (events, _) = gui.release();
    assert_eq!(
        events
            .iter()
            .filter(|e| e.kind != EventKind::Release)
            .count(),
        0
    );
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
    fix.gui
        .access("ToggleButton 0")
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

// TEMPORARY

use gui::{GuiDrawer, Widget};
pub fn print_widget_tree<D: GuiDrawer>(gui: &Gui<D>) {
    use indexmap::IndexMap;
    use ptree::{output::print_tree, TreeBuilder};
    let aliases = gui
        .aliases
        .iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect::<IndexMap<usize, String>>();
    let mut tree = TreeBuilder::new(gui.root.get_id().to_string());
    fn recurse<E: GuiDrawer>(
        tree: &mut TreeBuilder,
        w: &Widget,
        gui: &Gui<E>,
        aliases: &IndexMap<usize, String>,
    ) {
        for child in w.children().values() {
            let alias = if let Some(alias) = aliases.get(&child.get_id()) {
                format!(" \"{}\"", alias)
            } else {
                String::new()
            };
            let name = format!(
                "{}{}      pos={} size={}",
                child.get_id(),
                alias,
                child.pos,
                child.size
            );
            tree.begin_child(name);
            recurse(tree, &child, gui, aliases);
            tree.end_child();
        }
    }
    recurse(&mut tree, &gui.root, &gui, &aliases);
    let tree = tree.build();

    print_tree(&tree).unwrap();
}
