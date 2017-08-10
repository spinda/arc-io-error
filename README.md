# arc-io-error

This library provides the [`IoError`](https://docs.rs/arc-io-error/0.1.0) type, a
version of
[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
implemented on top of
[`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) instead
of [`std::boxed::Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html),
making it cloneable.

[![Build Status](https://travis-ci.org/spinda/arc-io-error.svg?branch=master)](https://travis-ci.org/spinda/arc-io-error)
[![Build status](https://ci.appveyor.com/api/projects/status/TODO?svg=true)](https://ci.appveyor.com/project/spinda/arc-io-error)
[![Crates.io](https://img.shields.io/crates/v/arc-io-error.svg?maxAge=2592000)](https://crates.io/crates/arc-io-error)

[Documentation](https://docs.rs/arc-io-error/0.1.0)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
arc-io-error = "0.1.0"
```

Next, add this to your crate:

```rust
extern crate arc_io_error;

use arc_io_error::IoError;
```

## Overview

The API of this type has been designed to match
[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html), with two
exceptions:

- [`IoError::new`](https://docs.rs/arc-io-error/0.1.0/struct.IoError.html#method.new)
  takes [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
  instead of
  [`std::boxed::Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html).
- [`IoError`](https://docs.rs/arc-io-error/0.1.0/struct.IoError.html) has no
  equivalent to
  [`std::io::Error::get_mut`](https://doc.rust-lang.org/std/io/struct.Error.html#method.get_mut),
  as the inner error instance is shared.

[`IoError`](https://docs.rs/arc-io-error/0.1.0/struct.IoError.html) implements
[`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html)
for [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
and vice-versa, so the two types can easily be converted between each other.
A type containing
[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) can
be made
[`std::clone::Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)-compatible
by instead storing
[`IoError`](https://docs.rs/arc-io-error/0.1.0/struct.IoError.html) internally
and converting from/to
[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) on API
boundaries.

Clones derived from the same original
[`IoError`](https://docs.rs/arc-io-error/0.1.0/struct.IoError.html)
instance will share a single heap-allocated error instance (if one is
present) using
[`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
instances produced by converting those clones back with the
[`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html)
implementation will also share the same single error instance.

## License

Licensed under either of

 * [Apache License, Version 2.0](/LICENSE-APACHE)
 * [MIT License](/LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
