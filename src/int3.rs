use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Copy, Clone, Debug, PartialEq, Hash)]
#[repr(C)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[inline]
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

    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        int3(x, y, z)
    }

    #[inline]
    pub fn xyz_len(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    #[inline]
    pub fn only_x(&self) -> Self {
        int3(self.x, 0, 0)
    }

    #[inline]
    pub fn only_y(&self) -> Self {
        int3(0, self.y, 0)
    }

    #[inline]
    pub fn only_z(&self) -> Self {
        int3(0, 0, self.z)
    }

    #[inline]
    pub fn abs(&self) -> Self {
        int3(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline]
    pub fn min(&self, other: Self) -> Self {
        int3(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    #[inline]
    pub fn max(&self, other: Self) -> Self {
        int3(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    #[inline]
    pub fn sign(&self) -> Self {
        int3(
            crate::sign_i32(self.x),
            crate::sign_i32(self.y),
            crate::sign_i32(self.z),
        )
    }

    #[inline]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    #[inline]
    pub fn xyz_dist(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl fmt::Display for Int3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
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
    #[inline]
    fn neg(self) -> Self {
        int3(-self.x, -self.y, -self.z)
    }
}

impl Add<Int3> for Int3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        int3(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Int3> for Int3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        int3(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<i32> for Int3 {
    type Output = Self;
    #[inline]
    fn mul(self, n: i32) -> Self {
        int3(self.x * n, self.y * n, self.z * n)
    }
}

impl Mul<Int3> for i32 {
    type Output = Int3;
    #[inline]
    fn mul(self, v: Int3) -> Int3 {
        int3(v.x * self, v.y * self, v.z * self)
    }
}

impl Mul<Int3> for Int3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        int3(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<i32> for Int3 {
    type Output = Self;
    #[inline]
    fn div(self, n: i32) -> Self {
        int3(self.x / n, self.y / n, self.z / n)
    }
}

impl Div<Int3> for Int3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        int3(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
