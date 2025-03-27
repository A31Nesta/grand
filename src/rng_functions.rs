use crate::Randomizable;

pub fn random_f64(min: f64, max: f64) -> f64 {
    let rand_base = u128::random();
    let range = max - min;

    let proportion_unit = (u128::MAX as f64) / range;
    let proportional_rand = rand_base as f64 / proportion_unit;

    proportional_rand + min
}