use super::{Vector3, Vector4, Vector2};

pub fn grad3(index: usize) -> Vector3{

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

const PERM: [usize; 256] = [
    151,160,137,91,90,15,
    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
    190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
    102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,
    135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,
    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167,43,172,9,
    129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,
    251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,
    49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
];

fn skew_val(dimension: u32) -> f32{
    let n = dimension as f32;
    ((n + 1.0).sqrt() - 1.0) / n
}

fn unskew_val(dimension: u32) -> f32 {
    let n = dimension as f32;
    (1.0 - (1.0 / (n + 1.0).sqrt())) / n
}

fn hash(to_hash: &[isize]) -> usize{
    let index = to_hash
        .iter()
        .map(|&a| (a & 0xff) as usize)
        .reduce(|a, b| PERM[a] ^ b)
        .unwrap();
    PERM[index]
}


pub fn simplex2d(x: f32, y: f32) -> f32 {
    let point = Vector2::new(x, y);

    let skew = skew_val(2);
    let unskew = unskew_val(2);

    // skew to get simplex cell coords
    let skew = Vector2::ONE * (point.sum() * skew);
    let cell = (point + skew).floor();

    // unskew to get those coords in 2d space
    let unskew = Vector2::ONE * (cell.sum() * unskew_val(2));
    let unskew_point = cell - unskew;

    // dist of cell coords from coord origin
    let offset1 = point - unskew_point;

    // figure out which simplex tri we're in
    let order = if offset1.x > offset1.y {Vector2::X} else {Vector2::Y};

    // corner offsets in 2d space
    let offset2 = offset1 - order + unskew;
    let offset3 = offset1 - Vector2::ONE + unskew * 2.0;

    // get hashed gradient indices
    // its fine to use as i32 as we floored those numbers earlier or we know they are whole intsw
    let gi0 = hash(&[cell.x as isize, cell.y as isize]);
    let gi1_cell = cell + order;
    let gi1 = hash(&[gi1_cell.x as isize, gi1_cell.y as isize]);
    let gi2 = hash(&[cell.x as isize + 1, cell.y as isize + 1]);

    // contributions from all three corners
    let mut noise_total = 0.0;

    let t0 = 1.0 - offset1.sqr_magnitude() * 2.0;
    if t0 > 0.0 {
        noise_total += (2.0 * (t0 * t0) + (t0 * t0 * t0 * t0)) * grad3(gi0).xy().dot(offset1)
    }

    let t1 = 1.0 - offset2.sqr_magnitude() * 2.0;
    if t1 >= 0.0 {
        noise_total += (2.0 * (t1 * t1) + (t1 * t1 * t1 * t1)) * grad3(gi1).xy().dot(offset2)
    }

    let t2 = 1.0 - offset3.sqr_magnitude() * 2.0;
    if t2 >= 0.0 {
        noise_total += (2.0 * (t2 * t2) + (t2 * t2 * t2 * t2)) * grad3(gi2).xy().dot(offset3)
    }

    noise_total
}