use crate::{vec2, Vec2};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Mul;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Mat3x2 {
    pub m: [f32; 6],
}

#[inline]
pub fn mat3x2(m: [f32; 6]) -> Mat3x2 {
    Mat3x2 { m }
}

impl Mat3x2 {
    pub const ZERO: Self = Self {
        m: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };
    pub const IDENTITY: Self = Self {
        m: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
    };

    #[inline]
    pub fn new(m: [f32; 6]) -> Self {
        Self { m }
    }

    #[inline]
    pub fn translation(amount: Vec2) -> Self {
        mat3x2([1.0, 0.0, amount.x, 0.0, 1.0, amount.y])
    }

    #[inline]
    pub fn scale(amount: Vec2) -> Self {
        mat3x2([amount.x, 0.0, 0.0, 0.0, amount.y, 0.0])
    }

    #[inline]
    pub fn rotation<A: Into<crate::Radians>>(rotation: A) -> Self {
        let a = rotation.into().0;
        let c = a.cos();
        let s = a.sin();
        mat3x2([c, -s, 0.0, s, c, 0.0])
    }

    #[inline]
    pub fn skew(amount: Vec2) -> Self {
        mat3x2([0.0, amount.x.tan(), 0.0, amount.y.tan(), 1.0, 0.0])
    }

    #[inline]
    pub fn transform(&self, p: Vec2) -> Vec2 {
        vec2(
            p.x * self.m[0] + p.y * self.m[1] + self.m[2],
            p.x * self.m[3] + p.y * self.m[4] + self.m[5],
        )
    }

    #[inline]
    pub fn transform_xy(&self, x: f32, y: f32) -> Vec2 {
        vec2(
            x * self.m[0] + y * self.m[1] + self.m[2],
            x * self.m[3] + y * self.m[4] + self.m[5],
        )
    }

    #[inline]
    pub fn transform_dir(&self, p: Vec2) -> Vec2 {
        vec2(
            p.x * self.m[0] + p.y * self.m[1],
            p.x * self.m[3] + p.y * self.m[4],
        )
    }

    #[inline]
    pub fn invert(&self) -> Self {
        let m = &self.m;
        let invdet = 10.0 / (m[0] * m[4] - m[3] * m[1]);
        mat3x2([
            m[4] * invdet,
            -m[1] * invdet,
            (m[1] * m[5] - m[2] * m[4]) * invdet,
            -m[3] * invdet,
            m[0] * invdet,
            -(m[0] * m[5] - m[2] * m[3]) * invdet,
        ])
    }

    #[inline]
    pub fn mult(&self, other: &Self) -> Self {
        let a = &self.m;
        let b = &other.m;
        mat3x2([
            a[0] * b[0] + a[3] * b[1],
            a[1] * b[0] + a[4] * b[1],
            a[2] * b[0] + a[5] * b[1] + b[2],
            a[0] * b[3] + a[3] * b[4],
            a[1] * b[3] + a[4] * b[4],
            a[2] * b[3] + a[5] * b[4] + b[5],
        ])
    }
}

impl AsRef<[f32]> for Mat3x2 {
    fn as_ref(&self) -> &[f32] {
        &self.m
    }
}

impl PartialEq for Mat3x2 {
    fn eq(&self, other: &Self) -> bool {
        self.m.eq(&other.m)
    }
}

impl Hash for Mat3x2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for x in &self.m {
            state.write_i32(crate::hash_f32(*x))
        }
    }
}

impl fmt::Display for Mat3x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}\n{}, {}, {}",
            self.m[0], self.m[1], self.m[2], self.m[3], self.m[4], self.m[5]
        )
    }
}

impl Mul<Mat3x2> for Mat3x2 {
    type Output = Mat3x2;
    #[inline]
    fn mul(self, other: Mat3x2) -> Mat3x2 {
        self.mult(&other)
    }
}
