use crate::rng_traits::Randomizable;

pub fn random_f64(min: f64, max: f64) -> f64 {
    let rand_base = u128::random();
    let range = max - min;

    let proportion_unit = (u128::MAX as f64) / range;
    let proportional_rand = rand_base as f64 / proportion_unit;

    proportional_rand + min
}

pub fn random_usize(min: usize, max: usize) -> usize {
    let rand_base = usize::random();
    let range = max - min;

    let proportion_unit = usize::MAX / range;
    let proportional_rand = rand_base / proportion_unit;

    proportional_rand + min
}
pub fn random_i64(min: i64, max: i64) -> i64 {
    let rand_base = u64::random();
    let range = max - min;

    let proportion_unit = u64::MAX / range as u64;
    let proportional_rand = rand_base / proportion_unit;

    proportional_rand as i64 + min
}