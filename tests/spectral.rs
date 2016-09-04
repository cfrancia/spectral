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

#[test]
fn should_not_panic_if_vec_length_matches_expected() {
    let test_vec = vec![1,2,3];
    Spec::assert_that(&test_vec).has_length(3);
}

#[test]
#[should_panic]
fn should_panic_if_vec_length_does_not_match_expected() {
    let test_vec = vec![1,2,3];
    Spec::assert_that(&test_vec).has_length(1);
}

#[test]
fn should_be_able_to_map_to_inner_field_of_struct_when_matching() {
    let test_struct = TestStruct { value: 5 };
    Spec::assert_that(&test_struct).map(|val| &val.value).is_equal_to(&5);
}

struct TestStruct {
    pub value: u8,
}
