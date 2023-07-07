use std::ops::{Mul, Sub, Add};

/// Bilerp function, values should be in order:
/// (x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)
/// 
/// Where x and y are the two values used to either get a result from a function or sample from a list
/// - The position values are all clamped between 0 and 1
pub fn bilerp<T>(vals: [T; 4], position: (f32, f32)) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
    T: Copy
{
    let (x, y) = (position.0.clamp(0.0, 1.0), position.1.clamp(0.0, 1.0));

    vals[0] * (1.0 - x) * (1.0 - y) +
    vals[1] * (x) * (1.0 - y) +
    vals[2] * (1.0 - x) * (y) +
    vals[3] * (x) * (y)
}

/// The same as bilerp but the position values given are put through the given function first, allowing fancier interpolation
pub fn bi_interp_by_fn<T>(vals: [T; 4], position: (f32, f32), func: fn(f32) -> f32) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
    T: Copy
{
    bilerp(vals, (func(position.0.clamp(0.0, 1.0)), func(position.1.clamp(0.0, 1.0))))
}

/// The same as bi_interp_by_fn but the position values given are put through the given functions first in the order that matches them, for even fancier interpolation
pub fn bi_interp_by_double_fn<T>(vals: [T; 4], position: (f32, f32), x_func: fn(f32) -> f32, y_func: fn(f32) -> f32) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
    T: Copy
{
    bilerp(vals, (x_func(position.0.clamp(0.0, 1.0)), y_func(position.1.clamp(0.0, 1.0))))
}

// pub fn inverse_bilerp(min: f32, maxx: f32, maxy: f32, max: f32, value: f32) -> (f32, f32) {
//     let (mut x, mut y) = (0.5, 0.5);

//     for _ in 0..5 {
//         let residual = value - (min * (1.0 - x) * (1.0 - y) + maxx * x * (1.0 - y) + maxy * (1.0 - x) * y + max * x * y);

//         let jacob_matrix = Mat
//     }

//     (x, y)
// }