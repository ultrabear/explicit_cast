# `explicit_cast`
A crate for explicitly widening, truncating, and sign casting integers.  
This crate is very simple, but the functionality it provides is universal, as such, it is distributed under `Apache-2.0 OR MIT`.

## Usage
```rust
use explict_cast::prelude::*;

let my_value: u8 = u16::MAX.truncate();
let next: i16 = my_value.widen().sign_cast();

assert_eq!(next, 0xffi16);
```
