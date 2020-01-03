//! Trait definition for interoperability between Gui and a renderer/drawer.
//!
//! It is encouraged to implement custom drawers for applications that need their
//! own GUI look (such as games).
//!
//! The following modules provide default drawers for when you just want to get something working
//! (before starting thinking about the look). Each module supports one specific rendering backend.
//!
//! (Currently there are none)

use crate::*;
use slog::Logger;

pub trait GuiDrawer: Sized {
    type Context;
    fn window_size(&self, ctx: &mut Self::Context) -> (f32, f32);
    fn transform_mouse(&self, m: (f32, f32), ctx: &mut Self::Context) -> (f32, f32);
    fn update(
        &self,
        gui: &Gui<Self>,
        events: &[Event],
        log: Logger,
        ctx: &mut Self::Context,
    ) -> Vec<WidgetOp>;
    /// Determine size of rendered text without rendering it.
    fn text_size(&self, text: &str, ctx: &mut Self::Context) -> (f32, f32);
}

/// Empty implementor of GuiDrawer, for a headless Gui.
/// Note: Text size and window size are always zero.
pub struct NoDrawer;
impl GuiDrawer for NoDrawer {
    type Context = ();
    fn window_size(&self, _ctx: &mut Self::Context) -> (f32, f32) {
        (0.0, 0.0)
    }
    fn transform_mouse(&self, m: (f32, f32), _ctx: &mut Self::Context) -> (f32, f32) {
        m
    }
    fn update(
        &self,
        _gui: &Gui<Self>,
        _events: &[Event],
        _log: Logger,
        _ctx: &mut Self::Context,
    ) -> Vec<WidgetOp> {
        Vec::new()
    }
    fn text_size(&self, text: &str, _ctx: &mut Self::Context) -> (f32, f32) {
        // NOTE:
        (10.0 * text.len() as f32, 10.0)
    }
}

pub enum WidgetOp {
    Resize { id: Id, size: (f32, f32) },
}
