# nnn

[<img alt="github" src="https://img.shields.io/badge/github-vic1707/nnn-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/vic1707/nnn)
[<img alt="crates.io" src="https://img.shields.io/crates/v/nnn.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/nnn)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/vic1707/nnn/hygiene.yml?branch=main&style=for-the-badge" height="20">](https://github.com/vic1707/nnn/actions?query=branch%3Amain)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-nnn-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/nnn)
[<img alt="lines" src="https://tokei.rs/b1/github/vic1707/nnn?type=Rust&style=for-the-badge?category=code&logo=https://simpleicons.org/icons/rust.svg" height="20">](https://github.com/vic1707/nnn)
[<img alt="maintenance" src="https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg?style=for-the-badge" height="20">](https://github.com/vic1707/nnn)

### nnn Crate Documentation

The `nnn` crate provides a procedural macro to help create [`newtype`](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)s with validation and sanitization based on a specified set of rules. Its design focuses on being as slim and non-intrusive as possible.

#### Philosophy

The primary goal of `nnn` is to provide tools, not guardrails, the only errors returned by `nnn` are parsing/syntax errors and footguns.
(e.g., `nnn` allows using a `finite` validator on a `String`—though this will not compile because `String` lacks `.is_finite()`. The same applies to the `each` validator and sanitizer, which are only available on inner with `.iter()`.)

By design, `nnn` doesn’t “hold hands” or attempt to protect users from all possible mistakes. Instead, it prioritizes flexibility and assumes user expertise.

---

#### Inspirations

The `nnn` crate draws heavy inspiration from the excellent [`nutype`](https://docs.rs/nutype/), borrowing much of its syntax and approach. While `nutype` offers robust features, its internal complexity motivated this project.

This crate was developed as a fun, 3-week challenge to explore whether `nutype`'s functionality could be reimagined in a simpler form. The result is `nnn`, which aims to provide a more streamlined experience without sacrificing power.

---

#### Complete example

```rs
use nnn::nnn;

#[nnn(
    derive(Debug, PartialEq, Eq, PartialOrd, Ord),
    nnn_derive(TryFrom),
    consts(
            ZERO = 0.0_f64,
        pub ONE = 1.0_f64,
    ),
    default = 5.0_f64,
    validators(finite, positive),
    sanitizers(custom = |v: f64| v.abs()),
    // Serialize & Deserialize are only available in test env.
    cfg(test, derive(Serialize, Deserialize)),
    attrs(
        repr(transparent),
    ),
)]
struct PositiveFiniteFloat(f64);
```

I encourage you to see what code is generated using [`cargo-expand`](https://github.com/dtolnay/cargo-expand).

#### Usage

Every argument in a `nnn` declaration must be provided in a single macro invocation, so

```rs
#[nnn(derive(Debug))]
#[nnn(default = 5.0_f64)]
struct Dummy(f64);
```

is invalid.

Note that `derive` and other attributes must be passed via `nnn` to make it clear that `nnn` manages them.

##### Arguments

Below is a complete list of supported arguments for the `nnn` macro:

- **`cfg(<condition>, <nnn_arguments>)`**: Adds conditional compilation to the provided arguments.

- **`consts(pub EIGHT = 8, pub(in crate) NINE = 9, ...)`**: Defines associated constants for the newtype.
  _Note:_ `nnn` will generate unit tests ensuring these values are correct.

- **`derive(<traits>, ...)`**: Specifies standard Rust derives for the newtype.

- **`nnn_derive(<traits>, ...)`**: Declares specific derives re-implemented by `nnn` that ensure validation and sanitization are applied where appropriate.

- **`default` or `default = ...`**: Defines a default value for the newtype. Can be either:

  - `#[nnn(default)]`: Uses the inner type's default value.
  - `#[nnn(default = ...)]`: Specifies a custom default value.

  _Note:_ `nnn` will generate a unit test ensuring the default value is correct.

- **`new_unchecked`**: Enables the generation of the unsafe method `const fn new_unchecked(v: <inner>) -> Self` that bypasses validation and sanitization.

- **`sanitizers(<list of sanitizers>)`**: Declares a list of sanitizers to apply to the input.
  _Note:_ Sanitization is executed **before** validation.

- **`validators(<list of validators>)`**: Declares a list of validators to ensure the input meets the desired conditions.

- **`attrs(<attributes to pass down to the newtype>)`**: Specifies additional attributes for the newtype, such as `#[repr(C)]` or `#[non_exhaustive]`.

##### Derive Handling

While most derives are passed through transparently, there are exceptions:

1. **`Eq` & `Ord`**

Automatically implemented except when the `finite` or `not_nan` validator is provided, in which case a manual implementation is generated.

2. **`Deserialize`**

Passed transparently, with `nnn` injecting `#[serde(try_from = "<inner>")]` to ensure validation and sanitization during deserialization.

**Note:** Some derives are disallowed as they bypass validation. For these cases, `nnn` provides a custom `nnn_derive` to replace standard derives while ensuring validation and sanitization are preserved.

3. **`FromStr`**

Provided via the `nnn_derive` argument, generates an implementation using the inner `FromStr` implementation and passing parsed value through sanitization and validation.
It generates the following error enum implementing `Debug`, `Clone`, `PartialEq`, `Eq` and `Display`.

```rs
enum /* new_type's name */ParseError {
    InnerParse(</* inner_type */ as ::core::str::FromStr>::Err),
    Validation(/* new_type's name */Error),
}
```

---

#### Sanitizers

_To see examples, each sanitizer is tested in the [test folder](./tests/sanitizers)._

The `sanitizers` argument accepts the following options:

- **Iterables**:

  - `each(...)`: Applies sanitizers to each element in a collection.
  - `sort`: Sorts the elements in the collection.
  - `dedup`: Removes duplicate elements from the collection using rust's `dedup` which only removes consecutive duplicates.

- **Strings**:

  - `trim`: Removes leading and trailing whitespace.
  - `lowercase`: Converts the string to lowercase.
  - `uppercase`: Converts the string to uppercase.

- **Common**:
  - `custom = ...`: Specifies a custom sanitization function.

---

#### Validators

_To see examples, each validator is tested in the [test folder](./tests/validators)._

Each validator generates a specific variant for the corresponding error type which has the same visibility as the `new_type`.
This error enum implements the traits: `Debug`, `Error`, `Display`, `Clone`, `PartialEq`, `Eq`.

```rs
enum /* new_type's name */Error {
    Positive,
    Finite,
    /// idx of the problematic element, inner error
    Each(usize, Box<Self>),
    /* ... */
}
```

The `validators` argument accepts the following options:

- **Iterables**:

  - `not_empty`: Ensures an iterable is not empty.
  - `each(...)`: Applies validators to each element in an iterable.
  - `min_length = ...`: Ensures the iterable has a minimum length.
  - `min_length_or_eq = ...`: Ensures the iterable has a minimum length or is equal to the specified length.
  - `length = ...`: Ensures the iterable has an exact length.
  - `max_length = ...`: Ensures the iterable has a maximum length.
  - `max_length_or_eq = ...`: Ensures the iterable has a maximum length or is equal to the specified length.

- **Numerics**:

  - `min = ...`: Ensures the value is greater than this value.
  - `min_or_eq = ...`: Ensures the value is greater than or equal to this value.
  - `max = ...`: Ensures the value is less than this value.
  - `max_or_eq = ...`: Ensures the value is less than or equal to this value.
  - `positive`: Ensures the value is positive.
  - `negative`: Ensures the value is negative.

- **Float specifics**:

  - `finite`: Ensures the value is finite.
  - `not_infinite`: Ensures the value is not infinite.
  - `not_nan`: Ensures the value is not NaN.

- **String specifics**:

  - `regex = ...`: Ensures the string matches a regular expression. You can pass a raw string or a variable.
    _Note:_ A test is generated to ensure the pattern is valid.
    _Note2:_ raw string regex can be checked at compile time with an optional feature (see: [Optional Features](#Optional-Features)).

- **Common**:

  - `exactly = ...`: Ensures the value is equal to the specified value.
  - `custom(with = ..., error = ...)`: Validates using a custom function, specifying an error path.
  - `predicate(with = ..., error_name = ...)`: Uses a predicate function with an optional custom error variant name (defaults to `Predicate`).

##### _Note:_ The `with =` argument to the `custom` and `predicate` validator/sanitizer can be of 3 forms:

- **inlined closure:** `with = |str: &String| f64::from_str(str)`
- **function path:** `with = f64::from_str`
- **inlined block:**
  These must use the variable `mut value: <inner>`.
  - _For validators:_ `with = { f64::from_str(&value) }`
  - _For sanitizers:_ `with = { value = value.to_uppercase(); }`

---

#### Optional Features

- **Compile-Time Regex**: When `regex_validation` is enabled, raw literal regex patterns are validated at compile time, so you don't have to run the generated test every time.

---

#### Why that name

For those who wonder, the name `nnn` reflects a 3-week, carefree adventure with no expectations — it's simply 'n' for 'newtype', tapped a random number of times.

![](https://i.makeagif.com/media/9-14-2024/8wcpfp.gif)

Ladies and Gentelmens, welcome to n-n-n-newtypes.

#### License

This project is licensed under the **[WTFPL](./LICENSE)**.
