use mass::*;
use super::assert_almost_eq;

#[test]
fn kilograms() {
    let kg = 100.0;

    let mass = Mass::from_kilograms(kg);
    let other = mass.as_kilograms();

    assert_almost_eq(other, kg);
}

#[test]
fn grams() {
    let mass = Mass::from_kilograms(1.0);
    let grams = mass.as_grams();

    assert_almost_eq(grams, 1000.0);
}
