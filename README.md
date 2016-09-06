# spectral
Fluent test assertions for Rust.

Influenced by Google Truth and other fluent assertion frameworks.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spectral = { git = "https://github.com/cfrancia/spectral" }
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
