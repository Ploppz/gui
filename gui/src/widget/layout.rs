use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct WidgetConfig {
    /// Optional positioning; makes this widget not participate in its siblings' layout.
    /// If `Some`, the layer of this widget will be incremented relative to its parent.
    /// This means that it will show on a layer above.
    pub place: Option<Placement>,
    /// The axis along which to stack children
    pub layout_direction: Axis,
    /// Anchor along main axis
    pub layout_main_align: Anchor,
    /// If true, children are stacked in the cross axis when the main axis fills up.
    pub layout_wrap: bool,
    /// Alignment of children along the cross axis (the axis which is not the direction).
    pub layout_cross_align: Anchor,
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
            layout_main_align: Anchor::Min,
            layout_cross_align: Anchor::Min,
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
        layout_cross_align: Anchor,
        layout_main_margin: f32,
    ) -> Self {
        self.layout_direction = layout_direction;
        self.layout_wrap = layout_wrap;
        self.layout_cross_align = layout_cross_align;
        self.layout_main_margin = layout_main_margin;
        self
    }
    pub fn set_layout(
        &mut self,
        layout_direction: Axis,
        layout_wrap: bool,
        layout_cross_align: Anchor,
        layout_main_margin: f32,
    ) -> &mut Self {
        self.layout_direction = layout_direction;
        self.layout_wrap = layout_wrap;
        self.layout_cross_align = layout_cross_align;
        self.layout_main_margin = layout_main_margin;
        self
    }
    pub fn layout_direction(mut self, value: Axis) -> Self {
        self.layout_direction = value;
        self
    }
    pub fn layout_cross_align(mut self, value: Anchor) -> Self {
        self.layout_cross_align = value;
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
    pub fn set_size_hint(&mut self, x: SizeHint, y: SizeHint) -> &mut Self {
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
    pub fn set_size(&mut self, w: f32, h: f32) -> &mut Self {
        self.size_hint.x = SizeHint::External(w);
        self.size_hint.y = SizeHint::External(h);
        self
    }
    pub fn set_width(&mut self, w: f32) -> &mut Self {
        self.size_hint.x = SizeHint::External(w);
        self
    }
    pub fn set_height(&mut self, h: f32) -> &mut Self {
        self.size_hint.y = SizeHint::External(h);
        self
    }
    pub fn padding(mut self, top: f32, bot: f32, left: f32, right: f32) -> Self {
        self.padding.min = Vec2::new(left, top);
        self.padding.max = Vec2::new(right, bot);
        self
    }
    pub fn set_padding(&mut self, top: f32, bot: f32, left: f32, right: f32) -> &mut Self {
        self.padding.min = Vec2::new(left, top);
        self.padding.max = Vec2::new(right, bot);
        self
    }
}

impl Widget {
    /// Recursively updates the position of children, and updates size of `self` if applicable.
    /// Additionally, updates sizes of text fields using `GuiDrawer`
    pub(crate) fn layout_alg(&mut self) {
        for child in self.children.values_mut() {
            // Recurse
            child.layout_alg();
        }

        // println!("Positioning Parent [{}]", self.id);
        if self.config.layout_wrap {
            unimplemented!()
        }
        let layout_cross_align = self.config.layout_cross_align;
        let layout_main_margin = self.config.layout_main_margin;
        let padding_min = self.config.padding.min;

        let (main_axis, cross_axis) = (
            self.config.layout_direction,
            self.config.layout_direction.other(),
        );

        //
        // Figure out size first, based on children's sizes
        //

        let mut main_size = self.config.padding.min[main_axis] + self.config.padding.max[main_axis];
        let mut cross_size = 0.0;

        for child in self.children.values() {
            if let None = child.config.place {
                main_size += child.size[main_axis] + layout_main_margin;
                if child.size[cross_axis] > cross_size {
                    cross_size = child.size[cross_axis]
                }
            }
        }
        // because it should only be _between_ children - not after the last one
        main_size -= layout_main_margin;

        let mut new_size = self.size;

        let intrinsic_size = self.determine_size(&mut *self.gui.borrow_mut().text_calc);
        new_size[main_axis] = match self.config.size_hint[main_axis] {
            SizeHint::Minimize => main_size,
            SizeHint::External(s) => s,
            SizeHint::Intrinsic => intrinsic_size.expect("no intrinsic size").x,
        };
        new_size[cross_axis] = match self.config.size_hint[cross_axis] {
            SizeHint::Minimize => {
                cross_size
                    + self.config.padding.min[cross_axis]
                    + self.config.padding.max[cross_axis]
            }
            SizeHint::External(s) => s,
            SizeHint::Intrinsic => intrinsic_size.expect("no intrinsic size").y,
        };

        if new_size != self.size {
            self.size = new_size;
            self.gui
                .borrow_mut()
                .push_event(Event::change(self.id, Widget::size));
        }

        //
        // Update positions of all children
        //
        let size = self.size;
        // Keeps track of position along main axis
        // TODO: needs special inital value sometimes
        let mut main_progress = self.config.padding.min[main_axis];

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
                child_relative_pos[main_axis] = main_progress;
                child_relative_pos[cross_axis] = match layout_cross_align {
                    Anchor::Min => padding_min[cross_axis],
                    Anchor::Center => (size[cross_axis] - child.size[cross_axis]) / 2.0,
                    Anchor::Max => unimplemented!(),
                };
                main_progress += child.size[main_axis] + layout_main_margin;
            };

            // println!("Positioning Child [{}] relative_pos={:?}", child.id, child_relative_pos);
            child.rel_pos = child_relative_pos;
        }
    }
}
