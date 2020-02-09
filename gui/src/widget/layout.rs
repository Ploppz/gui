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

impl Widget {
    /// Recursively updates the position of children, and updates size of `self` if applicable.
    /// Additionally, updates sizes of text fields using `GuiDrawer`
    pub(crate) fn layout_alg<D: GuiDrawer>(
        &mut self,
        gui: GuiShared,
        drawer: &D,
        ctx: &mut D::Context,
    ) {
        for child in self.children.values_mut() {
            // Recurse
            child.layout_alg(gui.clone(), drawer, ctx);
        }

        // println!("Positioning Parent [{}]", self.id);
        if self.config.layout_wrap {
            unimplemented!()
        }
        let size = self.size;
        let layout_align = self.config.layout_align;
        let layout_main_margin = self.config.layout_main_margin;
        let padding_min = self.config.padding.min;

        let (main_axis, cross_axis) = (
            self.config.layout_direction,
            self.config.layout_direction.other(),
        );

        let mut layout_progress = self.config.padding.min[main_axis];
        // max width/height along cross axis
        let mut cross_size = 0.0;

        for child in self.children.values_mut() {
            let mut child_relative_pos = Vec2::zero();
            if let Some(place) = child.config.place {
                // Child does not participate in layout
                child_relative_pos.x = match place.x {
                    PlacementAxis::Fixed(x) => match place.x_anchor {
                        Anchor::Min => x,
                        Anchor::Center => (size.x - child.size.x) / 2.0 + x,
                        Anchor::Max => size.x - child.size.x - x,
                    },
                };
                child_relative_pos.y = match place.y {
                    PlacementAxis::Fixed(y) => match place.y_anchor {
                        Anchor::Min => y,
                        Anchor::Center => (size.y - child.size.y) / 2.0 + y,
                        Anchor::Max => size.y - child.size.y - y,
                    },
                };
            } else {
                // Layout algorithm
                child_relative_pos[main_axis] = layout_progress;
                layout_progress += child.size[main_axis] + layout_main_margin;
                child_relative_pos[cross_axis] = match layout_align {
                    Anchor::Min => padding_min[cross_axis],
                    Anchor::Center => (size[cross_axis] - child.size[cross_axis]) / 2.0,
                    Anchor::Max => unimplemented!(),
                };
                if child.size[cross_axis] > cross_size {
                    cross_size = child.size[cross_axis]
                }
            };

            // println!("Positioning Child [{}] relative_pos={:?}", child.id, child_relative_pos);
            child.rel_pos = child_relative_pos;
        }
        // because it should only be _between_ children - not after the last one
        layout_progress -= layout_main_margin;
        layout_progress += self.config.padding.max[main_axis];

        let mut new_size = self.size;
        // println!("[positioning {}] pre size {:?}", self.id, new_size);

        if let Some(intrinsic_size) = self.determine_size(&mut drawer.context_free(ctx)) {
            new_size = intrinsic_size;
        } else {
            match self.config.size_hint[main_axis] {
                SizeHint::Minimize => new_size[main_axis] = layout_progress,
                SizeHint::External(s) => new_size[main_axis] = s,
            }
            match self.config.size_hint[cross_axis] {
                SizeHint::Minimize => {
                    new_size[cross_axis] = cross_size
                        + self.config.padding.min[cross_axis]
                        + self.config.padding.max[cross_axis]
                }
                SizeHint::External(s) => new_size[cross_axis] = s,
            }
        }

        if new_size != self.size {
            self.size = new_size;
            gui.borrow_mut()
                .push_event(Event::change(self.id, Widget::size));
        }
    }
}
