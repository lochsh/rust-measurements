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

#[test]
fn milligrams() {
    let mass = Mass::from_kilograms(2.0);
    let milligrams = mass.as_milligrams();

    assert_almost_eq(milligrams, 2000000.0);
}

#[test]
fn micrograms() {
    let mass = Mass::from_kilograms(5.5);
    let micrograms = mass.as_micrograms();

    assert_almost_eq(micrograms, 5500000000.0);
}
