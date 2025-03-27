pub mod rng;

use rng::Randomizable;

pub fn random_f64() -> f64 {
    // const MIN: u64 = i64::MIN as u64;
    // const MAX: u64 = i64::MAX as u64;

    let rand_base = i128::random();
    let range = u64::MAX;

    let proportion_unit = u128::MAX / range as u128;

    let proportional_rand_int = rand_base / proportion_unit as i128;
    let proportional_rand_remainder = rand_base % proportion_unit as i128;
    let proportional_rand_decimals = 1f64 / proportional_rand_remainder as f64;

    proportional_rand_int as f64 + proportional_rand_decimals
}