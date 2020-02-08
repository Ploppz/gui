use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct WidgetConfig {
    /// Optional positioning; makes this widget not participate in its siblings' layout
    pub place: Option<Placement>,
    /// The axis along which to stack children
    pub layout_direction: Axis,
    /// If true, children are stacked in the cross axis when the main axis fills up.
    pub layout_wrap: bool,
    /// Alignment of children along the cross axis (the axis which is not the direction).
    pub layout_align: Anchor,
    /// Space between widgets in the main axis.
    /// TODO: should maybe be a "justify" enum where you can choose to space them evenly etc
    pub layout_main_margin: f32,

    pub padding: Rect,

    // size hints
    pub size_hint: Vec2<SizeHint>,
}
impl Default for WidgetConfig {
    fn default() -> Self {
        WidgetConfig {
            place: None,
            layout_direction: Axis::X,
            layout_wrap: false,
            layout_align: Anchor::Min,
            layout_main_margin: 0.0,

            padding: Rect::zero(),

            size_hint: Vec2::default(),
        }
    }
}
impl WidgetConfig {
    pub fn layout(
        mut self,
        layout_direction: Axis,
        layout_wrap: bool,
        layout_align: Anchor,
        _layout_main_margin: f32,
    ) -> Self {
        self.layout_direction = layout_direction;
        self.layout_wrap = layout_wrap;
        self.layout_align = layout_align;
        self.layout_main_margin = self.layout_main_margin;
        self
    }
    pub fn layout_direction(mut self, value: Axis) -> Self {
        self.layout_direction = value;
        self
    }
    pub fn layout_align(mut self, value: Anchor) -> Self {
        self.layout_align = value;
        self
    }
    pub fn placement(mut self, place: Placement) -> Self {
        self.place = Some(place);
        self
    }
    pub fn set_placement(&mut self, place: Placement) -> &mut Self {
        self.place = Some(place);
        self
    }
    pub fn size_hint(mut self, x: SizeHint, y: SizeHint) -> Self {
        self.size_hint = Vec2::new(x, y);
        self
    }
    /// Fixed width
    pub fn width(mut self, w: f32) -> Self {
        self.size_hint.x = SizeHint::External(w);
        self
    }
    /// Fixed height
    pub fn height(mut self, h: f32) -> Self {
        self.size_hint.y = SizeHint::External(h);
        self
    }
    pub fn set_size(&mut self, w: f32, h: f32) {
        self.size_hint.x = SizeHint::External(w);
        self.size_hint.y = SizeHint::External(h);
    }
    pub fn set_width(&mut self, w: f32) {
        self.size_hint.x = SizeHint::External(w);
    }
    pub fn set_height(&mut self, h: f32) {
        self.size_hint.y = SizeHint::External(h);
    }
    pub fn padding(mut self, top: f32, bot: f32, left: f32, right: f32) -> Self {
        self.padding.min = Vec2::new(left, top);
        self.padding.max = Vec2::new(right, bot);
        self
    }
}
