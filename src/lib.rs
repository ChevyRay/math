#[allow(clippy::many_single_char_names)]
mod angles;
#[allow(clippy::many_single_char_names)]
mod approx;
#[allow(clippy::many_single_char_names)]
mod helper;
#[allow(clippy::many_single_char_names)]
mod int2;
#[allow(clippy::many_single_char_names)]
mod int3;
#[allow(clippy::many_single_char_names)]
mod int_rect;
#[allow(clippy::many_single_char_names)]
mod mat3x2;
#[allow(clippy::many_single_char_names)]
mod mat4x4;
#[allow(clippy::many_single_char_names)]
mod rect;
#[allow(clippy::many_single_char_names)]
mod vec2;
#[allow(clippy::many_single_char_names)]
mod vec3;
#[allow(clippy::many_single_char_names)]
mod vec4;
#[allow(clippy::many_single_char_names)]
mod color;

pub use crate::approx::{approx, approx_f32, Approx};
pub use angles::{deg, rad, Degrees, Radians};
pub use helper::*;
pub use color::Color;
pub use int2::{int2, Int2};
pub use int3::{int3, Int3};
pub use int_rect::{irect, IntRect, IntRectIter};
pub use mat3x2::{mat3x2, Mat3x2};
pub use mat4x4::{mat4x4, Mat4x4};
pub use rect::{rect, Rect};
pub use vec2::{vec2, Vec2};
pub use vec3::{vec3, Vec3};
pub use vec4::{vec4, Vec4};
