use rand::{thread_rng, distributions::{Alphanumeric, DistString}};

pub static FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";
pub fn generate_token(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}