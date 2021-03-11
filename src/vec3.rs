use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[allow(clippy::len_without_is_empty)]
impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub const RIGHT: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const LEFT: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const DOWN: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    pub const BACK: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3(x, y, z)
    }

    #[inline]
    pub fn extend(&self, w: f32) -> crate::Vec4 {
        crate::vec4(self.x, self.y, self.z, w)
    }

    #[inline]
    pub fn from(val: crate::Vec2, z: f32) -> Self {
        vec3(val.x, val.y, z)
    }

    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    #[inline]
    pub fn norm(&self) -> Self {
        let len = self.len();
        vec3(self.x / len, self.y / len, self.z / len)
    }

    #[inline]
    pub fn only_x(&self) -> Self {
        vec3(self.x, 0.0, 0.0)
    }

    #[inline]
    pub fn only_y(&self) -> Self {
        vec3(0.0, self.y, 0.0)
    }

    #[inline]
    pub fn only_z(&self) -> Self {
        vec3(0.0, 0.0, self.z)
    }

    #[inline]
    pub fn abs(&self) -> Self {
        vec3(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline]
    pub fn floor(&self) -> Self {
        vec3(self.x.floor(), self.y.floor(), self.z.floor())
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        vec3(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    #[inline]
    pub fn round(&self) -> Self {
        vec3(self.x.round(), self.y.round(), self.z.round())
    }

    #[inline]
    pub fn min(&self, other: Self) -> Self {
        vec3(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    #[inline]
    pub fn max(&self, other: Self) -> Self {
        vec3(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    #[inline]
    pub fn sign(&self) -> Self {
        vec3(
            crate::sign(self.x),
            crate::sign(self.y),
            crate::sign(self.z),
        )
    }

    #[inline]
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        vec3(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline]
    pub fn sqr_dist(&self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        x * x + y * y + z * z
    }

    #[inline]
    pub fn dist(&self, other: Self) -> f32 {
        self.sqr_dist(other).sqrt()
    }

    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        vec3(
            crate::lerp(self.x, other.x, t),
            crate::lerp(self.y, other.y, t),
            crate::lerp(self.z, other.z, t),
        )
    }

    #[inline]
    pub fn bezier3(&self, b: Self, c: Self, t: f32) -> Self {
        vec3(
            crate::bezier3(self.x, b.x, c.x, t),
            crate::bezier3(self.y, b.y, c.y, t),
            crate::bezier3(self.z, b.z, c.z, t),
        )
    }

    #[inline]
    pub fn bezier4(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec3(
            crate::bezier4(self.x, b.x, c.x, d.x, t),
            crate::bezier4(self.y, b.y, c.y, d.y, t),
            crate::bezier4(self.z, b.z, c.z, d.z, t),
        )
    }

    #[inline]
    pub fn catmull_rom(&self, b: Self, c: Self, d: Self, t: f32) -> Self {
        vec3(
            crate::catmull_rom(self.x, b.x, c.x, d.x, t),
            crate::catmull_rom(self.y, b.y, c.y, d.y, t),
            crate::catmull_rom(self.z, b.z, c.z, d.z, t),
        )
    }

    #[inline]
    pub fn hermite(&self, self_tangent: Self, other: Self, other_tangent: Self, t: f32) -> Self {
        vec3(
            crate::hermite(self.x, self_tangent.x, other.x, other_tangent.x, t),
            crate::hermite(self.y, self_tangent.y, other.y, other_tangent.y, t),
            crate::hermite(self.z, self_tangent.z, other.z, other_tangent.z, t),
        )
    }

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
