//! Trait definition for interoperability between Gui and a renderer/drawer.
use crate::*;

pub trait GuiDrawer: Sized {
    type Context;
    fn window_size(&self, ctx: &mut Self::Context) -> (f32, f32);
    fn transform_mouse(&self, m: (f32, f32), ctx: &mut Self::Context) -> (f32, f32);
    fn update(&self,
        gui: &Gui<Self>,
        events: &[(String, WidgetEvent)],
        ctx: &mut Self::Context) -> Vec<WidgetOp>;
}

/// Empty implementor of GuiDrawer, for a headless Gui.
pub struct NoDrawer;
impl GuiDrawer for NoDrawer {
    type Context = ();
    fn window_size(&self, _ctx: &mut Self::Context) -> (f32, f32) {
        (0.0, 0.0)
    }
    fn transform_mouse(&self, m: (f32, f32), _ctx: &mut Self::Context) -> (f32, f32) {
        m
    }
    fn update(&self,
        _gui: &Gui<Self>,
        _events: &[(String, WidgetEvent)],
        _ctx: &mut Self::Context) -> Vec<WidgetOp> {
        Vec::new()
    }
}

pub enum WidgetOp {
    Resize {id: String, size: (f32, f32)}
}

