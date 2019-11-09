
#[derive(Copy, Clone, Debug)]
pub enum Placement {
    Percentage (f32, f32),
    Fixed (Position),
    Float (Axis, Anchor),
}
impl Placement {
    pub fn fixed(x: f32, y: f32) -> Self {
        Self::Fixed (Position {x, y, x_anchor: Anchor::Min, y_anchor: Anchor::Min})
    }
    pub fn x_anchor(self, a: Anchor) -> Self {
        if let Self::Fixed(Position {mut x_anchor, ..}) =  self {
            x_anchor = a
        } else {
            panic!("x_anchor should only be used on `Placement::Fixed`");
        }
        self
    }
    pub fn y_anchor(self, a: Anchor) -> Self {
        if let Self::Fixed(Position {mut y_anchor, ..}) =  self {
            y_anchor = a
        } else {
            panic!("y_anchor should only be used on `Placement::Fixed`");
        }
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: f32,
    y: f32,
    x_anchor: Anchor,
    y_anchor: Anchor,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WidgetEvent {
    Press,
    Release,
    Hover,
    Unhover,
    /// Change to any internal state
    Change,
    // TODO: perhaps something to notify that position has changed
}

#[derive(Copy, Clone, Debug)]
pub enum SizeHint {
    None,
    /// Minimize, with internal padding
    Minimize {top: f32, bot: f32, left: f32, right: f32},
    Percentage (f32, f32),
}