use gui::{Button, Gui};
use gui::{FromWidget, Pos};
#[macro_use]
extern crate derive_deref;

use vxdraw::{void_logger, Color, ShowWindow, VxDraw};

// use cgmath::SquareMatrix;
// use cgmath::Matrix4;
use gui_drawer::GuiDrawer;
use winput::Input;

#[macro_export]
macro_rules! match_downcast_ref {
    ( $any:expr, $( $bind:ident : $ty:ty => $arm:expr ),*, _ => $default:expr ) => (
        $(
            if $any.is::<$ty>() {
                let $bind = $any.downcast_ref::<$ty>().unwrap();
                $arm
            } else
        )*
        {
            $default
        }
    )
}
#[macro_export]
macro_rules! match_downcast_mut {
    ( $any:expr, $( $bind:ident : $ty:ty => $arm:expr ),*, _ => $default:expr ) => (
        $(
            if $any.is::<$ty>() {
                let $bind = $any.downcast_mut::<$ty>().unwrap();
                $arm
            } else
        )*
        {
            $default
        }
    )
}

fn main() {
    let mut vx = VxDraw::new(void_logger(), ShowWindow::Enable);
    vx.set_clear_color(Color::Rgba(0, 0, 0, 255));

    // let world_layer = vx.quads().add_layer(&vxdraw::quads::LayerOptions::new()
    // .fixed_perspective(Matrix4::identity()));

    // Create GUI
    let mut gui = Gui::default();

    gui.insert(
        Button1,
        Button::new("B1".to_string(), 60.0, 30.0),
        Pos(100.0),
        Pos(100.0),
    );
    gui.insert(
        Button2,
        Button::new("B2".to_string(), 60.0, 30.0),
        FromWidget(Button1, 0.0),
        FromWidget(Button1, 100.0),
    );

    let mut gui = GuiDrawer::new(gui, &mut vx);
    // TODO: make it possible to add widgets after creating GuiDrawer
    let mut input = Input::default();
    let mut events = vx.events_loop().unwrap();

    loop {
        let prspect = vx.perspective_projection();
        vx.set_perspective(prspect);

        process_input(&mut input, &mut events);
        gui.update(&input, &mut vx);

        vx.draw_frame();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
fn process_input(s: &mut Input, events: &mut winit::EventsLoop) {
    use winit::*;
    s.prepare_for_next_frame();
    events.poll_events(|evt| {
        if let Event::WindowEvent { event, .. } = evt {
            match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    s.register_key(&input);
                }
                WindowEvent::MouseWheel {
                    delta, modifiers, ..
                } => {
                    if let winit::MouseScrollDelta::LineDelta(_, v) = delta {
                        s.register_mouse_wheel(v);
                    }
                }
                WindowEvent::MouseInput {
                    state,
                    button,
                    modifiers,
                    ..
                } => {
                    let input = winput::MouseInput { state, modifiers };
                    s.register_mouse_input(input, button);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let pos: (i32, i32) = position.into();
                    s.register_mouse_position(pos.0 as f32, pos.1 as f32);
                }
                _ => {}
            }
        }
    });
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
    use gui::{self, Gui, ButtonState};
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::ops::Deref;

    use cgmath::{Matrix4, Vector3};
    use vxdraw::{quads, text, Color, VxDraw};
    use winput::Input;

    const DEJAVU: &[u8] = include_bytes!["../fonts/DejaVuSans.ttf"];

    struct Button {
        prev_model: gui::Button,
        quad: quads::Handle,
        text: text::Handle,
        w: f32,
        h: f32,

    }

    #[derive(Deref, DerefMut)]
    pub struct GuiDrawer<Id: Eq + Hash> {
        #[deref_target]
        gui: Gui<Id>,

        // Handles to VxDraw
        quads: quads::Layer,
        text: text::Layer,
        buttons: HashMap<Id, Button>,
    }
    impl<Id: Eq + Hash + Copy + Clone + Debug> GuiDrawer<Id> {
        fn proj_matrix(vx: &mut VxDraw) -> Matrix4<f32> {
            let (sw, sh) = vx.get_window_size_in_pixels();
            let (sw, sh) = (sw as f32, sh as f32);
            // transform (0,0) -> (-1,-1)
            // transform (sw,sh) -> (1,1)
            Matrix4::from_translation(Vector3::new(-1.0, -1.0, 0.0))
                * Matrix4::from_nonuniform_scale(2.0 / sw, 2.0 / sh, 1.0)
        }

        pub fn new(mut gui: Gui<Id>, vx: &mut VxDraw) -> GuiDrawer<Id> {
            let quad_matrix = Self::proj_matrix(vx);
            let text_matrix = Self::proj_matrix(vx);
            let quads = vx
                .quads()
                .add_layer(&vxdraw::quads::LayerOptions::new().fixed_perspective(quad_matrix));
            let text = vx.text().add_layer(
                DEJAVU,
                text::LayerOptions::new().fixed_perspective(text_matrix),
            );

            let mut buttons = HashMap::new();

            let (sw, sh) = vx.get_window_size_in_pixels();
            let (sw, sh) = (sw as f32, sh as f32);

            // as long as we work with pixels we can pass through mouse pos

            gui.update(&Input::default(), sw, sh, 0.0, 0.0);

            // Initiate state
            for (id, w) in gui.widgets.iter() {
                let pos = w.pos;
                let widget = &w.widget;
                match_downcast_ref! {widget,
                    button: gui::Button => {
                        println!("Adding button");
                        println!("Pos: {:?}", widget.deref().deref());

                        let text = vx.text().add(
                            &text,
                            &button.text,
                            text::TextOptions::new()
                                .font_size(30.0)
                                .translation(pos.to_tuple())
                                .scale(300.0)
                                .origin((0.5, 0.5))
                        );

                        let (tw, th) = vx.text().get_model_size(&text);
                        let quad = vx.quads().add(&quads, vxdraw::quads::Quad::new()
                            .translation(pos.to_tuple())
                            .width(tw + 6.0)
                            .height(th + 4.0));
                        vx.quads().set_solid_color(&quad, Color::Rgba(128,128,128, 255));

                        buttons.insert(*id, Button {
                            prev_model: gui::Button {
                                text: String::new(),
                                w: 0.0,
                                h: 0.0,
                                state: ButtonState::None,
                            },
                            quad,
                            text,
                            w: tw,
                            h: th,
                        });
                    },
                    _ => panic!("Unexpected Widget!")
                }
            }

            GuiDrawer {
                gui,
                quads,
                text,
                buttons,
            }
        }
        pub fn update(&mut self, input: &Input, vx: &mut VxDraw) {
            let (sw, sh) = vx.get_window_size_in_pixels();
            let (sw, sh) = (sw as f32, sh as f32);
            let mouse = input.get_mouse_position();

            self.gui.update(input, sw, sh, mouse.0, mouse.1);

            let quad_matrix = Self::proj_matrix(vx);
            let text_matrix = Self::proj_matrix(vx);
            vx.quads().set_perspective(&self.quads, Some(quad_matrix));
            vx.text().set_perspective(&self.text, Some(text_matrix));

            for (id, w) in self.gui.widgets.iter_mut() {
                let pos = w.pos;
                let widget = &mut w.widget;
                match_downcast_mut! {widget,
                    button: gui::Button => {
                        // let (sw, _sh) = vx.get_window_size_in_pixels();
                        let element = &mut self.buttons.get_mut(id).unwrap();

                        // Update model size depending on recorded width of text
                        button.w = element.w;
                        button.h = element.h;

                        if button.state != element.prev_model.state {
                            vx.quads().set_solid_color(&element.quad, match button.state {
                                ButtonState::Hover => Color::Rgba (180,0,0, 255),
                                ButtonState::None => Color::Rgba(128,128,128, 255),
                            });
                        }

                        // let w = vx.text().get_model_size(&element.text);
                        // println!("Text width: {:?}", w);
                        element.prev_model = button.clone();
                    },
                    _ => panic!("Unexpected Widget!")
                }
            }
        }
    }
}
