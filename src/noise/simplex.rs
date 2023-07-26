use super::Vector3;

pub fn grad3(index: i32) -> Vector3{

    match index % 12 {
        0 => [1, 1, 0].into(),
        1 => [-1, 1, 0].into(),
        2  => [1, -1, 0].into(),
        3 => [-1, -1, 0].into(),
        4 => [1, 0, 1].into(),
        5 => [-1, 0, 1].into(),
        6 => [1, 0, -1].into(),
        7 => [-1, 0, -1].into(),
        8 => [0, 1, 1].into(),
        9 => [0, -1, 1].into(),
        10 => [0, 1, -1].into(),
        11 => [0, -1, -1].into(),
        _ => panic!("Could not get gradient")
    }
}

// const GRAD4: [Vector4; 32] = [
//     Vector4{x: 0.0, y: 1.0, z: 1.0, w: 1.0},
//     Vector4{x: 0.0, y: 1.0, z: 1.0, w: -1.0},
//     Vector4{x: 0.0, y: 1.0, z: -1.0, w: 1.0},
//     Vector4{x: 0.0, y: 1.0, z: -1.0, w: -1.0},
//     Vector4{x: 0.0, y: -1.0, z: 1.0, w: 1.0},
//     Vector4{x: 0.0, y: -1.0, z: 1.0, w: -1.0},
//     Vector4{x: 0.0, y: -1.0, z: -1.0, w: 1.0},
//     Vector4{x: 0.0, y: -1.0, z: -1.0, w: -1.0},
//     Vector4{x: 1.0, y: 0.0, z: 1.0, w: 1.0},
//     Vector4{x: 1.0, y: 0.0, z: 1.0, w: -1.0},
//     Vector4{x: 1.0, y: 0.0, z: -1.0, w: 1.0},
//     Vector4{x: 1.0, y: 0.0, z: -1.0, w: -1.0},
//     Vector4{x: -1.0, y: 0.0, z: 1.0, w: 1.0},
//     Vector4{x: -1.0, y: 0.0, z: 1.0, w: -1.0},
//     Vector4{x: -1.0, y: 0.0, z: -1.0, w: 1.0},
//     Vector4{x: -1.0, y: 0.0, z: -1.0, w: -1.0},
//     Vector4{x: 1.0, y: 1.0, z: 0.0, w: 1.0},
//     Vector4{x: 1.0, y: 1.0, z: 0.0, w: -1.0},
//     Vector4{x: 1.0, y: -1.0, z: 0.0, w: 1.0},
//     Vector4{x: 1.0, y: -1.0, z: 0.0, w: -1.0},
//     Vector4{x: -1.0, y: 1.0, z: 0.0, w: 1.0},
//     Vector4{x: -1.0, y: 1.0, z: 0.0, w: -1.0},
//     Vector4{x: -1.0, y: -1.0, z: 0.0, w: 1.0},
//     Vector4{x: -1.0, y: -1.0, z: 0.0, w: -1.0},
//     Vector4{x: 1.0, y: 1.0, z: 1.0, w: 0.0},
//     Vector4{x: 1.0, y: 1.0, z: -1.0, w: 0.0},
//     Vector4{x: 1.0, y: -1.0, z: 1.0, w: 0.0},
//     Vector4{x: 1.0, y: -1.0, z: -1.0, w: 0.0},
//     Vector4{x: -1.0, y: 1.0, z: 1.0, w: 0.0},
//     Vector4{x: -1.0, y: 1.0, z: -1.0, w: 0.0},
//     Vector4{x: -1.0, y: -1.0, z: 1.0, w: 0.0},
//     Vector4{x: -1.0, y: -1.0, z: -1.0, w: 0.0},
// ];

const PERM: [i32; 512] = [
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
    138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,

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

fn hash(hash: i32) -> i32 {
    PERM[hash as usize]
}

/// 2d simplex noise function, takes in an x and y value and outputs a value between -1 and 1
pub fn simplex2d(x: f32, y: f32) -> f32 {
    let (n0, n1, n2): (f32, f32, f32);

    let skew = skew_val(2);
    let unskew = unskew_val(2);

    // skew to get simplex cell coords
    let s = (x + y) * skew;
    let xs = x + s;
    let ys = y + s;
    let i: i32 = xs.floor() as i32;
    let j: i32 = ys.floor() as i32;

    // unskew to get those coords in 2d space
    let t = (i + j) as f32 * unskew;
    let x0_1 = i as f32 - t;
    let y0_1 = j as f32 - t;
    let x0 = x - x0_1;
    let y0 = y - y0_1;

    // figure out which simplex tri we're in
    let (i1, j1): (i32, i32);
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
    let ii = i & 255;
    let jj = j & 255;
    let gi0 = hash(ii + hash(jj));
    let gi1 = hash(ii + i1 + hash(jj + j1));
    let gi2 = hash(ii + 1 + hash(jj + 1));


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

    45.23065 * (n0 + n1 + n2)
}

/// 3d simplex noise function, takes in an x, y and z value and outputs a value between -1 and 1
pub fn simplex3d(x: f32, y: f32, z: f32) -> f32 {
    let (n0, n1, n2, n3): (f32, f32, f32, f32);

    let skew = skew_val(3);
    let unskew = unskew_val(3);

    // skew for simplex cell coords
    let s = (x + y + z) * skew;
    let i = (x + s).floor() as i32;
    let j = (y + s).floor() as i32;
    let k = (z + s).floor() as i32;

    // unskew for for 3d space coords
    let t = (i + j + k) as f32 * unskew;
    let x0_1 = i as f32 - t;
    let y0_1 = j as f32 - t;
    let z0_1 = k as f32 - t;
    let x0 = x - x0_1;
    let y0 = y - y0_1;
    let z0 = z - z0_1;

    // figure out which simplex tetrahedron we're in
    let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
        if y0 >= z0 { // x y z
            (1, 0, 0, 1, 1, 0)
        } else if x0 >= z0 { // x z y
            (1, 0, 0, 1, 0 ,1)
        } else { // z x y
            (0, 0, 1, 1, 0, 1)
        }
    } else {
        if y0 < z0 { // z y x
            (0, 0, 1, 0, 1, 1)
        } else if x0 < z0 { // y z x
            (0, 1, 0, 0, 1, 1)
        } else { // y x z
            (0, 1, 0, 1, 1, 0)
        }
    };

    let x1 = x0 - i1 as f32 + unskew; // second corner corner
    let y1 = y0 - j1 as f32 + unskew;
    let z1 = z0 - k1 as f32 + unskew;
    let x2 = x0 - i2 as f32 + 2.0 * unskew; // third corner
    let y2 = y0 - j2 as f32 + 2.0 * unskew;
    let z2 = z0 - k2 as f32 + 2.0 * unskew;
    let x3 = x0 - 1.0 + 3.0 * unskew; // last corner
    let y3 = y0 - 1.0 + 3.0 * unskew;
    let z3 = z0 - 1.0 + 3.0 * unskew;

    // hased indices
    let ii = i & 255;
    let jj = j & 255;
    let kk = k & 255;
    let gi0 = hash(ii + hash(jj + hash(kk)));
    let gi1 = hash(ii + i1 + hash(jj + j1 + hash(kk + k1)));
    let gi2 = hash(ii + i2 + hash(jj + j2 + hash(kk + k2)));
    let gi3 = hash(ii + 1 + hash(jj + 1 + hash(kk + 1)));

    // calculate corner contribution
    let mut t0 = 0.5 - x0 * x0 - y0 * y0 - z0 * z0;
    if t0 < 0.0 {n0 = 0.0}
    else {
        t0 *= t0;
        n0 = t0 * t0 * grad3(gi0).dot([x0, y0, z0]);
    }

    let mut t1 = 0.5 - x1 * x1 - y1 * y1 - z1 * z1;
    if t1 < 0.0 {n1 = 0.0}
    else {
        t1 *= t1;
        n1 = t1 * t1 * grad3(gi1).dot([x1, y1, z1]);
    }

    let mut t2 = 0.5 - x2 * x2 - y2 * y2 - z2 * z2;
    if t2 < 0.0 {n2 = 0.0}
    else {
        t2 *= t2;
        n2 = t2 * t2 * grad3(gi2).dot([x2, y2, z2]);
    }

    let mut t3 = 0.5 - x3 * x3 - y3 * y3 - z3 * z3;
    if t3 < 0.0 {n3 = 0.0}
    else {
        t3 *= t3;
        n3 = t3 * t3 * grad3(gi3).dot([x3, y3, z3]);
    }


    32.0 * (n0 + n1 + n2 + n3)
}