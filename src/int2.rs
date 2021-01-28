use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Copy, Clone, Debug, PartialEq, Hash)]
#[repr(C)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

#[inline]
pub fn int2(x: i32, y: i32) -> Int2 {
    Int2 { x, y }
}

impl Int2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const UP: Self = Self { x: 0, y: -1 };

    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        int2(x, y)
    }

    #[inline]
    pub fn xy_len(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    #[inline]
    pub fn turn_left(&self) -> Self {
        int2(self.y, -self.x)
    }

    #[inline]
    pub fn turn_right(&self) -> Self {
        int2(-self.y, self.x)
    }

    #[inline]
    pub fn only_x(&self) -> Self {
        int2(self.x, 0)
    }

    #[inline]
    pub fn only_y(&self) -> Self {
        int2(0, self.y)
    }

    #[inline]
    pub fn abs(&self) -> Self {
        int2(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn min(&self, other: Self) -> Self {
        int2(self.x.min(other.x), self.y.min(other.y))
    }

    #[inline]
    pub fn max(&self, other: Self) -> Self {
        int2(self.x.max(other.x), self.y.max(other.y))
    }

    #[inline]
    pub fn sign(&self) -> Self {
        int2(crate::sign_i32(self.x), crate::sign_i32(self.y))
    }

    #[inline]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    #[inline]
    pub fn xy_dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Display for Int2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl From<i32> for Int2 {
    fn from(val: i32) -> Self {
        int2(val, val)
    }
}

impl From<(i32, i32)> for Int2 {
    fn from(val: (i32, i32)) -> Self {
        int2(val.0, val.1)
    }
}

impl From<Int2> for (i32, i32) {
    fn from(val: Int2) -> Self {
        (val.x, val.y)
    }
}

impl Neg for Int2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        int2(-self.x, -self.y)
    }
}

impl Add<Int2> for Int2 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        int2(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Int2> for Int2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        int2(self.x - other.x, self.y - other.y)
    }
}

impl Mul<i32> for Int2 {
    type Output = Self;
    #[inline]
    fn mul(self, n: i32) -> Self {
        int2(self.x * n, self.y * n)
    }
}

impl Mul<Int2> for i32 {
    type Output = Int2;
    #[inline]
    fn mul(self, v: Int2) -> Int2 {
        int2(v.x * self, v.y * self)
    }
}

impl Mul<Int2> for Int2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        int2(self.x * other.x, self.y * other.y)
    }
}

impl Div<i32> for Int2 {
    type Output = Self;
    #[inline]
    fn div(self, n: i32) -> Self {
        int2(self.x / n, self.y / n)
    }
}

impl Div<Int2> for Int2 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        int2(self.x / other.x, self.y / other.y)
    }
}
