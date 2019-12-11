#[derive(Copy, Clone, Debug)]
pub struct Placement {
    pub x: PlacementAxis,
    pub y: PlacementAxis,
    pub x_anchor: Anchor,
    pub y_anchor: Anchor,
}

#[derive(Copy, Clone, Debug)]
pub enum PlacementAxis {
    // Percentage(f32),
    Fixed(f32),
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
    pub fn centered() -> Self {
        Self {
            x: PlacementAxis::Fixed(0.0),
            y: PlacementAxis::Fixed(0.0),
            x_anchor: Anchor::Center,
            y_anchor: Anchor::Center,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
}
impl Axis {
    pub fn other(self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    Min,
    Center,
    Max,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SizeHint {
    /// Size is given externally - by application or rendering.
    /// In this case, size is not touched by `gui`.
    /// For example, a text field's size is determined by the render engine.
    External(f32),
    /// Size is determined by the size of children.
    /// Size will be set to exactly contain children (plus eventual padding).
    Minimize,
    // Percentage(f32, f32),
    // TODO ^ rather try "flex factors" like in Flutter
}
impl Default for SizeHint {
    fn default() -> Self {
        SizeHint::External(10.0)
    }
}
