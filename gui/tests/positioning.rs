use gui::{lens::*, test_common::*, vec::*, *};

#[test]
fn test_select_on_click() {
    // Test size of dropdown buttons
    let mut gui = TestGui::new();
    let select_id = gui.insert_in_root(
        Select::new()
            .with_option("One".into(), "one".into())
            .with_option("Two".into(), "two".into()),
    );

    // Hard-code the expected size of the text fields
    let text_size = Vec2::<f32>::new(30.0, 10.0);

    gui.update();

    assert_eq!(
        text_size,
        *gui.access(select_id)
            .chain(Select::main_button)
            .chain(Button::text_field)
            .chain(Widget::size)
            .get()
    );
}

#[test]
fn test_setting_button_size() {
    const SIZE: f32 = 8.64;
    let mut gui = TestGui::new();
    let id = gui.insert_in_root(Button::new());
    gui.access(id)
        .chain(Button::text_field)
        .configure(|config| {
            config.set_size_hint(SizeHint::External(SIZE), SizeHint::External(SIZE));
        });
    gui.update();
    assert_eq!(
        gui.access(id)
            .chain(Button::text_field)
            .chain(Widget::size)
            .get()
            .x,
        SIZE
    );
}
