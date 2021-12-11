
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn create_data(size: usize) -> Vec<f32> {
    let mut vec: Vec<f32> = (0..size).map(|v| v as f32).collect();
    vec.shuffle(&mut thread_rng());
    vec
}