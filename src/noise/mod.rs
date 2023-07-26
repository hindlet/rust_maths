use super::{Vector3, lerp, interp_by_fn};

mod simplex;
pub use simplex::*;


pub fn selector_noise_2d(
    x: f32,
    y: f32,
    low_noise: fn(f32, f32) -> f32,
    high_noise: fn(f32, f32) -> f32,
    selector_noise: fn(f32, f32) -> f32,
    interp_fn: Option<fn(f32) -> f32>,
) -> f32 {
    if let Some(func) = interp_fn {
        return interp_by_fn(low_noise(x, y), high_noise(x, y), selector_noise(x, y), func);
    } else {
        return lerp(low_noise(x, y), high_noise(x, y), selector_noise(x, y));
    }
}

pub fn selector_noise_3d(
    x: f32,
    y: f32,
    z: f32,
    low_noise: fn(f32, f32, f32) -> f32,
    high_noise: fn(f32, f32, f32) -> f32,
    selector_noise: fn(f32, f32, f32) -> f32,
    interp_fn: Option<fn(f32) -> f32>,
) -> f32 {
    if let Some(func) = interp_fn {
        return interp_by_fn(low_noise(x, y, z), high_noise(x, y, z), selector_noise(x, y, z), func);
    } else {
        return lerp(low_noise(x, y, z), high_noise(x, y, z), selector_noise(x, y, z));
    }
}