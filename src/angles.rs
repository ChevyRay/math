use std::f32::consts::PI;
use std::hash::{Hash, Hasher};

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Radians(pub f32);

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Degrees(pub f32);

#[inline]
pub fn rad(val: f32) -> Radians {
    Radians(val)
}

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
    #[inline]
    pub fn approx<T: Into<Radians>>(self, other: T) -> bool {
        crate::approx_f32(self.0, other.into().0)
    }
}

impl Degrees {
    #[inline]
    pub fn approx<T: Into<Radians>>(self, other: T) -> bool {
        other.into().approx(self)
    }
}

impl Hash for Radians {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(crate::hash_f32(self.0));
    }
}

impl Hash for Degrees {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let radians: Radians = (*self).into();
        radians.hash(state);
    }
}