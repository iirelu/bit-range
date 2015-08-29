//! A helper library for handling getting ranges of bits from containers of
//! bytes.

#![warn(missing_docs)]

use std::ops::Range;

/// A trait for getting subsections of bits from containers of bytes.
pub trait BitRange {
    /// Takes a range and converts the bits in that range into a u32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bit_range::BitRange;
    /// let bytes = [0b0010_0111, 0b0110_0110];
    /// assert_eq!(bytes.get_bit_range(5..12), 0b1110110);
    /// ```
    fn get_bit_range(&self, range: Range<u32>) -> u32;
    /// Takes an index and gets the bit at that index.
    ///
    /// ```rust
    /// use bit_range::BitRange;
    /// let bytes = vec![0b0010_0111, 0b110_0110];
    /// assert_eq!(bytes.get_bit(2), true);
    /// assert_eq!(bytes.get_bit(3), false);
    /// ```
    fn get_bit(&self, bit: u32) -> bool;
}

// This automatically works with Vec<u8> thanks to ~deref magic~
impl BitRange for [u8] {
    fn get_bit_range(&self, range: Range<u32>) -> u32 {
        let start_bit = range.start;
        let end_bit = range.end;
        let length = end_bit - start_bit;

        assert!(end_bit/8 <= self.len() as u32);
        assert!(length < 32);

        let mut result: u32 = 0;
        for (i, off) in (start_bit..end_bit).zip(1..) {
            result |= (self.get_bit(i) as u32) << (length-off);
        }
        result
    }

    fn get_bit(&self, bit: u32) -> bool {
        assert!(bit/8 < self.len() as u32);

        let byte = self[(bit/8) as usize] as u32;
        (byte >> 7-bit%8) & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::BitRange;

    #[test]
    fn test_get_bit_range() {
        let vec = vec![0b0100_1100, 0b0001_1111, 0b0000_0001];
        assert_eq!(vec.get_bit_range(1..14), 0b1001100000111);
    }

    #[test]
    fn test_get_bit() {
        let vec = vec![0b0100_1100, 0b0001_1111, 0b0000_0001];
        assert_eq!(vec.get_bit(1), true);
        assert_eq!(vec.get_bit(2), false);
        assert_eq!(vec.get_bit(22), false);
        assert_eq!(vec.get_bit(23), true);
    }
}
