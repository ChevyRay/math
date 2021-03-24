use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Neg, Sub, Rem, RemAssign, Index};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Default, Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn int3(x: i32, y: i32, z: i32) -> Int3 {
    Int3 { x, y, z }
}

impl Int3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub const RIGHT: Self = Self { x: 1, y: 0, z: 0 };
    pub const LEFT: Self = Self { x: -1, y: 0, z: 0 };
    pub const DOWN: Self = Self { x: 0, y: -1, z: 0 };
    pub const UP: Self = Self { x: 0, y: 1, z: 0 };
    pub const FORWARD: Self = Self { x: 0, y: 0, z: 1 };
    pub const BACK: Self = Self { x: 0, y: 0, z: -1 };

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        int3(x, y, z)
    }

    pub fn xyz_len(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn only_x(&self) -> Self {
        int3(self.x, 0, 0)
    }

    pub fn only_y(&self) -> Self {
        int3(0, self.y, 0)
    }

    pub fn only_z(&self) -> Self {
        int3(0, 0, self.z)
    }

    pub fn abs(&self) -> Self {
        int3(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn min(&self, other: Self) -> Self {
        int3(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn max(&self, other: Self) -> Self {
        int3(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    pub fn sign(&self) -> Self {
        int3(
            crate::sign_i32(self.x),
            crate::sign_i32(self.y),
            crate::sign_i32(self.z),
        )
    }

    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    pub fn xyz_dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl fmt::Display for Int3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl AsRef<[i32]> for Int3 {
    fn as_ref(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const i32, 3) }
    }
}

impl Index<usize> for Int3 {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_ref()[index]
    }
}

impl From<crate::Int2> for Int3 {
    fn from(val: crate::Int2) -> Self {
        int3(val.x, val.y, 0)
    }
}

impl From<i32> for Int3 {
    fn from(val: i32) -> Self {
        int3(val, val, val)
    }
}

impl From<(i32, i32)> for Int3 {
    fn from(val: (i32, i32)) -> Self {
        int3(val.0, val.1, 0)
    }
}

impl From<(i32, i32, i32)> for Int3 {
    fn from(val: (i32, i32, i32)) -> Self {
        int3(val.0, val.1, val.2)
    }
}

impl From<Int3> for (i32, i32, i32) {
    fn from(val: Int3) -> Self {
        (val.x, val.y, val.z)
    }
}

impl Neg for Int3 {
    type Output = Self;
    fn neg(self) -> Self {
        int3(-self.x, -self.y, -self.z)
    }
}

impl Add<Int3> for Int3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        int3(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Int3> for Int3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        int3(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<i32> for Int3 {
    type Output = Self;
    fn mul(self, n: i32) -> Self {
        int3(self.x * n, self.y * n, self.z * n)
    }
}

impl Mul<Int3> for i32 {
    type Output = Int3;
    fn mul(self, v: Int3) -> Int3 {
        int3(v.x * self, v.y * self, v.z * self)
    }
}

impl Mul<Int3> for Int3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        int3(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<i32> for Int3 {
    type Output = Self;
    fn div(self, n: i32) -> Self {
        int3(self.x / n, self.y / n, self.z / n)
    }
}

impl Div<Int3> for Int3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        int3(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Rem<Int3> for Int3 {
    type Output = Int3;
    fn rem(self, rhs: Int3) -> Self::Output {
        int3(self.x % rhs.x, self.y % rhs.y, self.z % rhs.z)
    }
}

impl RemAssign<Int3> for Int3 {
    fn rem_assign(&mut self, rhs: Int3) {
        *self = self.rem(rhs);
    }
}

impl Rem<i32> for Int3 {
    type Output = Int3;
    fn rem(self, rhs: i32) -> Self::Output {
        int3(self.x % rhs, self.y % rhs, self.z % rhs)
    }
}

impl RemAssign<i32> for Int3 {
    fn rem_assign(&mut self, rhs: i32) {
        *self = self.rem(rhs);
    }
}
