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

    gui.add_widget(Button1, Button::new("B1".to_string(), 30, 30).wrap(), Pos(100), Pos(100));
    gui.add_widget(Button2, Button::new("B2".to_string(), 30, 30).wrap(),
            FromWidget (Button1, 0), FromWidget (Button1, 100));

    let mut gui = GuiDrawer::new(gui, &mut vx);

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
//TODO why do I need Default??
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
    use gui::{Pos, FromWidget};
    use gui::Gui;
    use cgmath::SquareMatrix;

    use vxdraw::{
        debtri::DebugTriangle,
        quads,
        void_logger,
        ShowWindow,
        VxDraw,
        Color
    };
    use cgmath::Matrix4;
    use input::Input;

    fn vx_transform(pos: (i32, i32), sw: i32, sh: i32) -> (f32, f32) {
        (pos.0 as f32 / sw as f32 * 2.0 - 1.0,
         pos.1 as f32 / sh as f32 * 2.0 - 1.0)
    }
    fn vx_scale(x: i32, sw: i32) -> f32 {
        x as f32 / sw as f32 * 2.0
    }

    struct Button {
        quad: quads::Handle,
    }
    pub struct GuiDrawer<Id: Eq + Hash> {
        gui: Gui<Id>,

        // Handles to VxDraw
        buttons: HashMap<Id, Button>,
    }
    impl<Id: Eq + Hash + Copy + Clone + Debug> GuiDrawer<Id> {
        pub fn new(gui: Gui<Id>, vx: &mut VxDraw) -> GuiDrawer<Id> {
            let mut g = GuiDrawer {
                gui,
                buttons: HashMap::new(),
            };
            let (sw, sh) = vx.get_window_size_in_pixels();
            let (sw, sh) = (sw as i32, sh as i32);

            g.gui.update(&Input::default(), sw, sh);
            let layer = vx.quads().add_layer(&vxdraw::quads::LayerOptions::new()
                .fixed_perspective(Matrix4::identity()));

            // Initiate state
            for (id, pos, widget) in g.gui.get_state() {
                let quad = vx.quads().add(&layer, vxdraw::quads::Quad::new()
                    .translation(vx_transform(pos, sw, sh))
                    .width(vx_scale(60, sw))
                    .height(vx_scale(30, sh)));
                vx.quads().set_solid_color(&quad, Color::Rgba(128,128,128, 255));
                println!("Quad pos={:?}  w={} h={}", vx_transform(pos, sw, sh), vx_scale(60, sw), vx_scale(30, sh));
                g.buttons.insert(id, Button {
                    quad,
                });
            }
            g
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
