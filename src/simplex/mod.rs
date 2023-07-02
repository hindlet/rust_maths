use super::{Vector4, Vector3};

pub fn grad3(index: u32) -> Vector3{

    const DIAG: f32 = std::f32::consts::FRAC_1_SQRT_2;
    const DIAG2: f32 = 0.577_350_2;

    match index % 32 {
        0 | 12 => [DIAG, DIAG, 0.0].into(),
        1 | 13 => [-DIAG, DIAG, 0.0].into(),
        2 | 14 => [DIAG, -DIAG, 0.0].into(),
        3 | 15 => [-DIAG, -DIAG, 0.0].into(),
        4 | 16 => [DIAG, 0.0, DIAG].into(),
        5 | 17 => [-DIAG, 0.0, DIAG].into(),
        6 | 18 => [DIAG, 0.0, -DIAG].into(),
        7 | 19 => [-DIAG, 0.0, -DIAG].into(),
        8 | 20 => [0.0, DIAG, DIAG].into(),
        9 | 21 => [0.0, -DIAG, DIAG].into(),
        10 | 22 => [0.0, DIAG, -DIAG].into(),
        11 | 23 => [0.0, -DIAG, -DIAG].into(),
        24 => [DIAG2, DIAG2, DIAG2].into(),
        25 => [-DIAG2, DIAG2, DIAG2].into(),
        26 => [DIAG2, -DIAG2, DIAG2].into(),
        27 => [-DIAG2, -DIAG2, DIAG].into(),
        28 => [DIAG2, DIAG2, -DIAG2].into(),
        29 => [-DIAG2, DIAG2, -DIAG2].into(),
        30 => [DIAG2, -DIAG2, -DIAG2].into(),
        31 => [-DIAG2, -DIAG2, -DIAG2].into(),
        _ => panic!("Could not get gradient")
    }
}

// fn grad3(hash: u32, x: f32, y: f32) -> f32{
//     let h = hash & 0x3F;
//     let u = if h < 4 {x} else {y};
//     let v = if h < 4 {y} else {x};
//     let mut out = if h & 1 == 1 {-u} else {u};
//     if h & 2 == 2 {out += -2.0 * v} else {out += 2.0 * v};
//     out
// }

const GRAD4: [Vector4; 32] = [
    Vector4{x: 0.0, y: 1.0, z: 1.0, w: 1.0},
    Vector4{x: 0.0, y: 1.0, z: 1.0, w: -1.0},
    Vector4{x: 0.0, y: 1.0, z: -1.0, w: 1.0},
    Vector4{x: 0.0, y: 1.0, z: -1.0, w: -1.0},
    Vector4{x: 0.0, y: -1.0, z: 1.0, w: 1.0},
    Vector4{x: 0.0, y: -1.0, z: 1.0, w: -1.0},
    Vector4{x: 0.0, y: -1.0, z: -1.0, w: 1.0},
    Vector4{x: 0.0, y: -1.0, z: -1.0, w: -1.0},
    Vector4{x: 1.0, y: 0.0, z: 1.0, w: 1.0},
    Vector4{x: 1.0, y: 0.0, z: 1.0, w: -1.0},
    Vector4{x: 1.0, y: 0.0, z: -1.0, w: 1.0},
    Vector4{x: 1.0, y: 0.0, z: -1.0, w: -1.0},
    Vector4{x: -1.0, y: 0.0, z: 1.0, w: 1.0},
    Vector4{x: -1.0, y: 0.0, z: 1.0, w: -1.0},
    Vector4{x: -1.0, y: 0.0, z: -1.0, w: 1.0},
    Vector4{x: -1.0, y: 0.0, z: -1.0, w: -1.0},
    Vector4{x: 1.0, y: 1.0, z: 0.0, w: 1.0},
    Vector4{x: 1.0, y: 1.0, z: 0.0, w: -1.0},
    Vector4{x: 1.0, y: -1.0, z: 0.0, w: 1.0},
    Vector4{x: 1.0, y: -1.0, z: 0.0, w: -1.0},
    Vector4{x: -1.0, y: 1.0, z: 0.0, w: 1.0},
    Vector4{x: -1.0, y: 1.0, z: 0.0, w: -1.0},
    Vector4{x: -1.0, y: -1.0, z: 0.0, w: 1.0},
    Vector4{x: -1.0, y: -1.0, z: 0.0, w: -1.0},
    Vector4{x: 1.0, y: 1.0, z: 1.0, w: 0.0},
    Vector4{x: 1.0, y: 1.0, z: -1.0, w: 0.0},
    Vector4{x: 1.0, y: -1.0, z: 1.0, w: 0.0},
    Vector4{x: 1.0, y: -1.0, z: -1.0, w: 0.0},
    Vector4{x: -1.0, y: 1.0, z: 1.0, w: 0.0},
    Vector4{x: -1.0, y: 1.0, z: -1.0, w: 0.0},
    Vector4{x: -1.0, y: -1.0, z: 1.0, w: 0.0},
    Vector4{x: -1.0, y: -1.0, z: -1.0, w: 0.0},
];

const PERM: [u32; 256] = [
    151, 160, 137, 91, 90, 15,
    131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23,
    190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33,
    88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166,
    77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244,
    102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196,
    135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123,
    5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42,
    223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9,
    129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228,
    251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107,
    49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
    138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180
];



fn skew_val(dimension: u32) -> f32{
    let n = dimension as f32;
    ((n + 1.0).sqrt() - 1.0) / n
}

fn unskew_val(dimension: u32) -> f32 {
    let n = dimension as f32;
    (1.0 - (1.0 / (n + 1.0).sqrt())) / n
}

// fn hash(to_hash: &[isize]) -> usize{
//     let index = to_hash
//         .iter()
//         .map(|&a| (a & 0xff) as usize)
//         .reduce(|a, b| PERM[a] ^ b)
//         .unwrap();
//     PERM[index]
// }

fn hash(hash: u32) -> u32 {
    PERM[(hash & 255) as usize]
}


pub fn simplex2d(x: f32, y: f32) -> f32 {
    let (n0, n1, n2): (f32, f32, f32);

    let skew = skew_val(2);
    let unskew = unskew_val(2);

    // skew to get simplex cell coords
    let s = (x + y) * skew;
    let xs = x + s;
    let ys = y + s;
    let i: u32 = xs.floor() as u32;
    let j: u32 = ys.floor() as u32;

    // unskew to get those coords in 2d space
    let t = (i + j) as f32 * unskew;
    let x0_1 = i as f32 - t;
    let y0_1 = j as f32 - t;
    let x0 = x - x0_1;
    let y0 = y - y0_1;

    // figure out which simplex tri we're in
    let (i1, j1): (u32, u32);
    if x0 > y0 {
        i1 = 1;
        j1 = 0;
    } else {
        i1 = 0;
        j1 = 1;
    }

    // corner offsets in 2d space
    let x1 = x0 - i1 as f32 + unskew;
    let y1 = y0 - j1 as f32 + unskew;
    let x2 = x0 - 1.0 + 2.0 * unskew;
    let y2 = y0 - 1.0 + 2.0 * unskew;

    // get hashed gradient indices
    let gi0 = hash(i + hash(j)) % 12;
    let gi1 = hash(i + i1 + hash(j + j1)) % 12;
    let gi2 = hash(i + 1 + hash(j + 1)) % 12;


    let mut t0 = 0.5 - x0 * x0 - y0 * y0;
    if t0 < 0.0 {
        n0 = 0.0;
    } else {
        t0 *= t0;
        n0 = t0 * t0 * grad3(gi0).xy().dot([x0, y0]);
    }

    let mut t1 = 0.5 - x1 * x1 - y1 * y1;
    if t1 < 0.0 {
        n1 = 0.0;
    } else {
        t1 *= t1;
        n1 = t1 * t1 * grad3(gi1).xy().dot([x1, y1]);
    }

    let mut t2 = 0.5 - x2 * x2 - y2 * y2;
    if t2 < 0.0 {
        n2 = 0.0;
    } else {
        t2 *= t2;
        n2 = t2 * t2 * grad3(gi2).xy().dot([x2, y2]);
    }

    (45.23065 * (n0 + n1 + n2) + 1.0) / 2.0 
}