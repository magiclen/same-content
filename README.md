Same Content
====================

[![Build Status](https://travis-ci.org/magiclen/same-content.svg?branch=master)](https://travis-ci.org/magiclen/same-content)

Determine whether data from different sources are the same.

## Example

```rust
extern crate same_content;

use std::fs::File;

use same_content::*;

assert!(!same_content_from_files(&mut File::open("tests/data/P1140310.jpg").unwrap(), &mut File::open("tests/data/P1140558.jpg").unwrap()).unwrap());
```

## Change the Buffer Size

The default buffer size for the `same_content_from_files` function and the `same_content_from_readers` function is 256 bytes per stream. If you want to change that, you can use the `same_content_from_files2` function or the `same_content_from_readers2` function, and define a length explicitly.

For example, to change the buffer size to 4096 bytes,

```rust
#[macro_use] extern crate same_content;

use std::fs::File;

use same_content::*;
use same_content::generic_array::typenum::U4096;

assert!(!same_content_from_files2::<U4096>(&mut File::open("tests/data/P1140310.jpg").unwrap(), &mut File::open("tests/data/P1140558.jpg").unwrap()).unwrap());
```

## Crates.io

https://crates.io/crates/same-content

## Documentation

https://docs.rs/same-content

## License

[MIT](LICENSE)