use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub};
use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const FUCHSIA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const GREY: Self = Self {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };

    #[inline]
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    #[inline]
    pub fn new(hex: u32) -> Self {
        hex.into()
    }

    #[inline]
    pub fn packed(self) -> u32 {
        self.into()
    }

    #[inline]
    pub fn rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }

    #[inline]
    pub fn rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Self::rgba_f32(r, g, b, 1.0)
    }

    #[inline]
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

    #[inline]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        Self {
            r: crate::lerp(self.r as f32, to.r as f32, t) as u8,
            g: crate::lerp(self.g as f32, to.g as f32, t) as u8,
            b: crate::lerp(self.b as f32, to.b as f32, t) as u8,
            a: crate::lerp(self.a as f32, to.a as f32, t) as u8,
        }
    }

    #[inline]
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

impl From<u32> for Color {
    #[inline]
    fn from(val: u32) -> Self {
        Self {
            r: (val >> 24) as u8,
            g: (val >> 16) as u8,
            b: (val >> 8) as u8,
            a: val as u8,
        }
    }
}

impl From<[u8; 4]> for Color {
    #[inline]
    fn from(val: [u8; 4]) -> Self {
        Self {
            r: val[0],
            g: val[1],
            b: val[2],
            a: val[3],
        }
    }
}

impl From<Color> for [u8; 4] {
    #[inline]
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

impl From<Color> for u32 {
    #[inline]
    fn from(val: Color) -> Self {
        (val.r as u32) << 24 | (val.g as u32) << 16 | (val.b as u32) << 8 | (val.a as u32)
    }
}

impl Add<Color> for Color {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self::rgba(
            self.r.saturating_add(other.r),
            self.g.saturating_add(other.g),
            self.b.saturating_add(other.b),
            self.a.saturating_add(other.a),
        )
    }
}

impl Sub<Color> for Color {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::rgba(
            self.r.saturating_sub(other.r),
            self.g.saturating_sub(other.g),
            self.b.saturating_sub(other.b),
            self.a.saturating_sub(other.a),
        )
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        let (r1, g1, b1, a1) = self.floats();
        let (r2, g2, b2, a2) = other.floats();
        Self::rgba_f32(r1 * r2, g1 * g2, b1 * b2, a1 * a2)
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    #[inline]
    fn mul(self, n: f32) -> Self {
        let (r, g, b, a) = self.floats();
        Self::rgba_f32(r * n, g * n, b * n, a * n)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    #[inline]
    fn mul(self, color: Color) -> Color {
        let (r, g, b, a) = color.floats();
        Color::rgba_f32(r * self, g * self, b * self, a * self)
    }
}

impl Div<f32> for Color {
    type Output = Self;
    #[inline]
    fn div(self, n: f32) -> Self {
        let (r, g, b, a) = self.floats();
        Self::rgba_f32(r / n, g / n, b / n, a / n)
    }
}

impl Div<Color> for Color {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        let (r1, g1, b1, a1) = self.floats();
        let (r2, g2, b2, a2) = other.floats();
        Self::rgba_f32(r1 / r2, g1 / g2, b1 / b2, a1 / a2)
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
