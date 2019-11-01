use gui::Placement::*;
use gui::*;
#[macro_use]
extern crate derive_deref;

use vxdraw::{void_logger, Color, ShowWindow, VxDraw};
use winit::{event::*, event_loop::ControlFlow};

// use cgmath::SquareMatrix;
// use cgmath::Matrix4;
use gui::drawers::vxdraw::GuiDrawer;
use winput::Input;

fn main() {
    let events = winit::event_loop::EventLoop::new();
    let mut vx = VxDraw::new(void_logger(), ShowWindow::Enable, &events);
    vx.set_clear_color(Color::Rgba(0, 0, 0, 255));

    // let world_layer = vx.quads().add_layer(&vxdraw::quads::LayerOptions::new()
    // .fixed_perspective(Matrix4::identity()));

    // Create GUI
    let mut gui = Gui::default();

    gui.insert(
        Button1,
        Button::new("B1".to_string()),
        Pos(100.0),
        Pos(100.0),
    );
    gui.insert(
        Button2,
        ToggleButton::new("B2".to_string()),
        FromWidget(Button1, 0.0),
        FromWidget(Button1, 100.0),
    );

    let mut gui = GuiDrawer::new(gui, &mut vx);
    // TODO: make it possible to add widgets after creating GuiDrawer
    let mut input = Input::default();

    events.run(move |evt, _, control_flow| {
        let prspect = vx.perspective_projection();
        vx.set_perspective(prspect);

        process_input(&mut input, evt);
        let (events, capture) = gui.update(&input, &mut vx);

        // Just to show how to handle events in the applications
        for (id, event) in events {
            println!("[gui event][id={:?}] {:?}", id, event.event);
            // TODO: update a text field or something instead of printing (also show capture info)
        }

        vx.draw_frame();
        *control_flow = ControlFlow::Wait;
    })
}
fn process_input(s: &mut Input, evt: Event<()>) {
    s.prepare_for_next_frame();
    if let Event::WindowEvent { event, .. } = evt {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                s.register_key(&input);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::LineDelta(_, v) = delta {
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
