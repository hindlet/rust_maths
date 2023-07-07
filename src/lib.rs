// vectors
mod vectors;
pub use vectors::*;
// matrices
mod matrices;
pub use matrices::*;
// colliders
mod colliders;
pub use colliders::*;
// noise
mod simplex;
pub use simplex::*;
// interpolation
mod interpolation;
pub use interpolation::*;

use std::f32::consts::{FRAC_2_SQRT_PI, SQRT_2, PI};
pub use rand_chacha::{ChaChaRng, rand_core::SeedableRng};
pub use rand::Rng;


const ROOT_TWOPI: f32 = SQRT_2 * (FRAC_2_SQRT_PI * PI / 2.0); // sqrt(2.0 * PI)

pub fn normal_probabilty_density(value: f32, mean: f32, standard_deviation: f32) -> f32 {

    (1.0 / (standard_deviation * ROOT_TWOPI)) * (-1.0 * (value - mean) * (value - mean) / (2.0 * standard_deviation * standard_deviation) ).exp()
}


const P: f32 = 0.47047;
const A1: f32 = 0.3480242;
const A2: f32 = -0.0958798;
const A3: f32 = 0.7478556;

/// an approxomation of the error function from Abramowitz and Stegun (equation 7.1.26), it has a maximum error of 2.5x10^-5
fn error_fn_approx(value: f32) -> f32 {
    let t = 1.0 / (1.0 + P * value);
    1.0 - (A1 * t + A2 * t * t + A3 * t * t * t) * (t * t * -1.0).exp()
}


/// cumulative normal distribution fn, maximum error of 2.5x10^-5
pub fn normal_cmd(value: f32, mean: f32, standard_deviation: f32) -> f32 {
    let x = (value - mean) / (standard_deviation * SQRT_2);

    if x == 0.0 {return 0.5;}
    if x < 0.0 {return 0.5 * (1.0 - error_fn_approx(-x));}
    else {return 0.5 * (1.0 + error_fn_approx(x))}
}



/// sorts the given values list by their accosiated f32 in ascending order
pub fn quicksort<T>(list: Vec<(f32, T)>) -> Vec<(f32, T)>{
    if list.len() <= 1 {return list;}

    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut more = Vec::new();

    let target_val = list[list.len() / 2].0;

    for item in list {
        if item.0 < target_val {less.push(item)}
        else if item.0 > target_val {more.push(item)}
        else {equal.push(item)}
    }

    let mut sorted = quicksort(less);
    sorted.append(&mut equal);
    sorted.append(&mut quicksort(more));
    sorted
}


/// solves a quadratic and returns any real solutions
/// quadric in the form ax^2 + bx + c = 0
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>){
    let det = b * b - 4.0 * a * c;
    if det < 0.0 {return (None, None);}
    else if det == 0.0 {
        let s0 = -b / (2.0 * a);
        return (Some(s0), None);
    }
    else {
        let (s1, s2) = ((-b + det.sqrt()) / (2.0 * a),(-b - det.sqrt()) / (2.0 * a));
        return (Some(s1), Some(s2));
    }
}