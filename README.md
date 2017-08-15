# arc-io-error

> This library provides the
> [`IoError`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html)
> type, a version of
> [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
> implemented on top of
> [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) instead
> of [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html),
> making it cloneable.

[![Crates.io](https://img.shields.io/crates/v/arc-io-error.svg)](https://crates.io/crates/arc-io-error)
[![Linux/OSX Build Status](https://img.shields.io/travis/spinda/arc-io-error/master.svg)](https://travis-ci.org/spinda/arc-io-error)
[![Windows Build Status](https://img.shields.io/appveyor/ci/spinda/arc-io-error/master.svg)](https://ci.appveyor.com/project/spinda/arc-io-error)

[Documentation](https://docs.rs/arc-io-error/0.1.1)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
arc-io-error = "0.1.1"
```

Next, add this to your crate:

```rust
extern crate arc_io_error;

use arc_io_error::IoError;
```

## Overview

The API of
[`IoError`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html)
has been designed to match
[`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html),
with two exceptions:

- [`IoError::new`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html#method.new)
  and
  [`IoError::into_inner`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html#method.into_inner)
  substitute
  [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) for
  [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html), and
- [`IoError`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html)
  has no equivalent to
  [`io::Error::get_mut`](https://doc.rust-lang.org/std/io/struct.Error.html#method.get_mut),
  as the inner error instance is shared.

[`IoError`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html)
implements
[`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
for [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
and vice-versa, so the two types can easily be converted between each other.
A type containing
[`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) can
be made
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)-compatible
by instead storing
[`IoError`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html)
internally and converting from/to
[`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) on API
boundaries.

Clones derived from the same original
[`IoError`](https://docs.rs/arc-io-error/0.1.1/arc_io_error/struct.IoError.html)
instance will share a single heap-allocated error instance (if one is
present) using
[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
[`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
instances produced by converting those clones back with the
[`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
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
