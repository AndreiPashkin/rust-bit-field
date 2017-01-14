//! Provides the abstraction of a bit field, which allows for bit-level update and retrieval
//! operations.

#![no_std]

#[cfg(test)]
mod tests;

use core::ops::Range;

/// A generic trait which provides methods for extracting and setting specific bits or ranges of
/// bits.
pub trait BitField {

    /// Returns the length, eg number of bits, in this bit field.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// assert_eq!(0u32.bit_length(), 32);
    /// assert_eq!(0u64.bit_length(), 64);
    /// ```
    fn bit_length(&self) -> u8;

    /// Obtains the bit at the index `bit`; note that index 0 is the least significant bit, while
    /// index `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let value: u32 = 0b110101;
    ///
    /// assert_eq!(value.get_bit(1), false);
    /// assert_eq!(value.get_bit(2), true);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the bit index is out of bounds of the bit field.
    fn get_bit(&self, bit: u8) -> bool;

    /// Obtains the range of bits specified by `range`; note that index 0 is the least significant
    /// bit, while index `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let value: u32 = 0b110101;
    ///
    /// assert_eq!(value.get_bits(0..3), 0b101);
    /// assert_eq!(value.get_bits(2..6), 0b1101);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the start or end indexes of the range are out of bounds of the
    /// bit field.
    fn get_bits(&self, range: Range<u8>) -> Self;

    /// Sets the bit at the index `bit` to the value `value` (where true means a value of '1' and
    /// false means a value of '0'); note that index 0 is the least significant bit, while index
    /// `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let mut value = 0u32;
    ///
    /// value.set_bit(1, true);
    /// assert_eq!(value, 2u32);
    ///
    /// value.set_bit(3, true);
    /// assert_eq!(value, 10u32);
    ///
    /// value.set_bit(1, false);
    /// assert_eq!(value, 8u32);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the bit index is out of the bounds of the bit field.
    fn set_bit(&mut self, bit: u8, value: bool) -> &mut Self;

    /// Sets the range of bits defined by the range `range` to the lower bits of `value`; to be
    /// specific, if the range is N bits long, the N lower bits of `value` will be used; if any of
    /// the other bits in `value` are set to 1, this function will panic.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let mut value = 0u32;
    ///
    /// value.set_bits(0..2, 0b11);
    /// assert_eq!(value, 0b11);
    ///
    /// value.set_bits(0..4, 0b1010);
    /// assert_eq!(value, 0b1010);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the range is out of bounds of the bit field, or if there are `1`s 
    /// not in the lower N bits of `value`.
    fn set_bits(&mut self, range: Range<u8>, value: Self) -> &mut Self;
}

/// An internal macro used for implementing BitField on the standard integral types.
macro_rules! bitfield_numeric_impl {
    ($($t:ty)*) => ($(
        impl BitField for $t {
            fn bit_length(&self) -> u8 {
                ::core::mem::size_of::<Self>() as u8 * 8
            }

            fn get_bit(&self, bit: u8) -> bool {
                assert!(bit < self.bit_length());

                (*self & (1 << bit)) != 0
            }

            fn get_bits(&self, range: Range<u8>) -> Self {
                assert!(range.start < self.bit_length());
                assert!(range.end <= self.bit_length());
                assert!(range.start < range.end);

                // shift away high bits
                let bits = *self << (self.bit_length() - range.end) >> (self.bit_length() - range.end);

                // shift away low bits
                bits >> range.start
            }

            fn set_bit(&mut self, bit: u8, value: bool) -> &mut Self {
                assert!(bit < self.bit_length());

                if value {
                    *self |= 1 << bit;
                } else {
                    *self &= !(1 << bit);
                }

                self
            }

            fn set_bits(&mut self, range: Range<u8>, value: Self) -> &mut Self {
                assert!(range.start < self.bit_length());
                assert!(range.end <= self.bit_length());
                assert!(range.start < range.end);
                assert!(value << (self.bit_length() - (range.end - range.start)) >>
                        (self.bit_length() - (range.end - range.start)) == value,
                        "The provided value when setting a range of bits had zeros outside of the size of the range!");

                let bitmask: Self = !(!0 << (self.bit_length() - range.end) >>
                                    (self.bit_length() - range.end) >>
                                    range.start << range.start);

                // set bits
                *self = (*self & bitmask) | (value << range.start);

                self
            }
        }
    )*)
}

bitfield_numeric_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
