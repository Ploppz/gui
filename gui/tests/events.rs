use gui::{lens::*, test_common::*, *};
use slog::{o, Discard, Logger};
use winput::Input;

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
fn test_idempotent_positioning2() {
    // this one has only a dropdown button which is clicked
    let log = Logger::root(Discard, o!());
    let mut input = Input::default();
    let mut gui = Gui::new(NoDrawer);
    gui.insert_in_root_with_alias(
        DropdownButton::new()
            .option("one".to_string(), "one".to_string())
            .option("two".to_string(), "two".to_string()),
        "A".to_string(),
    );
    gui.update(&input, log.clone(), &mut ());

    // TODO: if we used our own coordinate struct with operators it would be easier
    let pos = gui
        .access("A")
        .chain(Widget::first_child)
        .chain(Widget::pos)
        .get()
        .clone();
    let size = gui
        .access("A")
        .chain(Widget::first_child)
        .chain(Widget::pos)
        .get()
        .clone();
    let click_pos = (pos.0 + size.0 / 2.0, pos.1 + size.1 / 2.0);

    input.register_mouse_position(click_pos.0, click_pos.1);

    press_left_mouse(&mut input);
    let (events, _) = gui.update(&input, log.clone(), &mut ());
    assert!(events.len() != 0);
    release_left_mouse(&mut input);
    let (events, _) = gui.update(&input, log.clone(), &mut ());
    assert!(events.len() != 0);
    let mut has_made_new_widgets = false;
    for event in events {
        has_made_new_widgets |= event.kind == EventKind::New;
    }
    assert!(has_made_new_widgets);
    // if any of the above asserts fail it might mean we failed to press the button
    println!("Should be done now");

    // the aim of this test is to test whether the layout alg completes in one single update, after
    // a dropdown button has been clicked. (dropdown button selected because its logic adds new
    // widgets)
    for _ in 0..4 {
        let (events, _) = gui.update(&input, log.clone(), &mut ());
        assert_eq!(events.len(), 0);
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
                "{}{}      pos{:?} size{:?}",
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
