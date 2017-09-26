#![no_std]
//! # bobbin-bits
//!
//!bobbin-bits defines types representing binary numbers of width 1 to 32 and ranged values from 1 to 32. These are useful for representing small bit fields and for indexing small collections.
//!
//!## Motivation
//!
//!Rust doesn't currently have direct support for unsigned integer types for widths other than u8, u16, u32, u64 and u128 or for ranged integers. Applications instead must store values in a larger primitive type and then check
//!that the values stay in the correct range, typically at function boundaries. This is error prone and can
//!impact performance.
//!
//!One solution is to define structs or enums to represent domain-specific values that are known to be in a specific range. This can eliminate run-time range checks at the cost of a large amount of boilerplate for managing conversions to and from these values. 
//!
//!For some APIs the code for managing these types ends up much larger than the API itself.
//!It can also prove to be a significant documentation challenge and barrier to learning the API. Having a unique type for almost every function parameter in an API is undesirable.
//!
//!This crate takes a different approach, defining a set of general-purpose types useful 
//!for representing bit fields <= 32 bits and integer ranges from 1 through 32. Conversion
//!traits are defined to and from Rust unsigned integer types and i32, performing
//!range checking where needed.
//!
//!## Panics
//!
//!These types will panic if a conversion fails because the value is out of range for the
//!destination type.
//!
//!## Representation
//!
//!Types U1 through U6 are enums with repr(u8), allowing exhaustive matching without a default
//!case. Their members are named with the prefix "B" followed by the
//!n-bit binary representation of that number, like this:
//!
//!```
//!#[repr(u8)]
//!pub enum U3 {
//!    B000 = 0b000,
//!    B001 = 0b001,
//!    B010 = 0b010,
//!    B011 = 0b011,
//!    B100 = 0b100,
//!    B101 = 0b101,
//!    B110 = 0b110,
//!    B111 = 0b111,
//!}
//!```
//!
//!
//!Similarly, R1 through R32 are enums with repr(usize). Their members are named with
//!the prefix "X" followed by the hexadecimal represention of the number, single digits
//!for 0-15 and two digits for 16-32:
//!
//!```
//!#[repr(usize)]
//!pub enum R12 {
//!    X0 = 0x0,
//!    X1 = 0x1,
//!    X2 = 0x2,
//!    X3 = 0x3,
//!    X4 = 0x4,
//!    X5 = 0x5,
//!    X6 = 0x6,
//!    X7 = 0x7,
//!    X8 = 0x8,
//!    X9 = 0x9,
//!    XA = 0xa,
//!    XB = 0xb,
//!}
//!```
//!
//!Types U7 and U8, U9 to U16 and U16 through U32 are wrappers around u8, u16 and u32 respectively:
//!
//!```
//!pub struct U20(u16);
//!```
//!
//!Unfortunately there is no literal representation of these values, so they must be
//!constructed using `From<T>` conversions or the `unchecked_from_xxx` functions
//!
//!## Traits
//!
//!The following traits are currently supported for all types:
//!
//!- `Debug for T`
//!- `Display for T`
//!- `LowerHex for T`
//!- `From<u8> for T`
//!- `From<T> for u8`
//!- `From<u16> for T`
//!- `From<T> for u16`
//!- `From<u32> for T`
//!- `From<T> for u32`
//!- `From<usize> for T`
//!- `From<T> for usize`
//!- `From<i32> for T`
//!- `From<T> for i32`
//!- `PartialEq<i32> for T`
//!
//!The following additional traits are also supported for U1:
//!
//!- `From<bool> for U1`
//!- `Not for U1`
//!
//!## Examples
//!
//!Here's an example using the U4 bit field type:
//!
//!```
//!use bobbin_bits::*;
//!
//!// Implemented using a single exhaustive match statement
//!fn to_hex_char<V: Into<U4>>(v: V) -> char {
//!    let v = v.into();
//!    match v {
//!        U4::B0000 => '0',
//!        U4::B0001 => '1',
//!        U4::B0010 => '2',
//!        U4::B0011 => '3',
//!        U4::B0100 => '4',
//!        U4::B0101 => '5',
//!        U4::B0110 => '6',
//!        U4::B0111 => '7',
//!        U4::B1000 => '8',
//!        U4::B1001 => '9',
//!        U4::B1010 => 'a',
//!        U4::B1011 => 'b',
//!        U4::B1100 => 'c',
//!        U4::B1101 => 'd',
//!        U4::B1110 => 'e',
//!        U4::B1111 => 'f',
//!    }
//!}
//!
//!// Call with a U4 bit field, no conversion or range checking is required.
//!let c = to_hex_char(U4::B1000);
//!assert_eq!(c, '8');
//!
//!// Call with a i32, v.into() performs range check.
//!let c = to_hex_char(8);
//!assert_eq!(c, '8');
//!
//!// Call with a u8, v.into() performs range check.
//!let c = to_hex_char(8_u8);
//!assert_eq!(c, '8');
//!
//!// Perform range check from u32 outside of function
//!let v: U4 = 8u32.into();
//!let c = to_hex_char(v);
//!assert_eq!(c, '8');
//!
//!// A function that will extract bits [4:7] from a u32 value
//!// without range checking
//!fn extract_u4(v: u32) -> U4 {
//!    unsafe {
//!        U4::from_u32_unchecked(v >> 4 & 0b1111)
//!    }
//!}
//!
//!// No range checking needs to take place if a U4 is used
//!// through the computation
//!let c = to_hex_char(extract_u4(0b0000_0000_1000_0000));
//!assert_eq!(c, '8');
//!
//!```
//!
//!Using the U12 and U13 types:
//!
//!```
//!use bobbin_bits::*;
//!
//!fn double_sample<V: Into<U12>>(v: V) -> U13 {
//!    let v = v.into();
//!    // Extracts into u16, multiplies, then wraps into U13
//!    // Performs range checking when creating the U13 value
//!    U13::from(v.value() * 2)
//!}
//!
//!// Range checking takes place within double_sample()
//!let v = double_sample(1000);
//!
//!// Unfortunately, no literal form for U13, so range checking
//!// happens when constructing U13 value from u16
//!assert_eq!(v, U13::from(2000));
//!
//!// When converting from types that cannot overflow the range (such as u8),
//!// no range checking is needed.
//!assert_eq!(double_sample(100), U13::from(200u8));
//!
//!// You can always access the underlying representation of the value
//!assert_eq!(v.value(), 2000u16);
//!
//!```
//!
//!Using the R4 range type, which supports values 0 to 3:
//!
//!```
//!use bobbin_bits::*;
//!
//!// Using R4 in an exhaustive match
//!fn get_port_name<I: Into<R4>>(index: I) -> &'static str {
//!    let index = index.into();
//!    match index {
//!        R4::X0 => "PORTA",
//!        R4::X1 => "PORTB",
//!        R4::X2 => "PORTC",
//!        R4::X3 => "PORTD",
//!    }
//!}
//!
//!pub const PORT_ADDR: [u32;4] = [0x1000_0000, 0x1000_2000, 0x1000_3000, 0x1000_4000];
//!
//!// Using a lookup table
//!fn get_port_address<I: Into<R4>>(index: I) -> u32 {
//!    // Is the optimizing compiler smart enough to eliminate the
//!    // bounds check here?
//!    PORT_ADDR[index.into() as usize]
//!}
//!
//!// From<i32> is implemented, range check happens in get_port_name()
//!let n = get_port_name(2);
//!assert_eq!(n, "PORTC");
//!
//!// Using R4::X2 does not need a range check
//!let n = get_port_name(R4::X2);
//!assert_eq!(n, "PORTC");
//!```

#[cfg(test)]
#[macro_use] extern crate std;

use core::fmt;
use core::mem::transmute;
use core::ops::Not;

#[macro_use] mod enums;
#[macro_use] mod structs;
#[macro_use] mod ranges;

// Generate enums from U1 to U6

impl_enum!(U1, 1, 0b1, 
    B0 = 0b0, 
    B1 = 0b1,
);

impl_enum!(U2, 2, 0b11, 
    B00 = 0b00,
    B01 = 0b01,
    B10 = 0b10,
    B11 = 0b11,
);

impl_enum!(U3, 3, 0b111, 
    B000 = 0b000,
    B001 = 0b001,
    B010 = 0b010,
    B011 = 0b011,
    B100 = 0b100,
    B101 = 0b101,
    B110 = 0b110,
    B111 = 0b111,
);

impl_enum!(U4, 4, 0b1111,
    B0000 = 0b0000,
    B0001 = 0b0001,
    B0010 = 0b0010,
    B0011 = 0b0011,
    B0100 = 0b0100,
    B0101 = 0b0101,
    B0110 = 0b0110,
    B0111 = 0b0111,
    B1000 = 0b1000,
    B1001 = 0b1001,
    B1010 = 0b1010,
    B1011 = 0b1011,
    B1100 = 0b1100,
    B1101 = 0b1101,
    B1110 = 0b1110,
    B1111 = 0b1111,
);

impl_enum!(U5, 5, 0b11111,
    B00000 = 0b00000,
    B00001 = 0b00001,
    B00010 = 0b00010,
    B00011 = 0b00011,
    B00100 = 0b00100,
    B00101 = 0b00101,
    B00110 = 0b00110,
    B00111 = 0b00111,
    B01000 = 0b01000,
    B01001 = 0b01001,
    B01010 = 0b01010,
    B01011 = 0b01011,
    B01100 = 0b01100,
    B01101 = 0b01101,
    B01110 = 0b01110,
    B01111 = 0b01111,
    B10000 = 0b10000,
    B10001 = 0b10001,
    B10010 = 0b10010,
    B10011 = 0b10011,
    B10100 = 0b10100,
    B10101 = 0b10101,
    B10110 = 0b10110,
    B10111 = 0b10111,
    B11000 = 0b11000,
    B11001 = 0b11001,
    B11010 = 0b11010,
    B11011 = 0b11011,
    B11100 = 0b11100,
    B11101 = 0b11101,
    B11110 = 0b11110,
    B11111 = 0b11111,
);

impl_enum!(U6, 6, 0b111111,
    B000000 = 0b000000,
    B000001 = 0b000001,
    B000010 = 0b000010,
    B000011 = 0b000011,
    B000100 = 0b000100,
    B000101 = 0b000101,
    B000110 = 0b000110,
    B000111 = 0b000111,
    B001000 = 0b001000,
    B001001 = 0b001001,
    B001010 = 0b001010,
    B001011 = 0b001011,
    B001100 = 0b001100,
    B001101 = 0b001101,
    B001110 = 0b001110,
    B001111 = 0b001111,
    B010000 = 0b010000,
    B010001 = 0b010001,
    B010010 = 0b010010,
    B010011 = 0b010011,
    B010100 = 0b010100,
    B010101 = 0b010101,
    B010110 = 0b010110,
    B010111 = 0b010111,
    B011000 = 0b011000,
    B011001 = 0b011001,
    B011010 = 0b011010,
    B011011 = 0b011011,
    B011100 = 0b011100,
    B011101 = 0b011101,
    B011110 = 0b011110,
    B011111 = 0b011111,
    B100000 = 0b100000,
    B100001 = 0b100001,
    B100010 = 0b100010,
    B100011 = 0b100011,
    B100100 = 0b100100,
    B100101 = 0b100101,
    B100110 = 0b100110,
    B100111 = 0b100111,
    B101000 = 0b101000,
    B101001 = 0b101001,
    B101010 = 0b101010,
    B101011 = 0b101011,
    B101100 = 0b101100,
    B101101 = 0b101101,
    B101110 = 0b101110,
    B101111 = 0b101111,
    B110000 = 0b110000,
    B110001 = 0b110001,
    B110010 = 0b110010,
    B110011 = 0b110011,
    B110100 = 0b110100,
    B110101 = 0b110101,
    B110110 = 0b110110,
    B110111 = 0b110111,
    B111000 = 0b111000,
    B111001 = 0b111001,
    B111010 = 0b111010,
    B111011 = 0b111011,
    B111100 = 0b111100,
    B111101 = 0b111101,
    B111110 = 0b111110,
    B111111 = 0b111111,
);

// Generate structs from U7 to U32

impl_u8!(U7, 0b111_1111);
impl_u8!(U8, 0b1111_11111);
impl_u16!(U9, 0b1_1111_1111);
impl_u16!(U10, 0b11_1111_1111);
impl_u16!(U11, 0b111_1111_1111);
impl_u16!(U12, 0b1111_1111_1111);
impl_u16!(U13, 0b1_1111_1111_1111);
impl_u16!(U14, 0b11_1111_1111_1111);
impl_u16!(U15, 0b111_1111_1111_1111);
impl_u16!(U16, 0b1111_1111_1111_1111);
impl_u32!(U17, 0b1_1111_1111_1111_1111);
impl_u32!(U18, 0b11_1111_1111_1111_1111);
impl_u32!(U19, 0b111_1111_1111_1111_1111);
impl_u32!(U20, 0b1111_1111_1111_1111_1111);
impl_u32!(U21, 0b1_1111_1111_1111_1111_1111);
impl_u32!(U22, 0b11_1111_1111_1111_1111_1111);
impl_u32!(U23, 0b111_1111_1111_1111_1111_1111);
impl_u32!(U24, 0b1111_1111_1111_1111_1111_1111);
impl_u32!(U25, 0b1_1111_1111_1111_1111_1111_1111);
impl_u32!(U26, 0b11_1111_1111_1111_1111_1111_1111);
impl_u32!(U27, 0b111_1111_1111_1111_1111_1111_1111);
impl_u32!(U28, 0b1111_1111_1111_1111_1111_1111_1111);
impl_u32!(U29, 0b1_1111_1111_1111_1111_1111_1111_1111);
impl_u32!(U30, 0b11_1111_1111_1111_1111_1111_1111_1111);
impl_u32!(U31, 0b111_1111_1111_1111_1111_1111_1111_1111);
impl_u32!(U32, 0b1111_1111_1111_1111_1111_1111_1111_1111);

// Generate ranges from R1 to R32

impl_range!(R1, 1,
    X0 = 0x0,
);

impl_range!(R2, 2,
    X0 = 0x0,
    X1 = 0x1,
);

impl_range_from!(R2, R1);

impl_range!(R3, 3,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
);

impl_range_from!(R3, R2, R1);

impl_range!(R4, 4,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
);

impl_range_from!(R4, R3, R2, R1);

impl_range!(R5, 5,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
);

impl_range_from!(R5, R4, R3, R2, R1);

impl_range!(R6, 6,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
);

impl_range_from!(R6, R5, R4, R3, R2, R1);

impl_range!(R7, 7,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
);

impl_range_from!(R7, R5, R4, R3, R2, R1);

impl_range!(R8, 8,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
);

impl_range_from!(R8, R7, R5, R4, R3, R2, R1);

impl_range!(R9, 9,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
);

impl_range_from!(R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R10, 10,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
);

impl_range_from!(R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R11, 11,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    Xa = 0xa,
);

impl_range_from!(R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R12, 12,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    Xa = 0xa,
    Xb = 0xb,
);

impl_range_from!(R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R13, 13,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    Xa = 0xa,
    Xb = 0xb,
    Xc = 0xc,
);

impl_range_from!(R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R14, 14,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    Xa = 0xa,
    Xb = 0xb,
    Xc = 0xc,
    Xd = 0xd,
);

impl_range_from!(R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R15, 15,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    Xa = 0xa,
    Xb = 0xb,
    Xc = 0xc,
    Xd = 0xd,
    Xe = 0xe,
);

impl_range_from!(R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R16, 16,
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    Xa = 0xa,
    Xb = 0xb,
    Xc = 0xc,
    Xd = 0xd,
    Xe = 0xe,
    Xf = 0xf,
);

impl_range_from!(R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R17, 17,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
);

impl_range_from!(R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R18, 18,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
);

impl_range_from!(R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R19, 19,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
);

impl_range_from!(R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R20, 20,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
);

impl_range_from!(R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R21, 21,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
);

impl_range_from!(R21, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R22, 22,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
);

impl_range_from!(R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R23, 23,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
);

impl_range_from!(R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R24, 24,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
);

impl_range_from!(R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R25, 25,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
);

impl_range_from!(R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R26, 26,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
    X19 = 0x19,
);

impl_range_from!(R26, R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R27, 27,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
    X19 = 0x19,
    X1a = 0x1a,
);

impl_range_from!(R27, R26, R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R28, 28,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
    X19 = 0x19,
    X1a = 0x1a,
    X1b = 0x1b,
);

impl_range_from!(R28, R27, R26, R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R29, 29,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
    X19 = 0x19,
    X1a = 0x1a,
    X1b = 0x1b,
    X1c = 0x1c,
);

impl_range_from!(R29, R28, R27, R26, R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R30, 30,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
    X19 = 0x19,
    X1a = 0x1a,
    X1b = 0x1b,
    X1c = 0x1c,
    X1d = 0x1d,
);

impl_range_from!(R30, R29, R28, R27, R26, R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R31, 31,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
    X19 = 0x19,
    X1a = 0x1a,
    X1b = 0x1b,
    X1c = 0x1c,
    X1d = 0x1d,
    X1e = 0x1e,
);

impl_range_from!(R31, R30, R29, R28, R27, R26, R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);

impl_range!(R32, 32,
    X00 = 0x00,
    X01 = 0x01,
    X02 = 0x02,
    X03 = 0x03,
    X04 = 0x04,
    X05 = 0x05,
    X06 = 0x06,
    X07 = 0x07,
    X08 = 0x08,
    X09 = 0x09,
    X0a = 0x0a,
    X0b = 0x0b,
    X0c = 0x0c,
    X0d = 0x0d,
    X0e = 0x0e,
    X0f = 0x0f,
    X10 = 0x10,
    X11 = 0x11,
    X12 = 0x12,
    X13 = 0x13,
    X14 = 0x14,
    X15 = 0x15,
    X16 = 0x16,
    X17 = 0x17,
    X18 = 0x18,
    X19 = 0x19,
    X1a = 0x1a,
    X1b = 0x1b,
    X1c = 0x1c,
    X1d = 0x1d,
    X1e = 0x1e,
    X1f = 0x1f,
);

impl_range_from!(R32, R31, R30, R29, R28, R27, R26, R25, R24, R23, R22, R20, R19, R18, R17, R16, R15, R14, R13, R12, R11, R10, R9, R8, R7, R5, R4, R3, R2, R1);


// Special Traits for U1


impl From<bool> for U1 {
    #[inline]
    fn from(other: bool) -> U1 {
        U1::from(other as u8)
    }
}

impl U1 {
    #[inline]
    pub fn is_set(&self) -> bool {
        *self as u8 != 0
    }
}

impl Not for U1 {
    type Output = U1;
    #[inline]
    fn not(self) -> U1 {
        U1::from(self as u8)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(U1::from(0), U1::B0);
        assert_eq!(U1::from(1), U1::B1);
        let v: u8 = U1::B0.into();
        assert_eq!(v, 0u8);
        assert_eq!(<U1 as Into<u8>>::into(U1::B1), 1u8);

        // let value: u32 = U1::B1.into();
    }
    #[test]
    fn test_bool_cast() {
        assert_eq!(true as u32, 1);
        assert_eq!(false as u32, 0);
        assert_eq!(0 != 0, false);
        assert_eq!(1 != 0, true);
    }

    #[test]
    fn test_fn_into() {
        fn abc<V: Into<U2>>(v: V) -> U2 {
            v.into()
        }
        assert_eq!(abc(0b00u8), U2::B00);
        assert_eq!(abc(0b11u8), U2::B11);

        fn def<V: Into<u8>>(v: V) -> u8 {
            v.into()
        }

        assert_eq!(def(U2::B01), 0b01u8);
        assert_eq!(def(0b01u8), 0b01u8);

        assert_eq!(U1::B0, U1::from(false));
        assert_eq!(U1::B1, U1::from(true));
    }

    #[test]
    fn test_b1() {
        let f = U1::B0;
        let t = U1::B1;

        assert!(t.is_set());
        assert!(!f.is_set());

        assert!(t.value() != 0);
        assert!(f.value() == 0);

        assert!(t == 1);
        assert!(f == 0);
    }

    #[test]
    fn test_b16() {
        fn use_b16<V: Into<U16>>(v: V) -> U16 {
            v.into()
        }
        fn use_u16(v: u16) -> u16 {
            v
        }

        let x: U16 = U16::from(0);
        assert_eq!(x.value(), 0);

        let x: U16 = U16::from(0b1111);
        assert_eq!(use_b16(x), x);
        assert_eq!(use_b16(0b1111), x);

        assert!(x == 0b1111);
        assert!(use_u16(x.into()) == 0b1111u16);

        assert!(U16::from(8).value() < 9);
    }

    // Ranges

    #[test]
    fn test_r2() {
        let v0 = R2::X0;
        let v1 = R2::X1;

        assert_eq!(v0, 0);
        assert_eq!(v1, 1);

        assert_eq!(v0, 0u8);
        assert_eq!(v1, 1u8);

        assert_eq!(v0, 0u16);
        assert_eq!(v1, 1u16);

        assert_eq!(v0, 0u32);
        assert_eq!(v1, 1u32);

        assert_eq!(v0, 0usize);
        assert_eq!(v1, 1usize);

        assert_eq!(v0, R2::from(0));
        assert_eq!(v1, R2::from(1));

        assert_eq!(v0, R2::from(0u8));
        assert_eq!(v1, R2::from(1u8));

        assert_eq!(v0, R2::from(0u16));
        assert_eq!(v1, R2::from(1u16));

        assert_eq!(v0, R2::from(0u32));
        assert_eq!(v1, R2::from(1u32));

        assert_eq!(v0, R2::from(0usize));
        assert_eq!(v1, R2::from(1usize));

        let x0: usize = v0.into();
        let x1: usize = v1.into();
        assert_eq!(x0, 0usize);
        assert_eq!(x1, 1usize);
    }

    #[test]
    fn test_range_convert() {
        let r2 = R2::X1;
        let r3: R3 = r2.into();
        let r4: R4 = r3.into();
        assert_eq!(r2.value(), r3.value());
        assert_eq!(r2.value(), r4.value());
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}",U1::from(0b1)),"0b1");
        assert_eq!(format!("{:?}",U2::from(0b11)),"0b11");
        assert_eq!(format!("{:?}",U3::from(0b111)),"0b111");
        assert_eq!(format!("{:?}",U4::from(0b1111)),"0b1111");
        assert_eq!(format!("{:?}",U5::from(0b11111)),"0b11111");
        assert_eq!(format!("{:?}",U6::from(0b111111)),"0b111111");
        assert_eq!(format!("{:?}",U8::from(0xffu16)),"0xff");
        assert_eq!(format!("{:?}",U12::from(0xfffu16)),"0xfff");
        assert_eq!(format!("{:?}",U16::from(0xffffu16)),"0xffff");
        assert_eq!(format!("{:?}",U20::from(0xfffffu32)),"0xfffff");
        assert_eq!(format!("{:?}",U24::from(0xffffffu32)),"0xffffff");
        assert_eq!(format!("{:?}",U28::from(0xfffffffu32)),"0xfffffff");
        assert_eq!(format!("{:?}",U32::from(0xffffffffu32)),"0xffffffff");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}",U1::from(0b1)),"1");
        assert_eq!(format!("{}",U2::from(0b11)),"3");
        assert_eq!(format!("{}",U3::from(0b111)),"7");
        assert_eq!(format!("{}",U4::from(0b1111)),"15");
        assert_eq!(format!("{}",U5::from(0b11111)),"31");
        assert_eq!(format!("{}",U6::from(0b111111)),"63");
        assert_eq!(format!("{}",U8::from(0xffu16)),"255");
        assert_eq!(format!("{}",U12::from(0xfffu16)),"4095");
        assert_eq!(format!("{}",U16::from(0xffffu16)),"65535");
        assert_eq!(format!("{}",U20::from(0xfffffu32)),"1048575");
        assert_eq!(format!("{}",U24::from(0xffffffu32)),"16777215");
        assert_eq!(format!("{}",U28::from(0xfffffffu32)),"268435455");
        assert_eq!(format!("{}",U32::from(0xffffffffu32)),"4294967295");
    }
}
