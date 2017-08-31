use mass::*;
use super::assert_almost_eq;

#[test]
fn kilograms() {
    let mass = Mass::from_kilograms(100.0);
    let other = mass.as_kilograms();

    assert_almost_eq(other, mass);
}
