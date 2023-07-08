use rust_maths::*;

fn main() {
    let mut rng = thread_rng();

    let mut max = f32::MIN;
    let mut min = f32::MAX;

    // runs a lot of simplex noise to find the max and min
    for _ in 0..200 {
        let (x, y) = (rng.gen::<f32>() * 10000.0, rng.gen::<f32>() * 10000.0);
        let val = simplex2d(x, y);
        println!("{}, {}: {}", x, y, val);
        if val > max {max = val}
        if val < min {min = val}
    }

    println!("Max val was: {} and min val was: {}", max, min);
}