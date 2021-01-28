use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        vec4(x, y, z, w)
    }

    #[inline]
    pub fn from(val: crate::Vec3, w: f32) -> Self {
        vec4(val.x, val.y, val.z, w)
    }

    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    #[inline]
    pub fn norm(&self) -> Self {
        let len = self.len();
        vec4(self.x / len, self.y / len, self.z / len, self.w / len)
    }

    #[inline]
    pub fn only_x(&self) -> Self {
        vec4(self.x, 0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn only_y(&self) -> Self {
        vec4(0.0, self.y, 0.0, 0.0)
    }

    #[inline]
    pub fn only_z(&self) -> Self {
        vec4(0.0, 0.0, self.z, 0.0)
    }

    #[inline]
    pub fn only_w(&self) -> Self {
        vec4(0.0, 0.0, 0.0, self.w)
    }

    #[inline]
    pub fn abs(&self) -> Self {
        vec4(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    #[inline]
    pub fn floor(&self) -> Self {
        vec4(self.x.floor(), self.y.floor(), self.z.abs(), self.w.abs())
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        vec4(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.abs())
    }

    #[inline]
    pub fn round(&self) -> Self {
        vec4(
            self.x.round(),
            self.y.round(),
            self.z.round(),
            self.w.round(),
        )
    }

    #[inline]
    pub fn min(&self, other: Self) -> Self {
        vec4(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    #[inline]
    pub fn max(&self, other: Self) -> Self {
        vec4(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }

    #[inline]
    pub fn sign(&self) -> Self {
        vec4(
            crate::sign(self.x),
            crate::sign(self.y),
            crate::sign(self.z),
            crate::sign(self.w),
        )
    }

    #[inline]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    #[inline]
    pub fn sqr_dist(&self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        let w = self.w - other.w;
        x * x + y * y + z * z + w * w
    }

    #[inline]
    pub fn dist(&self, other: Self) -> f32 {
        self.sqr_dist(other).sqrt()
    }

    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        vec4(
            crate::lerp(self.x, other.x, t),
            crate::lerp(self.y, other.y, t),
            crate::lerp(self.z, other.z, t),
            crate::lerp(self.w, other.w, t),
        )
    }

    #[inline]
    pub fn bezier3(&self, b: Self, c: Self, t: f32) -> Self {
        vec4(
            crate::bezier3(self.x, b.x, c.x, t),
            crate::bezier3(self.y, b.y, c.y, t),
            crate::bezier3(self.z, b.z, c.z, t),
            crate::bezier3(self.w, b.w, c.w, t),
        )
    }

    #[inline]
    pub fn bezier4(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec4(
            crate::bezier4(self.x, b.x, c.x, d.x, t),
            crate::bezier4(self.y, b.y, c.y, d.y, t),
            crate::bezier4(self.z, b.z, c.z, d.z, t),
            crate::bezier4(self.w, b.w, c.w, d.w, t),
        )
    }

    #[inline]
    pub fn catmull_rom(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec4(
            crate::catmull_rom(self.x, b.x, c.x, d.x, t),
            crate::catmull_rom(self.y, b.y, c.y, d.y, t),
            crate::catmull_rom(self.z, b.z, c.z, d.z, t),
            crate::catmull_rom(self.w, b.w, c.w, d.w, t),
        )
    }

    #[inline]
    pub fn hermite(&self, self_tangent: Self, other: Self, other_tangent: Self, t: f32) -> Self {
        vec4(
            crate::hermite(self.x, self_tangent.x, other.x, other_tangent.x, t),
            crate::hermite(self.y, self_tangent.y, other.y, other_tangent.y, t),
            crate::hermite(self.z, self_tangent.z, other.z, other_tangent.z, t),
            crate::hermite(self.w, self_tangent.w, other.w, other_tangent.w, t),
        )
    }

    #[inline]
    pub fn smooth_step(&self, target: Self, t: f32) -> Self {
        self.lerp(target, crate::smooth_step(t))
    }
}

impl AsRef<[f32]> for Vec4 {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const f32, 4) }
    }
}

impl Hash for Vec4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(crate::hash_f32(self.x));
        state.write_i32(crate::hash_f32(self.y));
        state.write_i32(crate::hash_f32(self.z));
        state.write_i32(crate::hash_f32(self.w));
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.x, self.y, self.z, self.w)
    }
}

impl From<crate::Vec2> for Vec4 {
    fn from(val: crate::Vec2) -> Self {
        vec4(val.x, val.y, 0.0, 0.0)
    }
}

impl From<crate::Vec3> for Vec4 {
    fn from(val: crate::Vec3) -> Self {
        vec4(val.x, val.y, val.z, 0.0)
    }
}

impl From<Vec4> for crate::Vec2 {
    fn from(val: Vec4) -> Self {
        crate::vec2(val.x, val.y)
    }
}

impl From<Vec4> for crate::Vec3 {
    fn from(val: Vec4) -> Self {
        crate::vec3(val.x, val.y, val.z)
    }
}

impl From<f32> for Vec4 {
    fn from(val: f32) -> Self {
        vec4(val, val, val, val)
    }
}

impl From<(f32, f32)> for Vec4 {
    fn from(val: (f32, f32)) -> Self {
        vec4(val.0, val.1, 0.0, 0.0)
    }
}

impl From<(f32, f32, f32)> for Vec4 {
    fn from(val: (f32, f32, f32)) -> Self {
        vec4(val.0, val.1, val.2, 0.0)
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from(val: (f32, f32, f32, f32)) -> Self {
        vec4(val.0, val.1, val.2, val.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    fn from(val: Vec4) -> Self {
        (val.x, val.y, val.z, val.w)
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        vec4(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        vec4(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        vec4(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, n: f32) -> Self {
        vec4(self.x * n, self.y * n, self.z * n, self.w * n)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, v: Vec4) -> Vec4 {
        vec4(v.x * self, v.y * self, v.z * self, v.w * self)
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        vec4(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, n: f32) -> Self {
        vec4(self.x / n, self.y / n, self.z / n, self.w / n)
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        vec4(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
            self.w / other.w,
        )
    }
}
