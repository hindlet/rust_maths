use rust_maths::*;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut max = f32::MIN;
    let mut min = f32::MAX;

    // runs a lot of simplex noise to find the max and min
    for _ in 0..1000000 {
        let val = simplex2d(rng.gen(), rng.gen());
        if val > max {max = val}
        if val < min {min = val}
    }

    println!("Max val was: {} and min val was: {}", max, min);
}