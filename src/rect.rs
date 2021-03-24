use crate::{vec2, IntRect, Vec2};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Sub, AddAssign};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
}

impl Rect {
    pub const EMPTY: Self = Self {
        x: 0.0,
        y: 0.0,
        w: 0.0,
        h: 0.0,
    };

    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        rect(x, y, w, h)
    }

    pub fn centered(center: Vec2, w: f32, h: f32) -> Self {
        rect(center.x - w * 0.5, center.y - h * 0.5, w, h)
    }

    pub fn of_size(w: f32, h: f32) -> Self {
        rect(0.0, 0.0, w, h)
    }

    pub fn size(&self) -> Vec2 {
        vec2(self.w, self.h)
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    pub fn min_x(&self) -> f32 {
        self.x.min(self.right())
    }

    pub fn max_x(&self) -> f32 {
        self.x.max(self.right())
    }

    pub fn min_y(&self) -> f32 {
        self.y.min(self.bottom())
    }

    pub fn max_y(&self) -> f32 {
        self.y.max(self.bottom())
    }

    pub fn center_x(&self) -> f32 {
        self.x + self.w * 0.5
    }

    pub fn center_y(&self) -> f32 {
        self.y + self.h * 0.5
    }

    pub fn top_left(&self) -> Vec2 {
        vec2(self.left(), self.top())
    }

    pub fn top_right(&self) -> Vec2 {
        vec2(self.right(), self.top())
    }

    pub fn bottom_right(&self) -> Vec2 {
        vec2(self.right(), self.bottom())
    }

    pub fn bottom_left(&self) -> Vec2 {
        vec2(self.left(), self.bottom())
    }

    pub fn min(&self) -> Vec2 {
        vec2(self.min_x(), self.min_y())
    }

    pub fn max(&self) -> Vec2 {
        vec2(self.max_x(), self.max_y())
    }

    pub fn center(&self) -> Vec2 {
        vec2(self.center_x(), self.center_y())
    }

    pub fn top_center(&self) -> Vec2 {
        vec2(self.center_x(), self.top())
    }

    pub fn bottom_center(&self) -> Vec2 {
        vec2(self.center_x(), self.bottom())
    }

    pub fn left_center(&self) -> Vec2 {
        vec2(self.left(), self.center_y())
    }

    pub fn right_center(&self) -> Vec2 {
        vec2(self.right(), self.center_y())
    }

    pub fn area(&self) -> f32 {
        self.w * self.h
    }

    pub fn perimeter(&self) -> f32 {
        self.w * 2.0 + self.h * 2.0
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.x && p.y >= self.y && p.x < self.right() && p.y < self.bottom()
    }

    pub fn contains_rect(&self, r: &Rect) -> bool {
        r.x >= self.x && r.y >= self.y && r.right() <= self.right() && r.bottom() <= self.bottom()
    }

    pub fn overlaps(&self, r: &Self) -> bool {
        self.x < r.right() && self.y < r.bottom() && self.right() > r.x && self.bottom() > r.y
    }

    pub fn overlap(&self, r: &Self) -> Option<Self> {
        let min = self.top_left().max(r.top_left());
        let max = self.bottom_right().min(r.bottom_right());
        if max.x > min.x && max.y > min.y {
            Some(rect(min.x, min.y, max.x - min.x, max.y - min.y))
        } else {
            None
        }
    }

    pub fn scale_to_fit(&self, outer: &Self) -> Self {
        let s = (outer.w / self.w).min(outer.h / self.h);
        let w = self.w * s;
        let h = self.h * s;
        rect((outer.w - w) * 0.5, (outer.h - h) * 0.5, w, h)
    }

    pub fn conflate(&self, r: &Self) -> Self {
        let x = self.min_x().min(r.min_x());
        let y = self.min_y().min(r.min_y());
        let w = self.max_x().max(r.max_x());
        let h = self.max_y().max(r.max_y());
        rect(x, y, w - x, h - y)
    }

    pub fn translate(&self, amount: Vec2) -> Self {
        rect(self.x + amount.x, self.y + amount.y, self.w, self.h)
    }

    pub fn inflate(&self, w: f32, h: f32) -> Self {
        rect(self.x - w * 0.5, self.y - h * 0.5, self.w + w, self.h + h)
    }

    pub fn non_neg(&self) -> Self {
        let mut r = *self;
        if r.w < 0.0 {
            r.x += r.w;
            r.w = -r.w;
        }
        if r.h < 0.0 {
            r.y += r.h;
            r.h = -r.h;
        }
        r
    }
}

impl AsRef<[f32]> for Rect {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const f32, 4) }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.x, self.y, self.w, self.h)
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.w.eq(&other.w) && self.h.eq(&other.h)
    }
}

impl Hash for Rect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(crate::hash_f32(self.x));
        state.write_i32(crate::hash_f32(self.y));
        state.write_i32(crate::hash_f32(self.w));
        state.write_i32(crate::hash_f32(self.h));
    }
}

impl From<(f32, f32, f32, f32)> for Rect {
    fn from(val: (f32, f32, f32, f32)) -> Self {
        rect(val.0, val.1, val.2, val.3)
    }
}

impl From<Rect> for (f32, f32, f32, f32) {
    fn from(val: Rect) -> Self {
        (val.x, val.y, val.w, val.h)
    }
}

impl From<IntRect> for Rect {
    fn from(val: IntRect) -> Self {
        rect(val.x as f32, val.y as f32, val.w as f32, val.h as f32)
    }
}

impl Add<Vec2> for Rect {
    type Output = Self;
    fn add(self, val: Vec2) -> Rect {
        rect(self.x + val.x, self.y + val.y, self.w, self.h)
    }
}

impl AddAssign<Vec2> for Rect {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = self.add(rhs);
    }
}

impl Sub<Vec2> for Rect {
    type Output = Self;
    fn sub(self, val: Vec2) -> Rect {
        rect(self.x - val.x, self.y - val.y, self.w, self.h)
    }
}

impl Mul<f32> for Rect {
    type Output = Self;
    fn mul(self, val: f32) -> Self {
        rect(self.x * val, self.y * val, self.w * val, self.h * val)
    }
}

impl Div<f32> for Rect {
    type Output = Self;
    fn div(self, val: f32) -> Self {
        rect(self.x / val, self.y / val, self.w / val, self.h / val)
    }
}
