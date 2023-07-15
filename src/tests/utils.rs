use rand::{thread_rng, Rng};

pub fn generate_random_u8_vec(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut array = Vec::with_capacity(size);

    for _ in 0..size {
        array.push(rng.gen_range(0..=100));
    }

    array
}
