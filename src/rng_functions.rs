use rust_decimal::{prelude::FromPrimitive, prelude::ToPrimitive, Decimal};

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
    let rand_base = u128::random();
    let range = max - min;

    let proportion_unit = u128::MAX / range as u128;
    let proportional_rand = rand_base / proportion_unit;

    proportional_rand as i64 + min
}

pub fn random_decimal(min: Decimal, max: Decimal) -> Decimal {
    let rand_base = u128::random();
    let range: f64 = (max - min).to_f64().unwrap();

    let proportion_unit = (u128::MAX as f64) / range;
    let proportional_rand: Decimal = Decimal::from_f64(rand_base as f64 / proportion_unit).unwrap();

    proportional_rand + min
}