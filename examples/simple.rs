use gui::{Gui, Button, Pos, FromWidget};

use vxdraw::{debtri::DebugTriangle, void_logger, ShowWindow, VxDraw, Color};
use input::Input;

fn main() {

    // Create GUI
    let mut gui = Gui::default();

    gui.add_widget(Button1, Button::new("B1".to_string(), 30, 30), Pos(0), Pos(0));
    gui.add_widget(Button2, Button::new("B2".to_string(), 30, 30),
            FromWidget (Button1, 50), FromWidget (Button1, 0));

    // 
    let mut vx = VxDraw::new(void_logger(), ShowWindow::Enable);
    let quad = vx.quads().add_layer(&vxdraw::quads::LayerOptions::new());
    let handle = vx.quads().add(&quad, vxdraw::quads::Quad::new());
    vx.quads().set_solid_color(&handle, Color::Rgba(0, 0, 0, 255));

    loop {
        gui.update(&Input::default(), 500, 500);
        vx.draw_frame();
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

use WidgetId::*;
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum WidgetId {
    Button1,
    Button2,
}
//TODO why do I need Default??
impl Default for WidgetId {
    fn default() -> Self {
        WidgetId::Button1
    }
}
