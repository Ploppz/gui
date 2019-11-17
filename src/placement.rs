#[derive(Copy, Clone, Debug)]
pub struct Placement {
    pub x: PlacementAxis,
    pub y: PlacementAxis,
    pub x_anchor: Anchor,
    pub y_anchor: Anchor,
}

#[derive(Copy, Clone, Debug)]
pub enum PlacementAxis {
    Percentage(f32),
    Fixed(f32),
    Float,
}
// each axis has an anchor
// each axis can be Float, Fixed(f32), Percentage(f32)

impl Placement {
    pub fn fixed(x: f32, y: f32) -> Self {
        Self {
            x: PlacementAxis::Fixed(x),
            y: PlacementAxis::Fixed(y),
            x_anchor: Anchor::Min,
            y_anchor: Anchor::Min,
        }
    }
    pub fn x_anchor(mut self, a: Anchor) -> Self {
        self.x_anchor = a;
        self
    }
    pub fn y_anchor(mut self, a: Anchor) -> Self {
        self.y_anchor = a;
        self
    }
    pub fn anchor(mut self, a: Anchor) -> Self {
        self.x_anchor = a;
        self.y_anchor = a;
        self
    }
    pub fn float() -> Self {
        Placement {
            x: PlacementAxis::Float,
            y: PlacementAxis::Float,
            x_anchor: Anchor::Min,
            y_anchor: Anchor::Min,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    Min,
    Max,
    Center,
}

#[derive(Copy, Clone, Debug)]
pub enum SizeHint {
    None,
    /// Minimize, with internal padding
    Minimize {
        top: f32,
        bot: f32,
        left: f32,
        right: f32,
    },
    Percentage(f32, f32),
}
