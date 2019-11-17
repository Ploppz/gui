#[derive(Copy, Clone, Debug)]
pub enum Placement {
    Percentage(f32, f32),
    Fixed(Position),
    Float(Axis, Anchor),
}
// each axis has an anchor
// each axis can be Float, Fixed(f32), Percentage(f32)

impl Placement {
    pub fn fixed(x: f32, y: f32) -> Self {
        Self::Fixed(Position {
            x,
            y,
            x_anchor: Anchor::Min,
            y_anchor: Anchor::Min,
        })
    }
    pub fn x_anchor(self, a: Anchor) -> Self {
        if let Self::Fixed(Position { mut x_anchor, .. }) = self {
            x_anchor = a
        } else {
            panic!("x_anchor should only be used on `Placement::Fixed`");
        }
        self
    }
    pub fn y_anchor(self, a: Anchor) -> Self {
        if let Self::Fixed(Position { mut y_anchor, .. }) = self {
            y_anchor = a
        } else {
            panic!("y_anchor should only be used on `Placement::Fixed`");
        }
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub x_anchor: Anchor,
    pub y_anchor: Anchor,
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
