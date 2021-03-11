pub use std::f32::consts::PI;
pub use std::f32::consts::SQRT_2;
pub use std::f32::consts::TAU;

#[inline]
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * (PI / 180.0)
}

#[inline]
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180.0 / PI)
}

#[inline]
pub fn sign(x: f32) -> f32 {
    match x {
        _ if x > 0.0 => 1.0,
        _ if x < 0.0 => -1.0,
        _ => 0.0,
    }
}

#[inline]
pub fn sign_i32(x: i32) -> i32 {
    match x {
        _ if x > 0 => 1,
        _ if x < 0 => -1,
        _ => 0,
    }
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
pub fn bezier3(a: f32, b: f32, c: f32, t: f32) -> f32 {
    a * (1.0 - t) * (1.0 - t) + b * 2.0 * (1.0 - t) * t + c * t * t
}

#[inline]
pub fn bezier4(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    t * t * t * (d + 3.0 * (b - c) - a) + 3.0 * t * t * (a - 2.0 * b + c) + 3.0 * t * (b - a) + a
}

#[inline]
pub fn hermite(p0: f32, m0: f32, p1: f32, m1: f32, t: f32) -> f32 {
    (2.0 * p0 - 2.0 * p1 + m1 + m0) * t * t * t
        + (3.0 * p1 - 3.0 * p0 - 2.0 * m0 - m1) * t * t
        + m0 * t
        + p0
}

#[inline]
pub fn catmull_rom(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    0.5 * (2.0 * b
        + (c - a) * t
        + (2.0 * a - 5.0 * b + 4.0 * c - d) * t * t
        + (3.0 * b - a - 3.0 * c + d) * t * t * t)
}

#[inline]
pub fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

#[inline]
pub fn sqr_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x = x1 - x2;
    let y = y1 - y2;
    x * x + y * y
}

#[inline]
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x = x1 - x2;
    let y = y1 - y2;
    (x * x + y * y).sqrt()
}

#[inline]
pub fn hash_f32(val: f32) -> i32 {
    unsafe {
        let p: *const f32 = &val;
        *(p as *const i32)
    }
}

/*#[inline]
pub fn clamp<T: Ord>(val: T, min: T, max: T) -> T {
    std::cmp::min(std::cmp::max(val, min), max)
}*/
