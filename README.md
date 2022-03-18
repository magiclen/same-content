Same Content
====================

[![CI](https://github.com/magiclen/same-content/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/same-content/actions/workflows/ci.yml)

Determine whether data from different sources are the same.

## Example

```rust
use std::fs::File;

use same_content::*;

assert!(!same_content_from_files(&mut File::open("tests/data/P1140310.jpg").unwrap(), &mut File::open("tests/data/P1140558.jpg").unwrap()).unwrap());
```

## Change the Buffer Size

The default buffer size for the `same_content_from_files` function and the `same_content_from_readers` function is 256 bytes per stream. If you want to change that, you can use the `same_content_from_files2` function or the `same_content_from_readers2` function, and define a length explicitly.

For example, to change the buffer size to 4096 bytes,

```rust
use std::fs::File;

use same_content::*;
use same_content::generic_array::typenum::U4096;

assert!(!same_content_from_files2::<U4096>(&mut File::open("tests/data/P1140310.jpg").unwrap(), &mut File::open("tests/data/P1140558.jpg").unwrap()).unwrap());
```

## Asynchronous APIs

You may want to use async APIs with your async runtime. This crate supports `tokio`, currently.

```toml
[dependencies.same-content]
version = "*"
features = ["tokio"]
```

After enabling the async feature, the async functions are available.

## Crates.io

https://crates.io/crates/same-content

## Documentation

https://docs.rs/same-content

## License

[MIT](LICENSE)