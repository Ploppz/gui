use gui::{Pos, FromWidget};
use gui::{Gui, Button, Widget};


use vxdraw::{debtri::DebugTriangle, void_logger, ShowWindow, VxDraw, Color};
use cgmath::Matrix4;
use input::Input;

fn main() {

    // Create GUI
    let mut gui = Gui::default();

    gui.add_widget(Button1, Button::new("B1".to_string(), 30, 30).wrap(), Pos(0), Pos(0));
    gui.add_widget(Button2, Button::new("B2".to_string(), 30, 30).wrap(),
            FromWidget (Button1, 50), FromWidget (Button1, 0));

    // 
    let mut vx = VxDraw::new(void_logger(), ShowWindow::Enable);
    vx.set_clear_color(Color::Rgba(0,0,0, 255));
    let quad = vx.quads().add_layer(&vxdraw::quads::LayerOptions::new()
        .fixed_perspective(Matrix4::identity()));
    let handle = vx.quads().add(&quad, vxdraw::quads::Quad::new()
        .translation((0.0, 0.0))
        .width(30.0)
        .height(30.0));
    vx.quads().set_solid_color(&handle, Color::Rgba(255,255,255, 255));

    loop {
        let prspect = vx.perspective_projection();
        vx.set_perspective(prspect);
        gui.update(&Input::default(), 500, 500);

        for (id, widget) in gui.widgets.iter() {
            match widget.widget {
                Widget::Button (ref button) => draw_button(&mut vx, &quad, button)
            }
        }

        vx.draw_frame();
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
fn draw_button(vx: &mut VxDraw, layer: &vxdraw::quads::Layer,  b: &Button) {
    vx.quads().add()
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
