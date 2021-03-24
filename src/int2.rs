use std::fmt;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Rem, RemAssign, Index};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Default, Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

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

    pub fn new(x: i32, y: i32) -> Self {
        int2(x, y)
    }

    pub fn xy_len(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn turn_left(&self) -> Self {
        int2(self.y, -self.x)
    }

    pub fn turn_right(&self) -> Self {
        int2(-self.y, self.x)
    }

    pub fn only_x(&self) -> Self {
        int2(self.x, 0)
    }

    pub fn only_y(&self) -> Self {
        int2(0, self.y)
    }

    pub fn abs(&self) -> Self {
        int2(self.x.abs(), self.y.abs())
    }

    pub fn min(&self, other: Self) -> Self {
        int2(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(&self, other: Self) -> Self {
        int2(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn sign(&self) -> Self {
        int2(crate::sign_i32(self.x), crate::sign_i32(self.y))
    }

    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    pub fn xy_dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Display for Int2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl AsRef<[i32]> for Int2 {
    fn as_ref(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const i32, 2) }
    }
}

impl Index<usize> for Int2 {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
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
    fn neg(self) -> Self {
        int2(-self.x, -self.y)
    }
}

impl Add<Int2> for Int2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        int2(self.x + other.x, self.y + other.y)
    }
}
impl AddAssign<Int2> for Int2 {
    fn add_assign(&mut self, other: Int2) {
        *self = self.add(other);
    }
}

impl Sub<Int2> for Int2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        int2(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign<Int2> for Int2 {
    fn sub_assign(&mut self, other: Int2) {
        *self = self.sub(other);
    }
}

impl Mul<i32> for Int2 {
    type Output = Self;
    fn mul(self, n: i32) -> Self {
        int2(self.x * n, self.y * n)
    }
}

impl Mul<Int2> for i32 {
    type Output = Int2;
    fn mul(self, v: Int2) -> Int2 {
        int2(v.x * self, v.y * self)
    }
}

impl MulAssign<i32> for Int2 {
    fn mul_assign(&mut self, other: i32) {
        *self = self.mul(other);
    }
}

impl Mul<Int2> for Int2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        int2(self.x * other.x, self.y * other.y)
    }
}

impl MulAssign<Int2> for Int2 {
    fn mul_assign(&mut self, other: Int2) {
        *self = self.mul(other);
    }
}

impl Div<i32> for Int2 {
    type Output = Self;
    fn div(self, n: i32) -> Self {
        int2(self.x / n, self.y / n)
    }
}

impl DivAssign<i32> for Int2 {
    fn div_assign(&mut self, other: i32) {
        *self = self.div(other);
    }
}

impl Div<Int2> for Int2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        int2(self.x / other.x, self.y / other.y)
    }
}

impl DivAssign<Int2> for Int2 {
    fn div_assign(&mut self, other: Int2) {
        *self = self.div(other);
    }
}

impl Rem<Int2> for Int2 {
    type Output = Int2;
    fn rem(self, rhs: Int2) -> Self::Output {
        int2(self.x % rhs.x, self.y % rhs.y)
    }
}

impl RemAssign<Int2> for Int2 {
    fn rem_assign(&mut self, rhs: Int2) {
        *self = self.rem(rhs);
    }
}

impl Rem<i32> for Int2 {
    type Output = Int2;
    fn rem(self, rhs: i32) -> Self::Output {
        int2(self.x % rhs, self.y % rhs)
    }
}

impl RemAssign<i32> for Int2 {
    fn rem_assign(&mut self, rhs: i32) {
        *self = self.rem(rhs);
    }
}