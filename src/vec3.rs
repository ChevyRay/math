use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::{Vec2, Vec4};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A three-dimensional floating point vector.
#[derive(Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Easy constructor.
#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[allow(clippy::len_without_is_empty)]
impl Vec3 {
    /// (0.0, 0.0, 0.0)
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    /// (1.0, 1.0, 1.0)
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    /// (1.0, 0.0, 0.0)
    pub const RIGHT: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    /// (-1.0, 0.0, 0.0)
    pub const LEFT: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    /// (0.0, -1.0, 0.0)
    pub const DOWN: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    /// (0.0, 1.0, 0.0)
    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    /// (0.0, 0.0, 1.0)
    pub const FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    /// (0.0, 0.0, -1.0)
    pub const BACK: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    /// Create a new vector.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3(x, y, z)
    }

    /// Extend this vector with a w-axis.
    #[inline]
    pub fn extend(&self, w: f32) -> Vec4 {
        crate::vec4(self.x, self.y, self.z, w)
    }

    /// Compose a new vector from a `Vec2` and the provided `z` axis.
    #[inline]
    pub fn from(val: Vec2, z: f32) -> Self {
        vec3(val.x, val.y, z)
    }

    /// The length of the vector, squared.
    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// The euclidean length of the vector.
    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    /// Normalize the vector.
    #[inline]
    pub fn norm(&self) -> Self {
        let len = self.len();
        vec3(self.x / len, self.y / len, self.z / len)
    }

    /// Zero the vector's y and z axes.
    #[inline]
    pub fn only_x(&self) -> Self {
        vec3(self.x, 0.0, 0.0)
    }

    /// Zero the vector's x and z axes.
    #[inline]
    pub fn only_y(&self) -> Self {
        vec3(0.0, self.y, 0.0)
    }

    /// Zero the vector's x and y axes.
    #[inline]
    pub fn only_z(&self) -> Self {
        vec3(0.0, 0.0, self.z)
    }

    /// Absolute the vector's components.
    #[inline]
    pub fn abs(&self) -> Self {
        vec3(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Round the vector's components down.
    #[inline]
    pub fn floor(&self) -> Self {
        vec3(self.x.floor(), self.y.floor(), self.z.floor())
    }

    /// Round the vector's components up.
    #[inline]
    pub fn ceil(&self) -> Self {
        vec3(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    /// Round the vector's components.
    #[inline]
    pub fn round(&self) -> Self {
        vec3(self.x.round(), self.y.round(), self.z.round())
    }

    /// Return the minimum of the vector's components.
    #[inline]
    pub fn min(&self, other: Self) -> Self {
        vec3(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Return the maximum of the vector's components.
    #[inline]
    pub fn max(&self, other: Self) -> Self {
        vec3(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Return the sign of the vector's components.
    #[inline]
    pub fn sign(&self) -> Self {
        vec3(
            crate::sign(self.x),
            crate::sign(self.y),
            crate::sign(self.z),
        )
    }

    /// Return a vector with its components clamped in the provided range.
    #[inline]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    /// Return the dot product of two vectors.
    #[inline]
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Return the cross product of two vectors.
    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        vec3(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Get the square distance between two vectors.
    #[inline]
    pub fn sqr_dist(&self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        x * x + y * y + z * z
    }

    /// Get the euclidean distance between two vectors.
    #[inline]
    pub fn dist(&self, other: Self) -> f32 {
        self.sqr_dist(other).sqrt()
    }

    /// Linear interpolation between two vectors by a factor `t`.
    /// For example, `t = 0.5` would return the midpoint between the two vectors.
    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        vec3(
            crate::lerp(self.x, other.x, t),
            crate::lerp(self.y, other.y, t),
            crate::lerp(self.z, other.z, t),
        )
    }

    /// Quadratic bezier interpolation by a factor `t`, using `b` as the anchor point.
    #[inline]
    pub fn bezier3(&self, b: Self, c: Self, t: f32) -> Self {
        vec3(
            crate::bezier3(self.x, b.x, c.x, t),
            crate::bezier3(self.y, b.y, c.y, t),
            crate::bezier3(self.z, b.z, c.z, t),
        )
    }

    /// Cubic bezier interpolation by a factor `t`, using `b` and `c` as the anchor points.
    #[inline]
    pub fn bezier4(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec3(
            crate::bezier4(self.x, b.x, c.x, d.x, t),
            crate::bezier4(self.y, b.y, c.y, d.y, t),
            crate::bezier4(self.z, b.z, c.z, d.z, t),
        )
    }

    /// Catmull-Rom interpolation by a factor `t`, using `b` and `c` as the anchor points.
    #[inline]
    pub fn catmull_rom(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec3(
            crate::catmull_rom(self.x, b.x, c.x, d.x, t),
            crate::catmull_rom(self.y, b.y, c.y, d.y, t),
            crate::catmull_rom(self.z, b.z, c.z, d.z, t),
        )
    }

    /// Hermite interpolation by a factor `t` using the provided tangents.
    #[inline]
    pub fn hermite(&self, self_tangent: Self, other: Self, other_tangent: Self, t: f32) -> Self {
        vec3(
            crate::hermite(self.x, self_tangent.x, other.x, other_tangent.x, t),
            crate::hermite(self.y, self_tangent.y, other.y, other_tangent.y, t),
            crate::hermite(self.z, self_tangent.z, other.z, other_tangent.z, t),
        )
    }

    /// Smooth-step interpolation between vectors by factor `t`.
    #[inline]
    pub fn smooth_step(&self, target: Self, t: f32) -> Self {
        self.lerp(target, crate::smooth_step(t))
    }
}

impl AsRef<[f32]> for Vec3 {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const f32, 3) }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z)
    }
}

impl Hash for Vec3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(crate::hash_f32(self.x));
        state.write_i32(crate::hash_f32(self.y));
        state.write_i32(crate::hash_f32(self.z));
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl From<crate::Vec2> for Vec3 {
    fn from(val: crate::Vec2) -> Self {
        vec3(val.x, val.y, 0.0)
    }
}

impl From<crate::Int3> for Vec3 {
    fn from(val: crate::Int3) -> Self {
        vec3(val.x as f32, val.y as f32, val.z as f32)
    }
}

impl From<Vec3> for crate::Vec2 {
    fn from(val: Vec3) -> Self {
        crate::vec2(val.x, val.y)
    }
}

impl From<f32> for Vec3 {
    fn from(val: f32) -> Self {
        vec3(val, val, val)
    }
}

impl From<(f32, f32)> for Vec3 {
    fn from(val: (f32, f32)) -> Self {
        vec3(val.0, val.1, 0.0)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(val: (f32, f32, f32)) -> Self {
        vec3(val.0, val.1, val.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(val: Vec3) -> Self {
        (val.x, val.y, val.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        vec3(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        vec3(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, n: f32) -> Self {
        vec3(self.x * n, self.y * n, self.z * n)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        vec3(v.x * self, v.y * self, v.z * self)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        vec3(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, n: f32) -> Self {
        vec3(self.x / n, self.y / n, self.z / n)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        vec3(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
