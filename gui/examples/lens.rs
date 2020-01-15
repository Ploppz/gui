use gui::{default::*, interactive::*, lens::*, *};
fn main() {
    let mut gui = Gui::new(NoDrawer);
    gui.insert_in_root_with_alias(ToggleButton::<()>::new(), "A".to_string());
    WidgetLens::new(&mut gui, "A")
        .chain(Widget::first_child)
        .chain(TextField::<()>::text)
        .put("Hey".to_string());

    println!(
        "Toggle button: {:?}",
        WidgetLens::new(&mut gui, "A").get_widget()
    );
    println!(
        "\nText field: {:?}",
        WidgetLens::new(&mut gui, "A")
            .chain(Widget::first_child)
            .get()
    );
    println!(
        "\nText: {}",
        WidgetLens::new(&mut gui, "A")
            .chain(Widget::first_child)
            .chain(TextField::<()>::text)
            .get()
    );
}
