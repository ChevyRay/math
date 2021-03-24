use crate::{int2, Int2};
use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Sub};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Default, Copy, Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct IntRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[inline]
pub fn irect(x: i32, y: i32, w: i32, h: i32) -> IntRect {
    IntRect { x, y, w, h }
}

impl IntRect {
    /// An empty (zeroed) rectangle.
    pub const EMPTY: Self = Self {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    };

    /// Create a new rectangle.
    #[inline]
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        irect(x, y, w, h)
    }

    /// Create a rectangle centered on the position with the provided size.
    #[inline]
    pub fn centered(center: Int2, w: i32, h: i32) -> Self {
        irect(center.x - w / 2, center.y - h / 2, w, h)
    }

    /// Create a rectangle with a width and height.
    #[inline]
    pub fn of_size(w: i32, h: i32) -> Self {
        irect(0, 0, w, h)
    }

    /// Return a version of the rectangle that is guaranteed not to have a negative size.
    #[inline]
    pub fn absolute(&self) -> Self {
        let mut r = *self;
        if r.w < 0 {
            r.x += r.w;
            r.w = -r.w;
        }
        if r.h < 0 {
            r.y += r.h;
            r.h = -r.h;
        }
        r
    }

    /// If the width *or* height of the rectangle is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    #[inline]
    pub fn min_x(&self) -> i32 {
        self.x.min(self.right())
    }

    #[inline]
    pub fn max_x(&self) -> i32 {
        self.x.max(self.right())
    }

    #[inline]
    pub fn min_y(&self) -> i32 {
        self.y.min(self.bottom())
    }

    #[inline]
    pub fn max_y(&self) -> i32 {
        self.y.max(self.bottom())
    }

    #[inline]
    pub fn center_x(&self) -> i32 {
        self.x + self.w / 2
    }

    #[inline]
    pub fn center_y(&self) -> i32 {
        self.y + self.h / 2
    }

    #[inline]
    pub fn top_left(&self) -> Int2 {
        int2(self.left(), self.top())
    }

    #[inline]
    pub fn top_right(&self) -> Int2 {
        int2(self.right(), self.top())
    }

    #[inline]
    pub fn bottom_right(&self) -> Int2 {
        int2(self.right(), self.bottom())
    }

    #[inline]
    pub fn bottom_left(&self) -> Int2 {
        int2(self.left(), self.bottom())
    }

    #[inline]
    pub fn size(&self) -> Int2 {
        int2(self.w, self.h)
    }

    #[inline]
    pub fn min(&self) -> Int2 {
        int2(self.min_x(), self.min_y())
    }

    #[inline]
    pub fn max(&self) -> Int2 {
        int2(self.max_x(), self.max_y())
    }

    #[inline]
    pub fn center(&self) -> Int2 {
        int2(self.center_x(), self.center_y())
    }

    #[inline]
    pub fn top_center(&self) -> Int2 {
        int2(self.center_x(), self.top())
    }

    #[inline]
    pub fn bottom_center(&self) -> Int2 {
        int2(self.center_x(), self.bottom())
    }

    #[inline]
    pub fn left_center(&self) -> Int2 {
        int2(self.left(), self.center_y())
    }

    #[inline]
    pub fn right_center(&self) -> Int2 {
        int2(self.right(), self.center_y())
    }

    #[inline]
    pub fn area(&self) -> i32 {
        self.w * self.h
    }

    #[inline]
    pub fn perimeter(&self) -> i32 {
        self.w * 2 + self.h * 2
    }

    #[inline]
    pub fn contains(&self, p: Int2) -> bool {
        p.x >= self.x && p.y >= self.y && p.x < self.right() && p.y < self.bottom()
    }

    #[inline]
    pub fn contains_rect(&self, r: &Self) -> bool {
        r.x >= self.x && r.y >= self.y && r.right() <= self.right() && r.bottom() <= self.bottom()
    }

    #[inline]
    pub fn clamp_point(&self, p: Int2) -> Int2 {
        int2(
            p.x.clamp(self.min_x(), self.max_x()),
            p.y.clamp(self.min_y(), self.max_y()),
        )
    }

    #[inline]
    pub fn overlaps(&self, r: &Self) -> bool {
        self.x < r.right() && self.y < r.bottom() && self.right() > r.x && self.bottom() > r.y
    }

    #[inline]
    pub fn overlap(&self, r: &Self) -> Option<Self> {
        let min = self.min().max(r.min());
        let max = self.max().min(r.max());
        if max.x > min.x && max.y > min.y {
            Some(irect(min.x, min.y, max.x - min.x, max.y - min.y))
        } else {
            None
        }
    }

    #[inline]
    pub fn scale_to_fit(&self, outer: &Self) -> Self {
        let s = ((outer.w as f32) / (self.w as f32)).min((outer.h as f32) / (self.h as f32));
        let w = (self.w as f32 * s) as i32;
        let h = (self.h as f32 * s) as i32;
        irect((outer.w - w) / 2, (outer.h - h) / 2, w, h)
    }

    #[inline]
    pub fn conflate(&self, r: &Self) -> Self {
        let x = self.min_x().min(r.min_x());
        let y = self.min_y().min(r.min_y());
        let w = self.max_x().max(r.max_x());
        let h = self.max_y().max(r.max_y());
        irect(x, y, w - x, h - y)
    }

    #[inline]
    pub fn translate(&self, amount: Int2) -> Self {
        irect(self.x + amount.x, self.y + amount.y, self.w, self.h)
    }

    #[inline]
    pub fn inflate(&self, w: i32, h: i32) -> Self {
        irect(self.x - w / 2, self.y - h / 2, self.w + w, self.h + h)
    }

    #[inline]
    pub fn non_neg(&self) -> Self {
        let mut r = *self;
        if r.w < 0 {
            r.x += r.w;
            r.w = -r.w;
        }
        if r.h < 0 {
            r.y += r.h;
            r.h = -r.h;
        }
        r
    }

    #[inline]
    pub fn iter(&self) -> IntRectIter {
        IntRectIter {
            min_x: self.min_x(),
            max_x: self.max_x(),
            max_y: self.max_y(),
            pos: Int2::ZERO,
        }
    }
}

impl IntoIterator for IntRect {
    type Item = Int2;
    type IntoIter = IntRectIter;

    fn into_iter(self) -> Self::IntoIter {
        IntRectIter {
            min_x: self.min_x(),
            max_x: self.max_x(),
            max_y: self.max_y(),
            pos: Int2::ZERO,
        }
    }
}

impl fmt::Display for IntRect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.x, self.y, self.w, self.h)
    }
}

impl Add<Int2> for IntRect {
    type Output = Self;
    fn add(self, val: Int2) -> IntRect {
        irect(self.x + val.x, self.y + val.y, self.w, self.h)
    }
}

impl Sub<Int2> for IntRect {
    type Output = Self;
    fn sub(self, val: Int2) -> IntRect {
        irect(self.x - val.x, self.y - val.y, self.w, self.h)
    }
}

pub struct IntRectIter {
    min_x: i32,
    max_x: i32,
    max_y: i32,
    pos: Int2,
}

impl Iterator for IntRectIter {
    type Item = Int2;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y < self.max_y {
            let p = self.pos;
            self.pos.x += 1;
            if self.pos.x >= self.max_x {
                self.pos.x = self.min_x;
                self.pos.y += 1;
            }
            Some(p)
        } else {
            None
        }
    }
}
