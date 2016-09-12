# spectral
Fluent test assertions for Rust.

Influenced by Google Truth and other fluent assertion frameworks.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spectral = "0.1.0"
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
expected <something> but was <something else>
```

To add additional clarification to the panic message, you can also deliberately state what you are asserting by calling the `asserting(...)` function rather than `assert_that(...)`:
```rust
asserting(&"test condition").that(&1).is_equal_to(&2);
```

Which will produce:
```
test condition: expected <2> but was <1>
```

### Mapping values

If you want to assert against a value contained within a struct, you can call `map(...)` with a closure, which will create a new `Spec` based upon the return value of the closure. You can then call any applicable assertions against the mapped value.
```rust
let test_struct = TestStruct { value: 5 };
assert_that(&test_struct).map(|val| &val.value).is_equal_to(&5);
```

## Assertions

As a general note, any type under test will usually need to implement at least `Debug`. Other assertions will have varying bounds attached to them.

### General
#### is_equal_to

Asserts that the actual value and the expected value are equal. The value type must implement `PartialEq`.

```rust
assert_that(&"hello").is_equal_to(&"hello");
```

#### matches

Accepts a function accepting the value type which returns a `bool`. Returning `false` will cause the assertion to fail.

NOTE: The resultant panic message will only state the actual value. It's recommended that you write your own assertion rather than relying upon this.

```rust
assert_that(&"hello").matches(|x| x.eq(&"hello"));
```

### Numbers

#### is_less_than

Asserts that the actual value is less than the expected value. The value type must implement `PartialOrd`.

```rust
assert_that(&1).is_less_than(&2);
```

#### is_less_than_or_equal_to

Asserts that the actual value is less than or equal to the expected value. The value type must implement `PartialOrd`.

```rust
assert_that(&2).is_less_than_or_equal_to(&2);
```

#### is_greater_than

Asserts that the actual value is greater than the expected value. The value type must implement `PartialOrd`.

```rust
assert_that(&2).is_greater_than(&1);
```

#### is_greater_than_or_equal_to

Asserts that the actual value is greater than or equal to the expected value. The value type must implement `PartialOrd`.

```rust
assert_that(&2).is_greater_than_or_equal_to(&1);
```

### Options

#### is_some

Asserts that the actual value is `Some`. The value type must be an `enum`.

```rust
assert_that(&Some(1)).is_some();
```

#### is_none

Asserts that the actual value is `None`. The value type must be an `enum`.

```rust
assert_that(&Option::None::<String>).is_none();
```

#### contains_value

Asserts that the actual value is a `Some` containing the expected value. The value type must be an `enum`.

```rust
assert_that(&Some(1)).contains_value(&1);
```

### Results

#### is_ok

Asserts that the actual value is `Ok`. The value type must be `Result`.

```rust
assert_that(&Result::Ok::<usize, usize>(1)).is_ok();
```

#### is_error

Asserts that the actual value is `Err`. The value type must be `Result`.

```rust
assert_that(&Result::Err::<usize, usize>(1)).is_error();
```

### Strings

#### starts_with

Asserts that the actual `&str` starts with the provided `&str`.

```rust
assert_that(&"Hello").starts_with(&"H");
```

#### ends_with

Asserts that the actual `&str` ends with the provided `&str`.

```rust
assert_that(&"Hello").ends_with(&"o");
```

#### contains

Asserts that the actual `&str` contains the provided `&str`.

```rust
assert_that(&"Hello").contains(&"e");
```

### Vectors

#### has_length

Asserts that the length of the actual vector is equal to the provided length. The value type must be of `Vec`.

```rust
assert_that(&vec![1, 2, 3, 4]).has_length(4);
```

#### contains

Asserts that the actual `Vec` contains the provided value. The generic type of the `Vec` must implement `PartialEq`.

```rust
assert_that(&vec![1, 2, 3, 4]).contains(&4);
```

#### mapped_contains

Maps the values of the actual `Vec` before asserting that the mapped `Vec` contains the provided value. The type of the mapped value must implement `PartialEq`.

NOTE: The panic message will refer to the mapped values rather than the values present in the original `Vec`.

```rust
#[derive(PartialEq, Debug)]
struct Simple {
    pub val: usize,
}

...

assert_that(&vec![Simple { val: 1 }, Simple { val: 2 } ]).mapped_contains(|x| &x.val, &2);
```

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

The `Spec` struct always implements `with_expected(...)`, `with_actual(...)` and `fail(...)`, which provides the necessary functionality to fail the test with the usual message format. If you need greater control of the failure message, you can call `fail_with_message(...)` which will directly print the provided message.

In either case, any description provided using `asserting(...)` will always be prepended to the panic message.

For example, to create an assertion that the length of a `Vec` is at least a certain value:
```rust
trait VecAtLeastLength {
    fn has_at_least_length(self, expected: usize);
}

impl<'s, T> VecAtLeastLength for Spec<'s, Vec<T>> {
    fn has_at_least_length(self, expected: usize) {
        let subject = self.subject;
        if expected > subject.len() {
            self.with_expected(format!("vec with length at least <{}>", expected))
                .with_actual(format!("<{}>", subject.len()))
                .fail();
        }
    }
}
```

## Notes

This is still very much a work in progress. There are still many assertions missing on basic types, and the ones that are there may still change or be updated as required.
