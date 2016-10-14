# spectral
Fluent test assertions for Rust.

Influenced by Google Truth and other fluent assertion frameworks.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spectral = "0.4.0"
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

### Mapping values

If you want to assert against a value contained within a struct, you can call `map(...)` with a closure, which will create a new `Spec` based upon the return value of the closure. You can then call any applicable assertions against the mapped value.
```rust
let test_struct = TestStruct { value: 5 };
assert_that(&test_struct).map(|val| &val.value).is_equal_to(&5);
```

## Macros

If you add `#[macro_use]` to the `extern crate` declaration, you can also use the macro form of `assert_that`.

```rust
assert_that!(test_vec).has_length(5)
```

This allows you to pass through a subject to test without needing to deliberately turn it into a reference. However, for consistency, you can also use a deliberate reference in the macro as well.

```rust
assert_that!(&test_vec).has_length(5)
```

## Assertions

As a general note, any type under test will usually need to implement at least `Debug`. Other assertions will have varying bounds attached to them.

### General
#### is_equal_to
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
#### is_some
#### is_none
#### contains_value

### Paths
#### exists
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
#### contains_key_with_value

### IntoIterator/Iterator
#### contains
#### mapped_contains
#### equals_iterator

### IntoIterator
#### matching_contains

## Optional Features

### Num Crate
The `num` crate is used for `Float` assertions. This feature will be enabled by default, but if you don't want the dependency on `num`, then simply disable it.

## How it works

The `Spec` struct implements a number of different bounded traits which provide assertions based upon the bound type.

As a single example, length assertions are provided by the `VecSpec` trait:
```rust
pub trait VecSpec {            
    fn has_length(self, expected: usize);
} 
```

Which is then implemented by Spec:
```rust
impl<'s, T> VecSpec for Spec<'s, Vec<T>> {
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
