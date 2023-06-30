use rust_maths::*;


#[test]
fn seeded_noise_test() {
    let mut rng = ChaChaRng::seed_from_u64(05810501850894);

    let set_one = vec![
        rng.gen_range(0..100),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
        rng.gen_range(0..100)
    ];

    let mut new_rand = ChaChaRng::seed_from_u64(05810501850894);

    for val in set_one {
        assert_eq!(val, new_rand.gen_range(0..100));
    }
}