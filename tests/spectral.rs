extern crate spectral;

use spectral::assert_that;

#[test]
fn should_not_panic_on_equal_subjects() {
    assert_that(&1).is_equal_to(&1);
}

#[test]
#[should_panic]
fn should_panic_on_unequal_subjects() {
    assert_that(&1).is_equal_to(&2);
}

#[test]
fn should_not_panic_if_vec_length_matches_expected() {
    let test_vec = vec![1,2,3];
    assert_that(&test_vec).has_length(3);
}

#[test]
#[should_panic]
fn should_panic_if_vec_length_does_not_match_expected() {
    let test_vec = vec![1,2,3];
    assert_that(&test_vec).has_length(1);
}

#[test]
fn should_not_panic_if_vec_contains_value() {
    let test_vec = vec![1,2,3];
    assert_that(&test_vec).contains(&2);
}

#[test]
#[should_panic]
fn should_panic_if_vec_does_not_contain_value() {
    let test_vec = vec![1,2,3];
    assert_that(&test_vec).contains(&5);
}

#[test]
fn should_not_panic_if_vec_contains_mapped_value() {
    let test_vec = vec![TestStruct { value: 5  }, TestStruct { value: 6 }];
    assert_that(&test_vec).mapped_contains(|val| &val.value, &5);
}

#[test]
#[should_panic]
fn should_panic_if_vec_does_not_contain_mapped_value() {
    let test_vec = vec![TestStruct { value: 5  }, TestStruct { value: 6 }];
    assert_that(&test_vec).mapped_contains(|val| &val.value, &1);
}

#[test]
fn should_not_panic_if_option_is_expected_to_contain_value_and_does() {
    let option = Some("Hello");
    assert_that(&option).is_some();
}

#[test]
#[should_panic]
fn should_panic_if_option_is_expected_to_contain_value_and_does_not() {
    let option: Option<&str> = None;
    assert_that(&option).is_some();
}

#[test]
fn should_not_panic_if_option_contains_expected_value() {
    let option = Some("Hello");
    assert_that(&option).contains_value(&"Hello");
}

#[test]
#[should_panic]
fn should_panic_if_option_does_not_contain_expected_value() {
    let option = Some("Hello");
    assert_that(&option).contains_value(&"Hi");
}

#[test]
#[should_panic]
fn should_panic_if_option_is_none_but_expected_value() {
    let option: Option<&str> = None;
    assert_that(&option).contains_value(&"Hello");
}

#[test]
fn should_not_panic_if_option_is_empty() {
    let option: Option<&str> = None;
    assert_that(&option).is_none();
}

#[test]
#[should_panic]
fn should_panic_if_option_is_not_empty_but_was_expected_as_empty() {
    let option = Some("Hello");
    assert_that(&option).is_none();
}

#[test]
fn should_not_panic_if_result_is_expected_to_be_ok_and_is() {
    let result: Result<&str, &str> = Ok("Hello");
    assert_that(&result).is_ok();
}

#[test]
#[should_panic]
fn should_panic_if_result_is_expected_to_be_ok_and_is_not() {
    let result: Result<&str, &str> = Err("Oh no");
    assert_that(&result).is_ok();
}

#[test]
fn should_not_panic_if_result_is_expected_to_be_error_and_is() {
    let result: Result<&str, &str> = Err("Oh no");
    assert_that(&result).is_error();
}

#[test]
#[should_panic]
fn should_panic_if_result_is_expected_to_be_error_and_is_not() {
    let result: Result<&str, &str> = Ok("Hello");
    assert_that(&result).is_error();
}

#[test]
fn should_not_panic_if_value_matches() {
    let value = "Hello";
    assert_that(&value).matches(|val| val.eq(&"Hello"));
}

#[test]
#[should_panic]
fn should_panic_if_value_does_not_match() {
    let value = "Hello";
    assert_that(&value).matches(|val| val.eq(&"Hi"));
}

#[test]
fn should_be_able_to_map_to_inner_field_of_struct_when_matching() {
    let test_struct = TestStruct { value: 5 };
    assert_that(&test_struct).map(|val| &val.value).is_equal_to(&5);
}

#[derive(Debug, PartialEq)]
struct TestStruct {
    pub value: u8,
}
