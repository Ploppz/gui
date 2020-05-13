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
    type Calculator: TextCalculator;
    fn window_size(&self, ctx: &mut Self::Context) -> Vec2;
    fn transform_mouse(&self, m: Vec2, ctx: &mut Self::Context) -> Vec2;
    fn update(
        &mut self,
        gui: &Gui<Self>,
        events: &[Event],
        log: Logger,
        ctx: &mut Self::Context,
    ) -> Vec<WidgetOp>;

    fn text_calc(&self, layer: u32, ctx: &mut Self::Context) -> Self::Calculator;
}

pub trait TextCalculator: 'static + std::fmt::Debug {
    fn text_size(&mut self, text: &str) -> Vec2;
}

/*
pub trait ContextFreeGuiDrawer {
    fn text_size(&mut self, text: &str) -> Vec2;
}

/// Contains references to the GuiDrawer as well as its context, and thus provides a `GuiDrawer`
/// interface where `type Context = ()` (kinda erases the context from the interface and moves it
/// internally).
/// This struct is used primarily as a parameter to functions of `trait Interactive`.
pub struct GuiDrawerWithContext<'a, 'b, D: GuiDrawer> {
    drawer: &'a D,
    ctx: &'b mut D::Context,
}
impl<'a, 'b, D: GuiDrawer> ContextFreeGuiDrawer for GuiDrawerWithContext<'a, 'b, D> {
    /// Determine size of rendered text without rendering it.
    fn text_size(&mut self, text: &str) -> Vec2 {
        self.drawer.text_size(text, self.ctx)
    }
}
*/

/// Text calculator used with `NoDrawer` - simple 10.0 times the number of characters
#[derive(Debug)]
pub struct NoTextCalculator;
impl TextCalculator for NoTextCalculator {
    fn text_size(&mut self, text: &str) -> Vec2 {
        Vec2::new(10.0 * text.len() as f32, 10.0)
    }
}
/// Empty implementor of GuiDrawer, for a headless Gui.
/// Note: Text size and window size are always zero.
pub struct NoDrawer;
impl GuiDrawer for NoDrawer {
    type Context = ();
    type Calculator = NoTextCalculator;
    fn window_size(&self, _ctx: &mut Self::Context) -> Vec2 {
        Vec2::zero()
    }
    fn transform_mouse(&self, m: Vec2, _ctx: &mut Self::Context) -> Vec2 {
        m
    }
    fn update(
        &mut self,
        _gui: &Gui<Self>,
        _events: &[Event],
        _log: Logger,
        _ctx: &mut Self::Context,
    ) -> Vec<WidgetOp> {
        Vec::new()
    }
    fn text_calc(&self, _layer: u32, _ctx: &mut Self::Context) -> Self::Calculator {
        NoTextCalculator
    }
}

pub enum WidgetOp {}
