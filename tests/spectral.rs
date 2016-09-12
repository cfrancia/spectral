extern crate spectral;

use spectral::prelude::*;

#[test]
#[should_panic(expected = "test condition: expected <2> but was <1>")]
fn should_contain_assertion_description_in_panic() {
    asserting(&"test condition").that(&1).is_equal_to(&2);
}

#[test]
#[should_panic(expected = "closure: expectation failed for value <\"Hello\">")]
fn should_contain_assertion_description_if_message_is_provided() {
    let value = "Hello";
    asserting(&"closure").that(&value).matches(|val| val.eq(&"Hi"));
}

#[test]
fn should_not_panic_on_equal_subjects() {
    assert_that(&1).is_equal_to(&1);
}

#[test]
#[should_panic(expected = "expected <2> but was <1>")]
fn should_panic_on_unequal_subjects() {
    assert_that(&1).is_equal_to(&2);
}

#[test]
fn should_not_panic_if_value_is_less_than_expected() {
    assert_that(&1).is_less_than(&2);
}

#[test]
#[should_panic(expected = "expected value less than <2> but was <3>")]
fn should_panic_if_value_is_greater_than_expected() {
    assert_that(&3).is_less_than(&2);
}

#[test]
fn should_not_panic_if_value_is_less_than_or_equal_to_than_expected() {
    assert_that(&2).is_less_than_or_equal_to(&2);
    assert_that(&2).is_less_than_or_equal_to(&3);
}

#[test]
#[should_panic(expected = "expected value less than or equal to <2> but was <3>")]
fn should_panic_if_value_is_greater_than_or_not_equal_to_expected() {
    assert_that(&3).is_less_than_or_equal_to(&2);
}

#[test]
fn should_not_panic_if_value_is_greater_than_expected() {
    assert_that(&3).is_greater_than(&2);
}

#[test]
#[should_panic(expected = "expected value greater than <3> but was <2>")]
fn should_panic_if_value_is_less_than_expected() {
    assert_that(&2).is_greater_than(&3);
}

#[test]
fn should_not_panic_if_value_is_greater_than_or_equal_to_expected() {
    assert_that(&3).is_greater_than_or_equal_to(&3);
    assert_that(&3).is_greater_than_or_equal_to(&2);
}

#[test]
#[should_panic(expected = "expected value greater than or equal to <3> but was <2>")]
fn should_panic_if_value_is_less_than_or_not_equal_to_expected() {
    assert_that(&2).is_greater_than_or_equal_to(&3);
}

#[test]
fn should_not_panic_if_vec_length_matches_expected() {
    let test_vec = vec![1,2,3];
    assert_that(&test_vec).has_length(3);
}

#[test]
#[should_panic(expected = "expected vec to have length <1> but was <3>")]
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
#[should_panic(expected = "expected vec to contain <5> but was <[1, 2, 3]>")]
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
#[should_panic(expected = "expected vec to contain <1> but was <[5, 6]>")]
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
#[should_panic(expected = "expected option[some] but was option[none]")]
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
#[should_panic(expected = "expected option to contain <\"Hi\"> but was <\"Hello\">")]
fn should_panic_if_option_does_not_contain_expected_value() {
    let option = Some("Hello");
    assert_that(&option).contains_value(&"Hi");
}

#[test]
#[should_panic(expected = "expected option<\"Hello\"> but was option[none]")]
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
#[should_panic(expected = "expected option[none] but was option<\"Hello\"")]
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
#[should_panic(expected = "expected result[ok] but was result[error]<\"Oh no\">")]
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
#[should_panic(expected = "expected result[error] but was result[ok]<\"Hello\">")]
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
#[should_panic(expected = "expectation failed for value <\"Hello\">")]
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
