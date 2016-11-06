# spectral
Fluent test assertions for Rust.

Influenced by Google Truth and other fluent assertion frameworks.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spectral = "0.5.0"
```

Then add this to your crate:
```rust
extern crate spectral
```

To quickly start using assertions, simply use the `prelude` module in your test module:
```rust
use spectral::prelude::*;
```

## Overview

Spectral allows you to write your assertions in a fluent manner by seperating out what you are testing with, what you are testing against and how you are asserting.

### Simple asserts

For example, to test that a produced value is equal to an expected value, you would write:
```rust
assert_that(&1).is_equal_to(&1);
```

Or that a Vec contains a certain number of elements:
```rust
let test_vec = vec![1,2,3];
assert_that(&test_vec).has_length(3);
```

The methods avaliable for asserting depend upon the type under test and what traits are implemented.

As described below, it's recommended to use the macro form of `assert_that!` to provide correct file and line numbers for failing assertions.

### Failure messages

For failing assertions, the usual panic message follows the following format:
```
    expected: <2>
     but was: <1>
```

To add additional clarification to the panic message, you can also deliberately state what you are asserting by calling the `asserting(...)` function rather than `assert_that(...)`:
```rust
asserting(&"test condition").that(&1).is_equal_to(&2);
```

Which will produce:
```
    test condition:
    expected: <2>
     but was: <1>
```

Using the macro form of `assert_that!` will provide you with the file and line of the failing assertion as well:
```
    expected: vec to have length <2>
     but was: <1>

    at location: tests/parser.rs:112
```

### Named Subjects

To make it more obvious what your subject actually is, you can call `.named(...)` after `assert_that` (or `asserting(...).that(...)`), which will print out the provided `&str` as the subject name if the assertion fails.

```
assert_that(&thing.attributes).named(&"thing attributes").has_length(2);
```

On failure, this will display:
```
    for subject [thing attributes]
    expected: vec to have length <2>
     but was: <1>
```

### Mapping values

If you want to assert against a value contained within a struct, you can call `map(...)` with a closure, which will create a new `Spec` based upon the return value of the closure. You can then call any applicable assertions against the mapped value.
```rust
let test_struct = TestStruct { value: 5 };
assert_that(&test_struct).map(|val| &val.value).is_equal_to(&5);
```

## Macros

If you add `#[macro_use]` to the `extern crate` declaration, you can also use the macro form of `assert_that` and `asserting`.

```rust
assert_that!(test_vec).has_length(5)
```

This allows you to pass through a subject to test without needing to deliberately turn it into a reference. However, for consistency, you can also use a deliberate reference in the macro as well.

```rust
assert_that!(&test_vec).has_length(5)
```

Additionally, this will provide you with the file and line number of the failing assertion (rather than just the internal spectral panic location).

## Assertions (Basic)

Note: Descriptions and examples for each of the assertions are further down in this readme.

### General
#### is_equal_to
#### is_not_equal_to
#### matches

### Booleans
#### is_true
#### is_false

### Numbers
#### is_less_than
#### is_less_than_or_equal_to
#### is_greater_than
#### is_greater_than_or_equal_to

### Floats (optional)
#### is_close_to

### Options
#### is_some -> (returns a new Spec with the Option value)
#### is_none
#### contains_value

### Paths
#### exists
#### does_not_exist
#### is_file
#### is_directory
#### has_file_name

### Results
#### is_ok -> (returns a new Spec with the Ok value)
#### is_error -> (returns a new Spec with the Err value)
#### is_ok_containing
#### is_err_containing

### Strings
#### starts_with
#### ends_with
#### contains

### Vectors
#### has_length
#### is_empty

### HashMaps
#### has_length
#### is_empty
#### contains_key -> (returns a new Spec with the key value)
#### does_not_contain_key
#### contains_entry
#### does_not_contain_entry

### IntoIterator/Iterator
#### contains
#### does_not_contain
#### contains_all_of
#### mapped_contains
#### equals_iterator

### IntoIterator
#### matching_contains

## Optional Features

### Num Crate
The `num` crate is used for `Float` assertions. This feature will be enabled by default, but if you don't want the dependency on `num`, then simply disable it.

## Assertions (Detailed)

As a general note, any type under test will usually need to implement at least `Debug`. Other assertions will have varying bounds attached to them.

### General
#### is_equal_to

Asserts that the subject and the expected value are equal. The subject type must implement `PartialEq`.

##### Example
```rust
assert_that(&"hello").is_equal_to(&"hello");
```

##### Failure Message
```bash
	expected: <2>
	 but was: <1>
```

#### is_not_equal_to

Asserts that the subject and the expected value are not equal. The subject type must implement `PartialEq`.

##### Example
```rust
assert_that(&"hello").is_not_equal_to(&"hello");
```

##### Failure Message
```bash
	expected: <1> to not equal <1>
	 but was: equal
```

#### matches
Accepts a function accepting the subject type which returns a bool. Returning false will cause the assertion to fail.

NOTE: The resultant panic message will only state the actual value. It's recommended that you write your own assertions rather than relying upon this.

##### Example
```rust
assert_that(&"Hello").matches(|val| val.eq(&"Hello"));
```

##### Failure Message
```bash
	expectation failed for value <"Hello">
```

### Booleans
#### is_true

Asserts that the subject is true. The subject type must be `bool`.

##### Example
```rust
assert_that(&true).is_true(); 
```

##### Failure Message
```bash
	expected: bool to be <true>
	 but was: <false>
```

#### is_false

Asserts that the subject is false. The subject type must be `bool`.

##### Example
```rust
assert_that(&false).is_false();
```

##### Failure Message
```bash
	expected: bool to be <false>
	 but was: <true>
```

### Numbers
#### is_less_than

Asserts that the subject value is less than the expected value. The subject type must implement `PartialOrd`.

##### Example
```rust
assert_that(&1).is_less_than(&2);
```

##### Failure Message
```bash
	expected: value less than <2>
	 but was: <3>
```

#### is_less_than_or_equal_to

Asserts that the subject is less than or equal to the expected value. The subject type must implement `PartialOrd`.

##### Example
```rust
assert_that(&2).is_less_than_or_equal_to(&2);
```

##### Failure Message
```bash
	expected: value less than or equal to <2>
	 but was: <3>
```

#### is_greater_than

Asserts that the subject is greater than the expected value. The subject type must implement `PartialOrd`.

##### Example
```rust
assert_that(&2).is_greater_than(&1);
```

##### Failure Message
```bash
	expected: value greater than <3>
	 but was: <2>
```

#### is_greater_than_or_equal_to

Asserts that the subject is greater than or equal to the expected value. The subject type must implement `PartialOrd`.

##### Example
```rust
assert_that(&2).is_greater_than_or_equal_to(&1); 
```

##### Failure Message
```bash
	expected: value greater than or equal to <3>
	 but was: <2>
```

### Floats (optional)
#### is_close_to

Asserts that the subject is close to the expected value by the specified tolerance. The subject type must implement `Float` and `Debug`.

##### Example
```rust
assert_that(&2.0f64).is_close_to(2.0f64, 0.01f64);
```

##### Failure Message
```bash
	expected: float close to <1> (tolerance of <0.01>)
	 but was: <2>
```

### Options
#### is_some -> (returns a new Spec with the Option value)

Asserts that the subject is `Some`. The subject type must be an `Option`.

This will return a new `Spec` containing the unwrapped value if it is `Some`.

##### Example
```rust
assert_that(&Some(1)).is_some();
```

##### Chaining
```rust
assert_that(&option).is_some().is_equal_to(&"Hello");
```

##### Failure Message
```bash
	expected: option[some]
	 but was: option[none]
```

#### is_none

Asserts that the subject is `None`. The value type must be an `Option`.

##### Example
```rust
assert_that(&Option::None::<String>).is_none();
```

##### Failure Message
```bash
	expected: option[none]
	 but was: option<"Hello">
```

#### contains_value

Asserts that the subject is a `Some` containing the expected value. The subject type must be an `Option`.

##### Example
```rust
assert_that(&Some(1)).contains_value(&1);
```

##### Failure Message
```bash
	expected: option to contain <"Hi">
	 but was: <"Hello">
```


### Paths
#### exists

Asserts that the subject `Path` refers to an existing location.

##### Example
```rust
assert_that(&Path::new("/tmp/file")).exists();
```

##### Failure Message
```bash
	expected: Path of <"/tmp/file"> to exist
	 but was: a non-existent Path
```

#### does_not_exist

Asserts that the subject `Path` does not refer to an existing location.

##### Example
```rust
assert_that(&Path::new("/tmp/file")).does_not_exist();
```

##### Failure Message
```bash
	expected: Path of <"/tmp/file"> to not exist
     but was: a resolvable Path
```

#### is_file

Asserts that the subject `Path` refers to an existing file.

##### Example
```rust
assert_that(&Path::new("/tmp/file")).is_a_file();
```

##### Failure Message
```bash
	expected: Path of <"/tmp/file"> to be a file
	 but was: not a resolvable file
```

#### is_directory

Asserts that the subject `Path` refers to an existing directory.

##### Example
```rust
assert_that(&Path::new("/tmp/dir/")).is_a_directory();
```

##### Failure Message
```bash
	expected: Path of <"/tmp/dir/"> to be a directory
	 but was: not a resolvable directory
```

#### has_file_name

Asserts that the subject `Path` has the expected file name.

##### Example
```rust
assert_that(&Path::new("/tmp/file")).has_file_name(&"file");
```

##### Failure Message
```bash
	expected: Path with file name of <pom.xml>
	 but was: <Cargo.toml>
```


### Results
#### is_ok -> (returns a new Spec with the Ok value)

Asserts that the subject is `Ok`. The value type must be a `Result`.
    
This will return a new `Spec` containing the unwrapped value if it is `Ok`.

##### Example
```rust
assert_that(&Result::Ok::<usize, usize>(1)).is_ok();
```

##### Chaining
```rust
let result: Result<&str, &str> = Ok("Hello");
assert_that(&result).is_ok().is_equal_to(&"Hello");
```

##### Failure Message
```bash
	expected: result[ok]
	 but was: result[error]<"Oh no">
```

#### is_error -> (returns a new Spec with the Err value)

Asserts that the subject is `Err`. The value type must be a `Result`.

This will return a new `Spec` containing the unwrapped value if it is `Err`.

##### Example
```rust
assert_that(&Result::Err::<usize, usize>(1)).is_error();
```

##### Chaining
```rust
let result: Result<&str, &str> = Err("Hello");
assert_that(&result).is_error().is_equal_to(&"Hello");
```

##### Failure Message
```bash
	expected: result[error]
	 but was: result[ok]<"Hello">
```

#### is_ok_containing

Asserts that the subject is an `Ok` Result containing the expected value. The subject type must be a `Result`.

##### Example
```rust
assert_that(&Result::Ok::<usize, usize>(1)).is_ok_containing(&1);
```

##### Failure Message
```bash
	expected: Result[ok] containing <"Hi">
	 but was: Result[ok] containing <"Hello">
```

#### is_err_containing

Asserts that the subject is an `Err` Result containing the expected value. The subject type must be a `Result`.

##### Example
```rust
assert_that(&Result::Err::<usize, usize>(1)).is_err_containing(&1);
```

##### Failure Message
```bash
	expected: Result[err] containing <"Oh no">
	 but was: Result[err] containing <"Whoops">
```


### Strings
#### starts_with

Asserts that the subject `&str` starts with the provided `&str`.

##### Example
```rust
assert_that(&"Hello").starts_with(&"H");
```

##### Failure Message
```bash
	expected: string starting with <"A">
	 but was: <"Hello">
```

#### ends_with

Asserts that the subject `&str` ends with the provided `&str`.

##### Example
```rust
assert_that(&"Hello").ends_with(&"o");
```

##### Failure Message
```bash
	expected: string ending with <"A">
	 but was: <"Hello">
```

#### contains

Asserts that the subject `&str` contains the provided `&str`.

##### Example
```rust
assert_that(&"Hello").contains(&"e");
```

##### Failure Message
```bash
	expected: string containing <"A">
	 but was: <"Hello">
```


### Vectors
#### has_length

Asserts that the length of the subject vector is equal to the provided length. The subject type must be of `Vec`.

##### Example
```rust
assert_that(&vec![1, 2, 3, 4]).has_length(4);
```

##### Failure Message
```bash
	expected: vec to have length <1>
	 but was: <3>
```

#### is_empty

Asserts that the subject vector is empty. The subject type must be of `Vec`.

##### Example
```rust
let test_vec: Vec<u8> = vec![];
assert_that(&test_vec).is_empty();
```

##### Failure Message
```bash
	expected: an empty vec
	 but was: a vec with length <1>
```


### HashMaps
#### has_length

Asserts that the length of the subject hashmap is equal to the provided length. The subject type must be of `HashMap`.

##### Example
```rust
let mut test_map = HashMap::new();
test_map.insert(1, 1);
test_map.insert(2, 2);

assert_that(&test_map).has_length(2);
```

##### Failure Message
```bash
	expected: hashmap to have length <1>
	 but was: <2>
```

#### is_empty

Asserts that the subject hashmap is empty. The subject type must be of `HashMap`.

##### Example
```rust
let test_map: HashMap<u8, u8> = HashMap::new();
assert_that(&test_map).is_empty();
```

##### Failure Message
```bash
	expected: an empty hashmap
	 but was: a hashmap with length <1>
```

#### contains_key -> (returns a new Spec with the key value)

Asserts that the subject hashmap contains the expected key. The subject type must be of `HashMap`.

This will return a new `Spec` containing the associated value if the key is present.

##### Example
```rust
let mut test_map = HashMap::new();
test_map.insert("hello", "hi");

assert_that(&test_map).contains_key(&"hello");
```

##### Chaining
```rust
let mut test_map = HashMap::new();
test_map.insert("hello", "hi");

assert_that(&test_map).contains_key(&"hello").is_equal_to(&"hi");
```

##### Failure Message
```bash
	expected: hashmap to contain key <"hello">
	 but was: <["hey", "hi"]>
```

#### does_not_contain_key

Asserts that the subject hashmap does not contain the provided key. The subject type must be of `HashMap`.

##### Example
```rust
let mut test_map = HashMap::new();
test_map.insert("hello", "hi");

assert_that(&test_map).does_not_contain_key(&"hey");
```

##### Failure Message
```bash
	expected: hashmap to not contain key <"hello">
	 but was: present in hashmap
```

#### contains_entry

Asserts that the subject hashmap contains the expected key with the expected value. The subject type must be of `HashMap`.

##### Example
```rust
let mut test_map = HashMap::new();
test_map.insert("hello", "hi");

assert_that(&test_map).contains_entry(&"hello", &"hi");
```

##### Failure Message
```bash
    expected: hashmap containing key <"hi"> with value <"hey">
     but was: key <"hi"> with value <"hello"> instead
```

#### does_not_contain_entry

Asserts that the subject hashmap does not contain the provided key and value. The subject type must be of `HashMap`.

##### Example
```rust
let mut test_map = HashMap::new();
test_map.insert("hello", "hi");

assert_that(&test_map).does_not_contain_entry(&"hello", &"hey");
```

##### Failure Message
```bash
    expected: hashmap to not contain key <"hello"> with value <"hi">
     but was: present in hashmap
```


### IntoIterator/Iterator
#### contains

Asserts that the subject contains the provided value. The subject must implement `IntoIterator` or `Iterator`, and the contained type must implement `PartialEq` and `Debug`.

##### Example
```rust
let test_vec = vec![1,2,3];
assert_that(&test_vec).contains(&2);
```

##### Failure Message
```bash
	expected: iterator to contain <1>
	 but was: <[5, 6]>
```

#### does_not_contain

Asserts that the subject does not contain the provided value. The subject must implement `IntoIterator` or `Iterator`, and the contained type must implement `PartialEq` and `Debug`.

##### Example
```rust
let test_vec = vec![1,2,3];
assert_that(&test_vec).does_not_contain(&4);
```

##### Failure Message
```bash
	expected: iterator to not contain <1>
	 but was: <[1, 2]>
```

#### contains_all_of

Asserts that the subject contains all of the provided values. The subject must implement `IntoIterator` or `Iterator`, and the contained type must implement `PartialEq` and `Debug`.

##### Example
```rust
let test_vec = vec![1, 2, 3];
assert_that(&test_vec.iter()).contains_all_of(&vec![&2, &3]);
```

##### Failure Message
```bash
    expected: iterator to contain items <[1, 6]>
     but was: <[1, 2, 3]>
```

#### mapped_contains

Maps the values of the subject before asserting that the mapped subject contains the provided value. The subject must implement IntoIterator, and the type of the mapped value must implement `PartialEq`.

NOTE: The panic message will refer to the mapped values rather than the values present in the original subject.

##### Example
```rust
#[derive(PartialEq, Debug)]
struct Simple {
    pub val: usize,
}

...

assert_that(&vec![Simple { val: 1 }, Simple { val: 2 } ]).mapped_contains(|x| &x.val, &2);
```

##### Failure Message
```bash
	expected: iterator to contain <5>
	 but was: <[1, 2, 3]>
```

#### equals_iterator

Asserts that the subject is equal to provided iterator. The subject must implement `IntoIterator` or `Iterator`, the contained type must implement `PartialEq` and `Debug` and the expected value must implement Iterator and Clone.

##### Example
```rust
let expected_vec = vec![1,2,3];
let test_vec = vec![1,2,3];
assert_that(&test_vec).equals_iterator(&expected_vec.iter());
```

##### Failure Message
```bash
	expected: Iterator item of <4> (read <[1, 2]>)
	 but was: Iterator item of <3> (read <[1, 2]>)
```


### IntoIterator
#### matching_contains

Asserts that the subject contains a matching item by using the provided function. The subject must implement `IntoIterator`, and the contained type must implement `Debug`.

##### Example
```rust
let mut test_into_iter = LinkedList::new();
test_into_iter.push_back(TestEnum::Bad);
test_into_iter.push_back(TestEnum::Good);
test_into_iter.push_back(TestEnum::Bad);

assert_that(&test_into_iter).matching_contains(|val| {
    match val {
        &TestEnum::Good => true,
        _ => false
    }
});
```

##### Failure Message
```bash
expectation failed for iterator with values <[Bad, Bad, Bad]>
```

## How it works

The `Spec` struct implements a number of different bounded traits which provide assertions based upon the bound type.

As a single example, length assertions are provided by the `VecAssertions` trait:
```rust
pub trait VecAssertions {            
    fn has_length(self, expected: usize);
} 
```

Which is then implemented by Spec:
```rust
impl<'s, T> VecAssertions for Spec<'s, Vec<T>> {
    fn has_length(self, expected: usize) {
      ...
    }
} 
```

Naturally traits need to be included with a `use` before they apply, but to avoid an excessive number of `use` statements there is a `prelude` module which re-exports commonly used assertion traits.

## Creating your own

To create your own assertions, simply create a new trait containing your assertion methods and implement Spec against it.

To fail an assertion, create a new `AssertionFailure` struct using `from_spec(...)` within your assertion method and pass in `self`.

`AssertionFailure` also implements builder methods `with_expected(...)`, `with_actual(...)` and `fail(...)`, which provides the necessary functionality to fail the test with the usual message format. If you need greater control of the failure message, you can call `fail_with_message(...)` which will directly print the provided message.

In either case, any description provided using `asserting(...)` will always be prepended to the panic message.

For example, to create an assertion that the length of a `Vec` is at least a certain value:
```rust
trait VecAtLeastLength {
    fn has_at_least_length(&mut self, expected: usize);
}

impl<'s, T> VecAtLeastLength for Spec<'s, Vec<T>> {
    fn has_at_least_length(&mut self, expected: usize) {
        let subject = self.subject;
        if expected > subject.len() {
            AssertionFailure::from_spec(self)
                .with_expected(format!("vec with length at least <{}>", expected))
                .with_actual(format!("<{}>", subject.len()))
                .fail();
        }
    }
}
```
