use std::f32::consts::PI;
use std::hash::{Hash, Hasher};

/// An angle in radians.
#[repr(C)]
#[derive(Default, Copy, Clone, PartialOrd)]
pub struct Radians(pub f32);

/// An angle in degrees.
#[repr(C)]
#[derive(Default, Copy, Clone, PartialOrd)]
pub struct Degrees(pub f32);

/// Construct an angle in radians.
#[inline]
pub fn rad(val: f32) -> Radians {
    Radians(val)
}

/// Construct an angle in degrees.
#[inline]
pub fn deg(val: f32) -> Degrees {
    Degrees(val)
}

impl From<Degrees> for Radians {
    #[inline]
    fn from(val: Degrees) -> Self {
        Self(val.0 * (PI / 180.0))
    }
}

impl From<Radians> for Degrees {
    #[inline]
    fn from(val: Radians) -> Self {
        Self(val.0 * (180.0 / PI))
    }
}

impl From<f32> for Radians {
    #[inline]
    fn from(val: f32) -> Self {
        Self(val)
    }
}

impl From<Radians> for f32 {
    #[inline]
    fn from(val: Radians) -> Self {
        val.0
    }
}

impl Radians {
    /// Check if two angles are approximately equal.
    #[inline]
    pub fn approx<T: Into<Radians>>(self, other: T) -> bool {
        crate::approx_f32(self.0, other.into().0)
    }
}

impl Degrees {
    /// Check if two angles are approximately equal.
    #[inline]
    pub fn approx<T: Into<Radians>>(self, other: T) -> bool {
        other.into().approx(self)
    }
}

impl PartialEq for Radians {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Hash for Radians {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(crate::hash_f32(self.0));
    }
}

impl PartialEq for Degrees {
    fn eq(&self, other: &Self) -> bool {
        let a: Radians = (*self).into();
        let b: Radians = (*other).into();
        a.eq(&b)
    }
}

impl Hash for Degrees {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let radians: Radians = (*self).into();
        radians.hash(state);
    }
}
