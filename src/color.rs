use crate::Vec4;
use serde::de::{Error, Visitor};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Index, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

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

    /// Convert an HSV color to RGBA.
    ///
    /// `h`: hue in degrees
    /// `s`: saturation (0 - 1)
    /// `v`: value (0 - 1)
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let range = (h / 60.0) as u8;
        let c = v * s;
        let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
        let m = v - c;
        match range {
            0 => Self::rgb_f32((c + m) * 255.0, (x + m) * 255.0, m * 255.0),
            1 => Self::rgb_f32((x + m) * 255.0, (c + m) * 255.0, m * 255.0),
            2 => Self::rgb_f32(m * 255.0, (c + m) * 255.0, (x + m) * 255.0),
            3 => Self::rgb_f32(m * 255.0, (x + m) * 255.0, (c + m) * 255.0),
            4 => Self::rgb_f32((x + m) * 255.0, m * 255.0, (c + m) * 255.0),
            _ => Self::rgb_f32((c + m) * 255.0, m * 255.0, (x + m) * 255.0),
        }
    }

    /// Convert to hue-saturation-value color space.
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let (r, g, b, _) = self.floats();

        let min = r.min(g.min(b));
        let max = r.max(g.max(b));
        let delta = max - min;

        let v = max;
        let s = match max > 1e-3 {
            true => delta / max,
            false => 0.0,
        };

        let h = match delta == 0.0 {
            true => 0.0,
            false => {
                if r == max {
                    (g - b) / delta
                } else if g == max {
                    2.0 + (b - r) / delta
                } else {
                    4.0 + (r - g) / delta
                }
            }
        };
        let h = ((h * 60.0) + 360.0) % 360.0;

        (h, s, v)
    }

    /// Convert to [CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) XYZ color space.
    pub fn to_xyz(&self) -> (f32, f32, f32) {
        fn comp(r: f32) -> f32 {
            if r <= 0.04045 {
                r / 12.92
            } else {
                ((r + 0.055) / 1.055).powf(2.4)
            }
        }
        let (r, g, b, _) = self.floats();
        let (r, g, b) = (comp(r), comp(g), comp(b));
        (
            (0.4124 * r + 0.3576 * g + 0.1805 * b) * 100.0,
            (0.2126 * r + 0.7152 * g + 0.0722 * b) * 100.0,
            (0.0193 * r + 0.1192 * g + 0.9505 * b) * 100.0,
        )
    }

    /// Convert from [CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) XYZ color space.
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        let (x, y, z) = (x / 100.0, y / 100.0, z / 100.0);
        let r = x * 3.2404542 + y * -1.5371385 + z * -0.4985314;
        let g = x * -0.9692660 + y * 1.8760108 + z * 0.0415560;
        let b = x * 0.0556434 + y * -0.2040259 + z * 1.0572252;
        fn comp(r: f32) -> f32 {
            if r > 0.0031308 {
                1.055 * r.powf(1.0 / 2.4) - 0.055
            } else {
                12.92 * r
            }
        }
        Self::rgb_f32(comp(r), comp(g), comp(b))
    }

    /// Convert to [OKLab](https://bottosson.github.io/posts/oklab) color space.
    pub fn to_oklab(&self) -> (f32, f32, f32) {
        fn comp(r: f32) -> f32 {
            if r > 0.04045 {
                ((r + 0.055) / 1.055).powf(2.4)
            } else {
                r / 12.92
            }
        }
        let (r, g, b, _) = self.floats();
        let (r, g, b) = (comp(r), comp(g), comp(b));
        let l = (0.4121656120 * r + 0.5362752080 * g + 0.0514575653 * b).cbrt();
        let m = (0.2118591070 * r + 0.6807189584 * g + 0.1074065790 * b).cbrt();
        let s = (0.0883097947 * r + 0.2818474174 * g + 0.6302613616 * b).cbrt();
        (
            0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
            1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
            0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
        )
    }

    /// Convert to [CIELAB](https://en.wikipedia.org/wiki/CIELAB_color_space) color space.
    pub fn to_cielab(&self) -> (f32, f32, f32) {
        let (x, y, z) = self.to_xyz();
        let x = x / 95.047;
        let y = y / 100.0;
        let z = z / 108.883;
        let x = if x > 0.008856 {
            x.cbrt()
        } else {
            7.787 * x + 16.0 / 116.0
        };
        let y = if y > 0.008856 {
            y.cbrt()
        } else {
            7.787 * y + 16.0 / 116.0
        };
        let z = if z > 0.008856 {
            z.cbrt()
        } else {
            7.787 * z + 16.0 / 116.0
        };
        ((116.0 * y) - 16.0, 500.0 * (x - y), 200.0 * (y - z))
    }

    /// Convert from [CIELAB](https://en.wikipedia.org/wiki/CIELAB_color_space) color space.
    pub fn from_cielab(l: f32, a: f32, b: f32) -> Self {
        let y = (l + 16.0) / 116.0;
        let x = a / 500.0 + y;
        let z = y - b / 200.0;
        let x3 = x.powf(3.0);
        let y3 = y.powf(3.0);
        let z3 = z.powf(3.0);
        let x = 95.047
            * if x3 > 0.008856 {
                x3
            } else {
                (x - 16.0 / 116.0) / 7.787
            };
        let y = 100.0
            * if y3 > 0.008856 {
                y3
            } else {
                (y - 16.0 / 116.0) / 7.787
            };
        let z = 108.883
            * if z3 > 0.008856 {
                z3
            } else {
                (z - 16.0 / 116.0) / 7.787
            };
        Self::from_xyz(x, y, z)
    }

    /// Convert from [OKLab](https://bottosson.github.io/posts/oklab) color space.
    pub fn from_oklab(l: f32, a: f32, b: f32) -> Self {
        let (l, m, s) = (
            (l + 0.3963377774 * a + 0.2158037573 * b).powf(3.0),
            (l - 0.1055613458 * a - 0.0638541728 * b).powf(3.0),
            (l - 0.0894841775 * a - 1.2914855480 * b).powf(3.0),
        );
        let r = 4.0767245293 * l - 3.3072168827 * m + 0.2307590544 * s;
        let g = -1.2681437731 * l + 2.6093323231 * m - 0.3411344290 * s;
        let b = -0.0041119885 * l - 0.7034763098 * m + 1.7068625689 * s;
        fn comp(r: f32) -> f32 {
            if r > 0.0031308 {
                1.055 * r.powf(2.4) - 0.055
            } else {
                12.92 * r
            }
        }
        Self::rgb_f32(comp(r), comp(g), comp(b))
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
        Color::rgba(self.r % rhs, self.g % rhs, self.b % rhs, self.a % rhs)
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
