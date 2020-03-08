use gui::{default::*, lens::*, *};
fn main() {
    let mut gui = Gui::new(NoDrawer, &mut ());
    gui.insert_in_root_with_alias(ToggleButton::<()>::new(), "A".to_string());
    gui.access("A")
        .chain(Widget::first_child)
        .chain(TextField::<()>::text)
        .put("Hey".to_string());

    println!("Toggle button: {:?}", gui.access("A").get_widget());
    println!(
        "\nText field: {:?}",
        gui.access("A").chain(Widget::first_child).get()
    );
    println!(
        "\nText: {}",
        gui.access("A")
            .chain(Widget::first_child)
            .chain(TextField::<()>::text)
            .get()
    );
}
