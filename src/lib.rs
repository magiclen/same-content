/*!
# Same Content

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
*/

pub extern crate generic_array;

use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};

use generic_array::typenum::{IsGreaterOrEqual, True, U1, U256};
use generic_array::{ArrayLength, GenericArray};

#[inline]
pub fn same_content_from_files(a: &mut File, b: &mut File) -> Result<bool, io::Error> {
    same_content_from_files2::<U256>(a, b)
}

#[inline]
pub fn same_content_from_files2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
    a: &mut File,
    b: &mut File,
) -> Result<bool, io::Error> {
    let metadata_a = a.metadata()?;
    let metadata_b = b.metadata()?;

    if metadata_a.len() != metadata_b.len() {
        return Ok(false);
    }

    a.seek(SeekFrom::Start(0))?;
    b.seek(SeekFrom::Start(0))?;

    same_content_from_readers2::<N>(a, b)
}

#[inline]
pub fn same_content_from_readers(a: &mut dyn Read, b: &mut dyn Read) -> Result<bool, io::Error> {
    same_content_from_readers2::<U256>(a, b)
}

pub fn same_content_from_readers2<N: ArrayLength<u8> + IsGreaterOrEqual<U1, Output = True>>(
    a: &mut dyn Read,
    b: &mut dyn Read,
) -> Result<bool, io::Error> {
    let mut buffer1: GenericArray<u8, N> = GenericArray::default();
    let mut buffer2: GenericArray<u8, N> = GenericArray::default();

    loop {
        let ca = a.read(&mut buffer1)?;

        if ca == 0 {
            let cb = read_try_exact(b, &mut buffer2[..1])?;

            return Ok(cb == 0);
        } else {
            let cb = read_try_exact(b, &mut buffer2[..ca])?;

            if ca != cb {
                return Ok(false);
            }

            if buffer1[..ca] != buffer2[..ca] {
                return Ok(false);
            }
        }
    }
}

fn read_try_exact(a: &mut dyn Read, mut buffer: &mut [u8]) -> Result<usize, io::Error> {
    let mut sum = 0;

    while !buffer.is_empty() {
        match a.read(buffer) {
            Ok(0) => break,
            Ok(n) => {
                buffer = &mut buffer[n..];

                sum += n;
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }

    Ok(sum)
}
