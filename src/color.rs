use crate::{Vec4};
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::{Serializer, Deserializer, Serialize, Deserialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Rem, RemAssign, Index};

/// A 32-bit RGBA color, with 8-bits per channel.
#[repr(C)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// 0x00000000
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    /// 0x000000ff
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    /// 0xffffffff
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    /// 0xff0000ff
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    /// 0x00ff00ff
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    /// 0x0000ffff
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    /// 0xffff00ff
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    /// 0x00ffffff
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    /// 0xff00ffff
    pub const FUCHSIA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    /// 0x808080ff
    pub const GREY: Self = Self {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };

    /// Construct a color from RGBA components.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Construct a fully-opaque color from RGB components.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Pack the color into an RGBA hexadecimal value.
    pub fn packed(self) -> u32 {
        self.into()
    }

    /// Construct a color from RGBA floating-point components in range (0.0 - 1.0).
    pub fn rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }

    /// Construct a fully-opaque color from RGB floating-point components in range (0.0 - 1.0).
    pub fn rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Self::rgba_f32(r, g, b, 1.0)
    }

    /// Construct a fully-saturated color from a radial hue.
    pub fn hue(deg: f32) -> Self {
        let mut h = deg % 360.0;
        if h < 0.0 {
            h += 360.0
        }
        if h < 60.0 {
            Self::rgb_f32(1.0, h / 60.0, 0.0)
        } else if h < 120.0 {
            Self::rgb_f32(1.0 - (h - 60.0) / 60.0, 1.0, 0.0)
        } else if h < 180.0 {
            Self::rgb_f32(0.0, 1.0, (h - 120.0) / 60.0)
        } else if h < 240.0 {
            Self::rgb_f32(0.0, 1.0 - (h - 180.0) / 60.0, 1.0)
        } else if h < 300.0 {
            Self::rgb_f32((h - 240.0) / 60.0, 0.0, 1.0)
        } else {
            Self::rgb_f32(1.0, 0.0, 1.0 - (h - 300.0) / 60.0)
        }
    }

    /// Linearly interpolate between two colors by a factor `t`.
    ///
    /// **NOTE:** the resulting RGBA components are truncated into u8 values,
    /// so this cannot be treated as an equivalent to [Vec4::lerp()](struct.Vec4.html#method.lerp).
    pub fn lerp(self, to: Self, t: f32) -> Self {
        Self {
            r: crate::lerp(self.r as f32, to.r as f32, t) as u8,
            g: crate::lerp(self.g as f32, to.g as f32, t) as u8,
            b: crate::lerp(self.b as f32, to.b as f32, t) as u8,
            a: crate::lerp(self.a as f32, to.a as f32, t) as u8,
        }
    }

    /// Quadratic bezier interpolate between two colors by a factor `t`.
    ///
    /// **NOTE:** the resulting RGBA components are truncated into u8 values,
    /// so this cannot be treated as an equivalent to [Vec4::bezier3()](struct.Vec4.html#method.bezier3).
    pub fn bezier3(self, b: Self, c: Self, t: f32) -> Self {
        Self {
            r: crate::bezier3(self.r as f32, b.r as f32, c.r as f32, t) as u8,
            g: crate::bezier3(self.g as f32, b.g as f32, c.g as f32, t) as u8,
            b: crate::bezier3(self.b as f32, b.b as f32, c.b as f32, t) as u8,
            a: crate::bezier3(self.a as f32, b.a as f32, c.a as f32, t) as u8,
        }
    }

    /// Cubic bezier interpolate between two colors by a factor `t`.
    ///
    /// **NOTE:** the resulting RGBA components are truncated into u8 values,
    /// so this cannot be treated as an equivalent to [Vec4::bezier4()](struct.Vec4.html#method.bezier4).
    pub fn bezier4(self, b: Self, c: Self, d: Self, t: f32) -> Self {
        Self {
            r: crate::bezier4(self.r as f32, b.r as f32, c.r as f32, d.r as f32, t) as u8,
            g: crate::bezier4(self.g as f32, b.g as f32, c.g as f32, d.g as f32, t) as u8,
            b: crate::bezier4(self.b as f32, b.b as f32, c.b as f32, d.b as f32, t) as u8,
            a: crate::bezier4(self.a as f32, b.a as f32, c.a as f32, d.a as f32, t) as u8,
        }
    }

    /// Retrieve the RGBA components as floating-point values in range (0.0 - 1.0).
    pub fn floats(self) -> (f32, f32, f32, f32) {
        (
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        )
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hex: u32 = (*self).into();
        write!(f, "{:08x}", hex)
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.packed().partial_cmp(&other.packed())
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Self) -> Ordering {
        self.packed().cmp(&other.packed())
    }
}

impl Index<usize> for Color {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 4);
        &self.as_ref()[index]
    }
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        Self {
            r: (val >> 24) as u8,
            g: (val >> 16) as u8,
            b: (val >> 8) as u8,
            a: val as u8,
        }
    }
}

impl From<Color> for u32 {
    fn from(val: Color) -> Self {
        (val.r as u32) << 24 | (val.g as u32) << 16 | (val.b as u32) << 8 | (val.a as u32)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(val: (u8, u8, u8, u8)) -> Self {
        Self::rgba(val.0, val.1, val.2, val.3)
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(val: Color) -> Self {
        (val.r, val.g, val.b, val.a)
    }
}

impl From<[u8; 4]> for Color {
    fn from(val: [u8; 4]) -> Self {
        Self::rgba(val[0], val[1], val[2], val[3])
    }
}

impl From<Color> for [u8; 4] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

impl From<Vec4> for Color {
    fn from(val: Vec4) -> Self {
        Self::rgba_f32(val.x, val.y, val.z, val.w)
    }
}

impl From<Color> for Vec4 {
    fn from(val: Color) -> Self {
        val.floats().into()
    }
}

impl AsRef<[u8]> for Color {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, 4) }
    }
}

impl Add<Color> for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::rgba(
            self.r.saturating_add(other.r),
            self.g.saturating_add(other.g),
            self.b.saturating_add(other.b),
            self.a.saturating_add(other.a),
        )
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}

impl Sub<Color> for Color {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::rgba(
            self.r.saturating_sub(other.r),
            self.g.saturating_sub(other.g),
            self.b.saturating_sub(other.b),
            self.a.saturating_sub(other.a),
        )
    }
}

impl SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        *self = *self - rhs;
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let (r1, g1, b1, a1) = self.floats();
        let (r2, g2, b2, a2) = other.floats();
        Self::rgba_f32(r1 * r2, g1 * g2, b1 * b2, a1 * a2)
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, n: f32) -> Self {
        let (r, g, b, a) = self.floats();
        Self::rgba_f32(r * n, g * n, b * n, a * n)
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, color: Color) -> Color {
        let (r, g, b, a) = color.floats();
        Color::rgba_f32(r * self, g * self, b * self, a * self)
    }
}

impl Div<Color> for Color {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let (r1, g1, b1, a1) = self.floats();
        let (r2, g2, b2, a2) = other.floats();
        Self::rgba_f32(r1 / r2, g1 / g2, b1 / b2, a1 / a2)
    }
}

impl DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        *self = *self / rhs;
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, n: f32) -> Self {
        let (r, g, b, a) = self.floats();
        Self::rgba_f32(r / n, g / n, b / n, a / n)
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl BitAnd<Color> for Color {
    type Output = Self;
    fn bitand(self, rhs: Color) -> Self::Output {
        Color::from(self.packed().bitand(rhs.packed()))
    }
}

impl BitAndAssign<Color> for Color {
    fn bitand_assign(&mut self, rhs: Color) {
        *self = self.bitand(rhs);
    }
}

impl BitOr<Color> for Color {
    type Output = Color;
    fn bitor(self, rhs: Color) -> Self::Output {
        Color::from(self.packed().bitor(rhs.packed()))
    }
}

impl BitOrAssign<Color> for Color {
    fn bitor_assign(&mut self, rhs: Color) {
        *self = self.bitor(rhs);
    }
}

impl BitXor<Color> for Color {
    type Output = Color;
    fn bitxor(self, rhs: Color) -> Self::Output {
        Color::from(self.packed().bitxor(rhs.packed()))
    }
}

impl BitXorAssign<Color> for Color {
    fn bitxor_assign(&mut self, rhs: Color) {
        *self = self.bitxor(rhs);
    }
}

impl Rem<Color> for Color {
    type Output = Color;
    fn rem(self, rhs: Color) -> Self::Output {
        Color::rgba(
            self.r % rhs.r,
            self.g % rhs.g,
            self.b % rhs.b,
            self.a % rhs.a,
        )
    }
}

impl RemAssign<Color> for Color {
    fn rem_assign(&mut self, rhs: Color) {
        *self = self.rem(rhs);
    }
}

impl Rem<u8> for Color {
    type Output = Color;
    fn rem(self, rhs: u8) -> Self::Output {
        Color::rgba(
            self.r % rhs,
            self.g % rhs,
            self.b % rhs,
            self.a % rhs,
        )
    }
}

impl RemAssign<u8> for Color {
    fn rem_assign(&mut self, rhs: u8) {
        *self = self.rem(rhs);
    }
}

#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.packed())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(ColorVisitor)
    }
}

#[cfg(feature = "serde")]
struct ColorVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an unsigned 32-bit integer")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Color::from(v))
    }
}
