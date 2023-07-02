use super::{Vector3, Vector4, Vector2};

const GRAD3: [Vector3; 12] = [
    Vector3{x: 1.0, y: 1.0, z: 0.0},
    Vector3{x: -1.0, y: 1.0, z: 0.0},
    Vector3{x: 1.0, y: -1.0, z: 0.0},
    Vector3{x: -1.0, y: -1.0, z: 0.0},
    Vector3{x: 1.0, y: 0.0, z: 1.0},
    Vector3{x: -1.0, y: 0.0, z: 1.0},
    Vector3{x: 1.0, y: 0.0, z: -1.0},
    Vector3{x: -1.0, y: 0.0, z: -1.0},
    Vector3{x: 0.0, y: 1.0, z: 1.0},
    Vector3{x: 0.0, y: -1.0, z: 1.0},
    Vector3{x: 0.0, y: 1.0, z: -1.0},
    Vector3{x: 0.0, y: -1.0, z: -1.0},
];

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

const PERM: [i32; 512] = [
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
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
];

fn skew_val(dimension: u32) -> f32{
    let n = dimension as f32;
    ((n + 1.0).sqrt() - 1.0) / n
}

fn unskew_val(dimension: u32) -> f32 {
    let n = dimension as f32;
    (1.0 - (1.0 / (n + 1.0).sqrt())) / n
}


pub fn simplex2d(x: f32, y: f32) -> f32 {
    let coords = Vector2::new(x, y);

    // skew to get simplex cell coords
    let skew = (coords.x + coords.y) * skew_val(2);
    let skew_coords = Vector2::new((x + skew).floor(), (y + skew).floor());

    // unskew to get those coords in 2d space
    let unskew = (skew_coords.x + skew_coords.y) * unskew_val(2);
    let unskew_coords = skew_coords - Vector2::ONE * unskew;

    // dist of cell coords from coord origin
    let dist_coords = coords - unskew_coords;

    // figure out which simplex tri we're in
    let mid_corner_tri = if dist_coords.x > dist_coords.y {Vector2::X} else {Vector2::Y};

    // corner offsets in 2d space
    let mid_corner = dist_coords -  mid_corner_tri + Vector2::ONE * unskew;
    let last_corner = dist_coords - Vector2::ONE + Vector2::ONE * 2.0 * unskew;

    // get hashed gradient indices
    // its fine to use as i32 as we floored those numbers earlier or we know they are whole intsw
    let i = (skew_coords.x as i32 & 255) as usize;
    let j = (skew_coords.y as i32 & 255) as usize;
    let gi0 = (PERM[i + PERM[j] as usize] % 12) as usize;
    let gi1 = (PERM[i + mid_corner_tri.x as usize + PERM[j + mid_corner_tri.y as usize] as usize] % 12) as usize;
    let gi2 = (PERM[i + 1 + PERM[j + 1] as usize] % 12) as usize;

    // contributions from all three corners
    let (mut contr_one, mut contr_two, mut contr_three) = (0.0, 0.0, 0.0);

    // I didn't use >= here as it would be zero anyway

    let t0 = 0.5 - dist_coords.dot(dist_coords);
    if t0 > 0.0 {
        contr_one = t0.powi(4) * GRAD3[gi0].xy().dot(dist_coords)
    }

    let t1 = 0.5 - mid_corner.dot(mid_corner);
    if t1 > 0.0 {
        contr_two = t1.powi(4) * GRAD3[gi1].xy().dot(mid_corner)
    }

    let t2 = 0.5 - last_corner.dot(last_corner);
    if t2 > 0.0 {
        contr_three = t2.powi(4) * GRAD3[gi2].xy().dot(last_corner)
    }

    (70.0 * (contr_one + contr_two + contr_three) + 1.0) / 2.0
}