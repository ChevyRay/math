use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::{Radians, Vec3};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A two-dimensional floating point vector.
#[derive(Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Easy constructor.
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

#[allow(clippy::len_without_is_empty)]
impl Vec2 {
    /// (0.0, 0.0)
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// (1.0, 1.0)
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    /// (1.0, 0.0)
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    /// (-1.0, 0.0)
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };

    /// (0.0, 1.0)
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };

    /// (0.0, -1.0)
    pub const UP: Self = Self { x: 0.0, y: -1.0 };

    /// Create a new vector.
    pub fn new(x: f32, y: f32) -> Self {
        vec2(x, y)
    }

    /// Extend this vector with a z-axis.
    pub fn extend(&self, z: f32) -> Vec3 {
        crate::vec3(self.x, self.y, z)
    }

    /// Create a normalized directional vector from the supplied rotation.
    pub fn polar<A: Into<Radians>>(rotation: A) -> Self {
        let rad = rotation.into().0;
        vec2(rad.cos(), rad.sin())
    }

    /// The length of the vector, squared.
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// The euclidean length of the vector.
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    /// Get the angle of the vector in radians.
    pub fn angle(&self) -> Radians {
        Radians(self.y.atan2(self.x))
    }

    /// Barycentric coordinate.
    pub fn bary(a: Self, b: Self, c: Self, t1: f32, t2: f32) -> Self {
        vec2(
            (a.x + (t1 * (b.x - a.x))) + (t2 * (c.x - a.x)),
            (a.y + (t1 * (b.y - a.y))) + (t2 * (c.y - a.y)),
        )
    }

    /// Normalize the vector.
    pub fn norm(&self) -> Self {
        let len = self.len();
        vec2(self.x / len, self.y / len)
    }

    /// Rotate the vector 90º left, creating a perpendicular vector.
    pub fn turn_left(&self) -> Self {
        vec2(self.y, -self.x)
    }

    /// Rotate the vector 90º right, creating a perpendicular vector.
    pub fn turn_right(&self) -> Self {
        vec2(-self.y, self.x)
    }

    /// Zero the y-axis of the vector.
    pub fn only_x(&self) -> Self {
        vec2(self.x, 0.0)
    }

    /// Zero the x-axis of the vector.
    pub fn only_y(&self) -> Self {
        vec2(0.0, self.y)
    }

    /// Absolute the vector's components.
    pub fn abs(&self) -> Self {
        vec2(self.x.abs(), self.y.abs())
    }

    /// Round the vector's components down.
    pub fn floor(&self) -> Self {
        vec2(self.x.floor(), self.y.floor())
    }

    /// Round the vector's components up.
    pub fn ceil(&self) -> Self {
        vec2(self.x.ceil(), self.y.ceil())
    }

    /// Round the vector's components.
    pub fn round(&self) -> Self {
        vec2(self.x.round(), self.y.round())
    }

    /// Return the minimum of the vector's components.
    pub fn min(&self, other: Self) -> Self {
        vec2(self.x.min(other.x), self.y.min(other.y))
    }

    /// Return the maximum of the vector's components.
    pub fn max(&self, other: Self) -> Self {
        vec2(self.x.max(other.x), self.y.max(other.y))
    }

    /// Sign the vector's components.
    pub fn sign(&self) -> Self {
        vec2(crate::sign(self.x), crate::sign(self.y))
    }

    /// Return a vector with its components clamped in the provided range.
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    /// Return the dot product of two vectors.
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Return the cross product of two vectors.
    pub fn cross(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Given an origin and normalized axis, project the vector into a point along that axis.
    pub fn project(&self, origin: Self, axis: Self) -> Self {
        origin + axis * self.dot(axis)
    }

    /// Get the square distance between two vectors.
    pub fn sqr_dist(&self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        x * x + y * y
    }

    /// Get the euclidean distance between two vectors.
    pub fn dist(&self, other: Self) -> f32 {
        self.sqr_dist(other).sqrt()
    }

    /// Linear interpolation between two vectors by a factor `t`.
    /// For example, `t = 0.5` would return the midpoint between the two vectors.
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        vec2(
            crate::lerp(self.x, other.x, t),
            crate::lerp(self.y, other.y, t),
        )
    }

    /// Quadratic bezier interpolation by a factor `t`, using `b` as the anchor point.
    pub fn bezier3(&self, b: Self, c: Self, t: f32) -> Self {
        vec2(
            crate::bezier3(self.x, b.x, c.x, t),
            crate::bezier3(self.y, b.y, c.y, t),
        )
    }

    /// Cubic bezier interpolation by a factor `t`, using `b` and `c` as the anchor points.
    pub fn bezier4(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec2(
            crate::bezier4(self.x, b.x, c.x, d.x, t),
            crate::bezier4(self.y, b.y, c.y, d.y, t),
        )
    }

    /// Catmull-Rom interpolation by a factor `t`, using `b` and `c` as the anchor points.
    pub fn catmull_rom(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec2(
            crate::catmull_rom(self.x, b.x, c.x, d.x, t),
            crate::catmull_rom(self.y, b.y, c.y, d.y, t),
        )
    }

    /// Hermite interpolation by a factor `t` using the provided tangents.
    pub fn hermite(&self, self_tangent: Self, other: Self, other_tangent: Self, t: f32) -> Self {
        vec2(
            crate::hermite(self.x, self_tangent.x, other.x, other_tangent.x, t),
            crate::hermite(self.y, self_tangent.y, other.y, other_tangent.y, t),
        )
    }

    /// Reflect a vector off the provided surface normal.
    pub fn reflect(&self, normal: Self) -> Self {
        let val = self.dot(normal) * 2.0;
        vec2(self.x - normal.x * val, self.y - normal.y * val)
    }

    /// Smooth-step interpolation between vectors by factor `t`.
    pub fn smooth_step(&self, target: Self, t: f32) -> Self {
        self.lerp(target, crate::smooth_step(t))
    }
}

impl AsRef<[f32]> for Vec2 {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const f32, 2) }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(crate::hash_f32(self.x));
        state.write_i32(crate::hash_f32(self.y));
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl From<f32> for Vec2 {
    fn from(val: f32) -> Self {
        vec2(val, val)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(val: (f32, f32)) -> Self {
        vec2(val.0, val.1)
    }
}

impl From<Vec2> for (f32, f32) {
    fn from(val: Vec2) -> Self {
        (val.x, val.y)
    }
}

impl From<crate::Int2> for Vec2 {
    fn from(val: crate::Int2) -> Self {
        vec2(val.x as f32, val.y as f32)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        vec2(-self.x, -self.y)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        vec2(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = self.add(rhs);
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        vec2(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = self.sub(rhs);
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, n: f32) -> Self {
        vec2(self.x * n, self.y * n)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.mul(rhs);
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 {
        vec2(v.x * self, v.y * self)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        vec2(self.x * other.x, self.y * other.y)
    }
}

impl MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        *self = self.mul(rhs);
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, n: f32) -> Self {
        vec2(self.x / n, self.y / n)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = self.div(rhs);
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        vec2(self.x / other.x, self.y / other.y)
    }
}

impl DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        *self = self.div(rhs);
    }
}
