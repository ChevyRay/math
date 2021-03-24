use crate::{vec2, vec3, vec4, Radians, Vec2, Vec3, Vec4};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Mul;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Mat4x4 {
    pub m: [f32; 16],
}

#[inline]
pub fn mat4x4(m: [f32; 16]) -> Mat4x4 {
    Mat4x4 { m }
}

impl Mat4x4 {
    pub const ZERO: Self = Self {
        m: [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ],
    };
    pub const IDENTITY: Self = Self {
        m: [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };

    #[inline]
    pub fn new(m: [f32; 16]) -> Self {
        Self { m }
    }

    #[inline]
    pub fn translation(amount: Vec3) -> Self {
        let (x, y, z) = amount.into();
        mat4x4([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
        ])
    }

    #[inline]
    pub fn scale(scale: Vec3) -> Self {
        let (x, y, z) = scale.into();
        mat4x4([
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[inline]
    pub fn rotation_x<A: Into<Radians>>(radians: A) -> Self {
        let a = radians.into().0;
        let c = a.cos();
        let s = a.sin();
        mat4x4([
            1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[inline]
    pub fn rotation_y<A: Into<Radians>>(radians: A) -> Self {
        let a = radians.into().0;
        let c = a.cos();
        let s = a.sin();
        mat4x4([
            c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[inline]
    pub fn rotation_z<A: Into<Radians>>(radians: A) -> Self {
        let a = radians.into().0;
        let c = a.cos();
        let s = a.sin();
        mat4x4([
            c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[inline]
    pub fn orthographic(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near_z: f32,
        far_z: f32,
    ) -> Self {
        mat4x4([
            2.0 / (right - left),
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 / (top - bottom),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / (near_z - far_z),
            0.0,
            (left + right) / (left - right),
            (top + bottom) / (bottom - top),
            near_z / (near_z - far_z),
            1.0,
        ])
    }

    #[inline]
    pub fn rotation_axis<A: Into<Radians>>(axis: Vec3, angle: A) -> Self {
        let (x, y, z) = axis.into();
        let a = angle.into().0;
        let num2 = a.sin();
        let num = a.cos();
        let num11 = x * x;
        let num10 = y * y;
        let num9 = z * z;
        let num8 = x * y;
        let num7 = x * z;
        let num6 = y * z;
        mat4x4([
            num11 + (num * (1.0 - num11)),
            (num8 - (num * num8)) + (num2 * z),
            (num7 - (num * num7)) - (num2 * y),
            0.0,
            (num8 - (num * num8)) - (num2 * z),
            num10 + (num * (1.0 - num10)),
            (num6 - (num * num6)) + (num2 * x),
            0.0,
            (num7 - (num * num7)) + (num2 * y),
            (num6 - (num * num6)) - (num2 * x),
            num9 + (num * (1.0 - num9)),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    #[inline]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let a = target - eye;
        let b = up.cross(a).norm();
        let c = a.cross(b);
        mat4x4([
            b.x,
            c.x,
            a.x,
            0.0,
            b.y,
            c.y,
            a.y,
            0.0,
            b.z,
            c.z,
            a.z,
            0.0,
            -b.dot(eye),
            -c.dot(eye),
            -a.dot(eye),
            1.0,
        ])
    }

    #[inline]
    pub fn perspective(width: f32, height: f32, near_z: f32, far_z: f32) -> Self {
        mat4x4([
            (2.0 * near_z) / width,
            0.0,
            0.0,
            0.0,
            (2.0 * near_z) / height,
            0.0,
            0.0,
            0.0,
            far_z / (near_z - far_z),
            0.0,
            0.0,
            -1.0,
            0.0,
            0.0,
            0.0,
            (near_z * far_z) / (near_z - far_z),
        ])
    }

    #[inline]
    pub fn perspective_fov(fov: f32, aspect_ratio: f32, near_z: f32, far_z: f32) -> Self {
        let num = 1.0 / (fov * 0.5).tan();
        let num9 = num / aspect_ratio;
        mat4x4([
            num9,
            0.0,
            0.0,
            0.0,
            num,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            far_z / (near_z - far_z),
            -1.0,
            0.0,
            0.0,
            0.0,
            (near_z * far_z) / (near_z - far_z),
        ])
    }

    #[inline]
    pub fn transform4(&self, p: &Vec4) -> Vec4 {
        let m = &self.m;
        vec4(
            p.x * m[0] + p.y * m[4] + p.z * m[8] + p.w * m[12],
            p.x * m[1] + p.y * m[5] + p.z * m[9] + p.w * m[13],
            p.x * m[2] + p.y * m[6] + p.z * m[10] + p.w * m[14],
            p.x * m[3] + p.y * m[7] + p.z * m[11] + p.w * m[15],
        )
    }

    #[inline]
    pub fn transform4_dir(&self, p: &Vec4) -> Vec4 {
        let m = &self.m;
        vec4(
            p.x * m[0] + p.y * m[4] + p.z * m[8],
            p.x * m[1] + p.y * m[5] + p.z * m[9],
            p.x * m[2] + p.y * m[6] + p.z * m[10],
            p.x * m[3] + p.y * m[7] + p.z * m[11],
        )
    }

    #[inline]
    pub fn transform3(&self, p: &Vec3) -> Vec3 {
        let m = &self.m;
        vec3(
            p.x * m[0] + p.y * m[4] + p.z * m[8] + m[12],
            p.x * m[1] + p.y * m[5] + p.z * m[9] + m[13],
            p.x * m[2] + p.y * m[6] + p.z * m[10] + m[14],
        )
    }

    #[inline]
    pub fn transform3_dir(&self, p: &Vec3) -> Vec3 {
        let m = &self.m;
        vec3(
            p.x * m[0] + p.y * m[4] + p.z * m[8],
            p.x * m[1] + p.y * m[5] + p.z * m[9],
            p.x * m[2] + p.y * m[6] + p.z * m[10],
        )
    }

    #[inline]
    pub fn transform2(&self, p: Vec2) -> Vec2 {
        let m = &self.m;
        vec2(
            p.x * m[0] + p.y * m[4] + m[12],
            p.x * m[1] + p.y * m[5] + m[13],
        )
    }

    #[inline]
    pub fn transform2_dir(&self, p: Vec2) -> Vec2 {
        let m = &self.m;
        vec2(p.x * m[0] + p.y * m[4], p.x * m[1] + p.y * m[5])
    }

    #[inline]
    pub fn invert(&self) -> Self {
        let m = &self.m;
        let b0 = m[8] * m[13] - m[9] * m[12];
        let b1 = m[8] * m[14] - m[10] * m[12];
        let b2 = m[11] * m[12] - m[8] * m[15];
        let b3 = m[9] * m[14] - m[10] * m[13];
        let b4 = m[11] * m[13] - m[9] * m[15];
        let b5 = m[10] * m[15] - m[11] * m[14];
        let d11 = m[5] * b5 + m[6] * b4 + m[7] * b3;
        let d12 = m[4] * b5 + m[6] * b2 + m[7] * b1;
        let d13 = m[4] * -b4 + m[5] * b2 + m[7] * b0;
        let d14 = m[4] * b3 + m[5] * -b1 + m[6] * b0;
        let det = m[0] * d11 - m[1] * d12 + m[2] * d13 - m[3] * d14;
        if det == 0.0 {
            return Self::ZERO;
        }
        let det = 1.0 / det;
        let a0 = m[0] * m[5] - m[1] * m[4];
        let a1 = m[0] * m[6] - m[2] * m[4];
        let a2 = m[3] * m[4] - m[0] * m[7];
        let a3 = m[1] * m[6] - m[2] * m[5];
        let a4 = m[3] * m[5] - m[1] * m[7];
        let a5 = m[2] * m[7] - m[3] * m[6];
        let d21 = m[1] * b5 + m[2] * b4 + m[3] * b3;
        let d22 = m[0] * b5 + m[2] * b2 + m[3] * b1;
        let d23 = m[0] * -b4 + m[1] * b2 + m[3] * b0;
        let d24 = m[0] * b3 + m[1] * -b1 + m[2] * b0;
        let d31 = m[13] * a5 + m[14] * a4 + m[15] * a3;
        let d32 = m[12] * a5 + m[14] * a2 + m[15] * a1;
        let d33 = m[12] * -a4 + m[13] * a2 + m[15] * a0;
        let d34 = m[12] * a3 + m[13] * -a1 + m[14] * a0;
        let d41 = m[9] * a5 + m[10] * a4 + m[11] * a3;
        let d42 = m[8] * a5 + m[10] * a2 + m[11] * a1;
        let d43 = m[8] * -a4 + m[9] * a2 + m[11] * a0;
        let d44 = m[8] * a3 + m[9] * -a1 + m[10] * a0;
        mat4x4([
            d11 * det,
            -d21 * det,
            d31 * det,
            -d41 * det,
            -d12 * det,
            d22 * det,
            -d32 * det,
            d42 * det,
            d13 * det,
            -d23 * det,
            d33 * det,
            -d43 * det,
            -d14 * det,
            d24 * det,
            -d34 * det,
            d44 * det,
        ])
    }

    #[inline]
    pub fn mult(&self, other: &Self) -> Self {
        let a = &self.m;
        let b = &other.m;
        mat4x4([
            a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12],
            a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13],
            a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14],
            a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15],
            a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12],
            a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13],
            a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14],
            a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15],
            a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12],
            a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13],
            a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14],
            a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15],
            a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12],
            a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13],
            a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14],
            a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15],
        ])
    }
}

impl AsRef<[f32]> for Mat4x4 {
    fn as_ref(&self) -> &[f32] {
        &self.m
    }
}

impl PartialEq for Mat4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.m.eq(&other.m)
    }
}

impl Hash for Mat4x4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for x in &self.m {
            state.write_i32(crate::hash_f32(*x))
        }
    }
}

impl fmt::Display for Mat4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = &self.m;
        write!(
            f,
            "{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}",
            m[0],
            m[1],
            m[2],
            m[3],
            m[4],
            m[5],
            m[6],
            m[7],
            m[8],
            m[9],
            m[10],
            m[11],
            m[12],
            m[13],
            m[14],
            m[15],
        )
    }
}

impl Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;
    #[inline]
    fn mul(self, other: Mat4x4) -> Mat4x4 {
        self.mult(&other)
    }
}
