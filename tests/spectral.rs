extern crate spectral;

use spectral::Spec;

#[test]
fn should_not_panic_on_equal_subjects() {
    Spec::assert_that(&1).is_equal_to(&1);
}

#[test]
#[should_panic]
fn should_panic_on_unequal_subjects() {
    Spec::assert_that(&1).is_equal_to(&2);
}
