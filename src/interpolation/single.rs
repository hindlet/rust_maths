use std::ops::{Mul, Sub, Add, Div};


/// Lerp, linearly interpolates between the two values and gives an output
/// - The position values are all clamped between 0 and 1
pub fn lerp<T>(min: T, max: T, position: f32) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
{
    let pos = position.clamp(0.0, 1.0);
    min * (1.0 - pos) + max * pos
}

/// The same as lerp, but passes input values through the given function first, allowing fancier interpolation
pub fn interp_by_fn<T>(min: T, max: T, position: f32, func: fn(f32) -> f32) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
{
    lerp(min, max, func(position))
}

/// Returns the position value that could be used to lerp between the two values, result is clamped to 0.0..=1.0 so values outisde the the values will return either 1 or 0
pub fn inverse_lerp<T>(min: T, max: T, value: T) -> f32
where
    T: Div<Output = f32>,
    T: Sub<Output = T>,
    T: Copy
{
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

