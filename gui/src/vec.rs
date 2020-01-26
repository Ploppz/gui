use crate::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect<T = f32> {
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}
impl Rect {
    pub fn zero() -> Self {
        Rect {
            min: Vec2::zero(),
            max: Vec2::zero(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}
impl Vec2 {
    pub fn one() -> Self {
        Vec2 { x: 1.0, y: 1.0 }
    }
    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }
    pub fn infinity() -> Self {
        Vec2 {
            x: std::f32::INFINITY,
            y: std::f32::INFINITY,
        }
    }
    pub fn length(self) -> f32 {
        self.y.mul_add(self.y, self.x * self.x).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.y.mul_add(self.y, self.x * self.x)
    }

    pub fn normalize(self) -> Vec2 {
        let len = self.length();
        Vec2::<f32>::new(self.x / len, self.y / len)
    }

    pub fn scale(self, x: f32, y: f32) -> Vec2 {
        Vec2::<f32>::new(self.x * x, self.y * y)
    }

    pub fn scale_uni(self, s: f32) -> Vec2 {
        Vec2::<f32>::new(self.x * s, self.y * s)
    }

    pub fn dot(a: Vec2, b: Vec2) -> f32 {
        a.y.mul_add(b.y, a.x * b.x)
    }

    pub fn cross(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn is_infinite(self) -> bool {
        self.x.is_infinite() || self.y.is_infinite()
    }

    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn clamp(self, limit: Vec2) -> Vec2 {
        Self {
            x: if self.x > limit.x {
                limit.x
            } else if self.x < -limit.x {
                -limit.x
            } else {
                self.x
            },
            y: if self.y > limit.y {
                limit.y
            } else if self.y < -limit.y {
                -limit.y
            } else {
                self.y
            },
        }
    }

    /// Rotate the vector around the origin by a given angle
    pub fn rotate(self, angle: f32) -> Vec2 {
        let angle = self.angle() + angle;
        let x = angle.cos() * self.length();
        let y = angle.sin() * self.length();
        Vec2 { x, y }
    }
}

// ---

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec2<T>;
    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul for Vec2<T>
where
    T: Mul<Output = T>,
{
    type Output = Vec2<T>;
    fn mul(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Vec2<T>;
    fn neg(self) -> Vec2<T> {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Div for Vec2<T>
where
    T: Div<Output = T>,
{
    type Output = Vec2<T>;
    fn div(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// ---

impl<T> AddAssign for Vec2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Vec2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Vec2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> MulAssign for Vec2<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, other: Vec2<T>) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl<T> DivAssign for Vec2<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, other: Vec2<T>) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

// ---

impl<T> Mul<f32> for Vec2<T>
where
    T: Mul<f32, Output = f32>,
{
    type Output = Vec2;
    fn mul(self, n: f32) -> Vec2 {
        Vec2 {
            x: self.x * n,
            y: self.y * n,
        }
    }
}

impl<T> Div<f32> for Vec2<T>
where
    T: Div<f32, Output = f32>,
{
    type Output = Vec2;
    fn div(self, n: f32) -> Vec2 {
        Vec2 {
            x: self.x / n,
            y: self.y / n,
        }
    }
}

// ---

impl From<(f32, f32)> for Vec2<f32> {
    fn from(point: (f32, f32)) -> Self {
        Vec2 {
            x: point.0,
            y: point.1,
        }
    }
}

impl From<Vec2<f32>> for (f32, f32) {
    fn from(point: Vec2<f32>) -> Self {
        (point.x, point.y)
    }
}

impl<T> std::ops::Index<Axis> for Vec2<T> {
    type Output = T;
    fn index(&self, idx: Axis) -> &T {
        match idx {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        }
    }
}

impl<T> std::ops::IndexMut<Axis> for Vec2<T> {
    fn index_mut(&mut self, idx: Axis) -> &mut T {
        match idx {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }
}

use std::fmt::{Display, Error, Formatter};
impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}
