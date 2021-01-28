use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };
    pub const UP: Self = Self { x: 0.0, y: -1.0 };

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        vec2(x, y)
    }

    #[inline]
    pub fn extend(&self, z: f32) -> crate::Vec3 {
        crate::vec3(self.x, self.y, z)
    }

    #[inline]
    pub fn polar(radians: f32) -> Self {
        vec2(radians.cos(), radians.sin())
    }

    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    #[inline]
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn bary(a: Self, b: Self, c: Self, t1: f32, t2: f32) -> Self {
        vec2(
            (a.x + (t1 * (b.x - a.x))) + (t2 * (c.x - a.x)),
            (a.y + (t1 * (b.y - a.y))) + (t2 * (c.y - a.y)),
        )
    }

    #[inline]
    pub fn norm(&self) -> Self {
        let len = self.len();
        vec2(self.x / len, self.y / len)
    }

    #[inline]
    pub fn turn_left(&self) -> Self {
        vec2(self.y, -self.x)
    }

    #[inline]
    pub fn turn_right(&self) -> Self {
        vec2(-self.y, self.x)
    }

    #[inline]
    pub fn only_x(&self) -> Self {
        vec2(self.x, 0.0)
    }

    #[inline]
    pub fn only_y(&self) -> Self {
        vec2(0.0, self.y)
    }

    #[inline]
    pub fn abs(&self) -> Self {
        vec2(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn floor(&self) -> Self {
        vec2(self.x.floor(), self.y.floor())
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        vec2(self.x.ceil(), self.y.ceil())
    }

    #[inline]
    pub fn round(&self) -> Self {
        vec2(self.x.round(), self.y.round())
    }

    #[inline]
    pub fn min(&self, other: Self) -> Self {
        vec2(self.x.min(other.x), self.y.min(other.y))
    }

    #[inline]
    pub fn max(&self, other: Self) -> Self {
        vec2(self.x.max(other.x), self.y.max(other.y))
    }

    #[inline]
    pub fn sign(&self) -> Self {
        vec2(crate::sign(self.x), crate::sign(self.y))
    }

    #[inline]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(&self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    pub fn project(&self, origin: Self, axis: Self) -> Self {
        origin + axis * self.dot(axis)
    }

    #[inline]
    pub fn sqr_dist(&self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        x * x + y * y
    }

    #[inline]
    pub fn dist(&self, other: Self) -> f32 {
        self.sqr_dist(other).sqrt()
    }

    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        vec2(
            crate::lerp(self.x, other.x, t),
            crate::lerp(self.y, other.y, t),
        )
    }

    #[inline]
    pub fn bezier3(&self, b: Self, c: Self, t: f32) -> Self {
        vec2(
            crate::bezier3(self.x, b.x, c.x, t),
            crate::bezier3(self.y, b.y, c.y, t),
        )
    }

    #[inline]
    pub fn bezier4(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec2(
            crate::bezier4(self.x, b.x, c.x, d.x, t),
            crate::bezier4(self.y, b.y, c.y, d.y, t),
        )
    }

    #[inline]
    pub fn catmull_rom(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec2(
            crate::catmull_rom(self.x, b.x, c.x, d.x, t),
            crate::catmull_rom(self.y, b.y, c.y, d.y, t),
        )
    }

    #[inline]
    pub fn hermite(&self, self_tangent: Self, other: Self, other_tangent: Self, t: f32) -> Self {
        vec2(
            crate::hermite(self.x, self_tangent.x, other.x, other_tangent.x, t),
            crate::hermite(self.y, self_tangent.y, other.y, other_tangent.y, t),
        )
    }

    #[inline]
    pub fn reflect(&self, axis: Self) -> Self {
        let val = self.dot(axis) * 2.0;
        vec2(self.x - axis.x * val, self.y - axis.y * val)
    }

    #[inline]
    pub fn smooth_step(&self, target: Self, t: f32) -> Self {
        self.lerp(target, crate::smooth_step(t))
    }
}

impl AsRef<[f32]> for Vec2 {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const f32, 2) }
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
    #[inline]
    fn neg(self) -> Self {
        vec2(-self.x, -self.y)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        vec2(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        vec2(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, n: f32) -> Self {
        vec2(self.x * n, self.y * n)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, v: Vec2) -> Vec2 {
        vec2(v.x * self, v.y * self)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        vec2(self.x * other.x, self.y * other.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, n: f32) -> Self {
        vec2(self.x / n, self.y / n)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        vec2(self.x / other.x, self.y / other.y)
    }
}
