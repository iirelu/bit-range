# bit-range

A small library for getting ranges of bits from collections of bytes.

* [Crates.io](https://crates.io/crates/bit-range)
* [Documentation](https://iirelu.github.io/bit-range/bit_range/)

## Example

```rust
extern crate bit_range;

use bit_range::BitRange;

let vec = vec![0b0110_1101, 0b0011_0100];
let slice = [0b1101_1010, 0b0000_0010, 0b1111_1111];

println!("{:0b}", vec.get_bit_range(1..5));

assert_eq!(vec.get_bit_range(1..9), slice.get_bit_range(0..8));
```
