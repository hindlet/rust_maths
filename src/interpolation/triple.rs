use std::ops::{Mul, Sub, Add};


/// Trilerp function, values should be in order:
/// (x, y, z), (x + 1, y, z), (x, y, z + 1), (x + 1, y, z + 1), (x, y + 1, z), (x + 1, y + 1, z), (x, y + 1, z + 1), (x + 1, y + 1, z + 1)
/// 
/// Where x, y and z are the three values used to either get a result from a function or sample from a list
/// - The position values are all clamped between 0 and 1
pub fn trilerp<T>(vals: [T; 8], position: (f32, f32, f32)) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
    T: Copy
{
    let (x, y, z) = (position.0.clamp(0.0, 1.0), position.1.clamp(0.0, 1.0), position.2.clamp(0.0, 1.0));

    vals[0] * (1.0 - x) * (1.0 - y) * (1.0 - z) +
    vals[1] * (x) * (1.0 - y) * (1.0 - z) +
    vals[2] * (1.0 - x) * (1.0 - y) * (z) +
    vals[3] * (x) * (1.0 - y) * (z) +
    vals[4] * (1.0 - x) * (y) * (1.0 - z) +
    vals[5] * (x) * (y) * (1.0 - z) +
    vals[6] * (1.0 - x) * (y) * (z) +
    vals[7] * (x) * (y) * (z) 
}

/// The same as trilerp but the position values given are put through the given function first, allowing fancier interpolation
pub fn tri_interp_by_fn<T>(vals: [T; 8], position: (f32, f32, f32), func: fn(f32) -> f32) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
    T: Copy
{
    trilerp(vals, (func(position.0), func(position.1), func(position.2 )))
}

/// The same as tri_interp_by_fn but the position values given are put through the given functions first in the order that matches them, for even fancier interpolation
pub fn tri_interp_by_triple_fn<T>(vals: [T; 8], position: (f32, f32, f32), x_func: fn(f32) -> f32, y_func: fn(f32) -> f32, z_func: fn(f32) -> f32) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
    T: Copy
{
    trilerp(vals, (x_func(position.0), y_func(position.1), z_func(position.2)))
}