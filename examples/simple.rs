use gui::{Pos, FromWidget};
use gui::{Gui, Button, Widget};


use vxdraw::{debtri::DebugTriangle, void_logger, ShowWindow, VxDraw, Color};

use cgmath::SquareMatrix;
use cgmath::Matrix4;
use input::Input;
use gui_drawer::GuiDrawer;

fn main() {
    let mut vx = VxDraw::new(void_logger(), ShowWindow::Enable);
    vx.set_clear_color(Color::Rgba(0,0,0, 255));
    
    // let world_layer = vx.quads().add_layer(&vxdraw::quads::LayerOptions::new()
        // .fixed_perspective(Matrix4::identity()));

    // Create GUI
    let mut gui = Gui::default();
    let mut gui = GuiDrawer::new(gui, &mut vx);

    gui.state.0.insert(Button1,
        Widget::Button(Button::new("B1".to_string(), 60, 30)),
        Pos(100),
        Pos(100));
    gui.state.0.insert(Button2,
        Widget::Button(Button::new("B2".to_string(), 60, 30)),
        FromWidget (Button1, 0),
        FromWidget (Button1, 100));


    loop {
        let prspect = vx.perspective_projection();
        vx.set_perspective(prspect);

        gui.update(&Input::default() /*TODO*/, &mut vx);

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
impl Default for WidgetId {
    fn default() -> Self {
        WidgetId::Button1
    }
}

mod gui_drawer {
    use vxdraw::quads::Handle;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::ops::Deref;
    use std::fmt::Debug;
    use gui::{self, Widget, Gui};
    use cgmath::SquareMatrix;

    use vxdraw::{
        debtri::DebugTriangle,
        quads,
        text,
        void_logger,
        ShowWindow,
        VxDraw,
        Color
    };
    use cgmath::Matrix4;
    use input::Input;

    type Widgets = (gui::Button,);

    const DEJAVU: &[u8] = include_bytes!["../fonts/DejaVuSans.ttf"];

    fn vx_transform(pos: (i32, i32), sw: i32, sh: i32) -> (f32, f32) {
        (pos.0 as f32 / sw as f32 * 2.0 - 1.0,
         pos.1 as f32 / sh as f32 * 2.0 - 1.0)
    }
    fn vx_scale(x: i32, sw: i32) -> f32 {
        x as f32 / sw as f32 * 2.0
    }

    struct Button {
        quad: quads::Handle,
        text: text::Handle,
    }

    pub struct GuiDrawer<Id: Eq + Hash> {
        gui: Gui<Id, Widgets>,

        // Handles to VxDraw
        quads: quads::Layer,
        text: text::Layer,
        buttons: HashMap<Id, Button>,
    }
    impl<Id: Eq + Hash + Copy + Clone + Debug> GuiDrawer<Id> {
        pub fn new(mut gui: Gui<Id, Widgets>, vx: &mut VxDraw) -> GuiDrawer<Id> {
            let quads = vx.quads().add_layer(&vxdraw::quads::LayerOptions::new()
                .fixed_perspective(Matrix4::identity()));
            let mut text = vx.text().add_layer(DEJAVU, text::LayerOptions::new()
                .fixed_perspective(Matrix4::identity()));

            let mut buttons = HashMap::new();

            let (sw, sh) = vx.get_window_size_in_pixels();
            let (sw, sh) = (sw as i32, sh as i32);

            gui.update(&Input::default(), sw, sh);

            // Initiate state
            for (id, pos, widget) in gui.get_state() {
                match widget {
                    Widget::Button (button) => {
                        let quad = vx.quads().add(&quads, vxdraw::quads::Quad::new()
                            .translation(vx_transform(pos, sw, sh))
                            .width(vx_scale(button.w, sw))
                            .height(vx_scale(button.h, sh)));
                        vx.quads().set_solid_color(&quad, Color::Rgba(128,128,128, 255));

                        let text = vx.text().add(
                            &text,
                            &button.text,
                            text::TextOptions::new()
                                .font_size(30.0)
                                .translation(vx_transform(pos, sw, sh))
                        );
                        buttons.insert(id, Button {
                            quad,
                            text,
                        });
                    }
                }
            }
            GuiDrawer {
                gui,
                quads,
                text,
                buttons,
            }
        }
        pub fn update(&mut self, input: &Input, vxdraw: &mut VxDraw) {
            let (sw, sh) = vxdraw.get_window_size_in_pixels();
            let (sw, sh) = (sw as i32, sh as i32);

            let updates = self.gui.update(input, sw, sh);

            // TODO: apply updates ..
            for (id, pos) in updates.positions {
            }
            for (id, state) in updates.buttons {
            }
        }
    }
    impl<Id: Eq + Hash> Deref for GuiDrawer<Id> {
        type Target = Gui<Id>;
        fn deref(&self) -> &Gui<Id> {
            &self.gui
        }
    }
}
