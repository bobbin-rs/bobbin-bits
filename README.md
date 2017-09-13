 # Bobbin Bits

Bobbin Bits defines a set of types for representing binary numbers of width 1 to 32 and ranged values from 1 to 32. These are intended to be useful for representing bit fields within peripheral registers and network protocols and for indexing small collections.

## Motivation

Rust currently does not directly support integer range types or unsigned integer types that 
are not u8, u16, u32, u64 or u128. Because of this, values are commonly stored in the next largest integer type and run-time range checks are used at function boundaries. Also,
matches usually require a fall-through handler even if all of the intended values
are covered.

One solution is to define ad-hoc structs or enums to represent domain-specific values
that are known to be in a specific range. This can eliminate run-time range checks at the cost of a large amount of boilerplate for managing conversions to and from these values. 

For some APIs the code for managing these types ends up much larger than the API itself.
It can also prove to be a significant documentation challenge and barrier to learning the API. Having a unique type for almost every function parameter in an API is undesirable.

This crate takes a different approach, defining a set of general-purpose types
useful for representing bit fields <= 32 bits and integer ranges from 1 through 32. Each
type supports conversions to and from Rust unsigned integer types and i32, performing range checking where needed.

## Representation

Types U1 through U6 are enums with repr(u8), allowing exhaustive matching without a default
case. Their members are named with the prefix "B" followed by the
n-bit binary representation of that number, like this:

```
#[repr(u8)]
pub enum U3 {
    B000 = 0b000,
    B001 = 0b001,
    B010 = 0b010,
    B011 = 0b011,
    B100 = 0b100,
    B101 = 0b101,
    B110 = 0b110,
    B111 = 0b111,
}
```

This allows zero-cost conversions when values are known to fit within the type.

Similarly, R1 through R32 are enums with repr(usize). Their members are named with
the prefix "X" followed by the hexadecimal represention of the number, single digits
for 0-15 and two digits for 16-32:

```
#[repr(usize)]
pub enum R12 {
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
    XA = 0xa,
    XB = 0xb,
}
```

Types U7 and U8, U9 to U16 and U16 through U32 are wrappers around u8, u16 and u32 respectively:

```
pub struct U20(u16);
```

Unfortunately there is no literal representation of these values, so they must be
constructed using `From<T>` conversions.

## Supported Traits

The following traits are currently supported for all types:

- `Debug for T`
- `Display for T`
- `LowerHex for T`
- `From<u8> for T`
- `From<T> for u8`
- `From<u16> for T`
- `From<T> for u16`
- `From<u32> for T`
- `From<T> for u32`
- `From<usize> for T`
- `From<T> for usize`
- `From<i32> for T`
- `From<T> for i32`
- `PartialEq<i32> for T`

The following additional traits are also supported for U1:

- `From<bool> for U1`
- `Not for U1`

## Examples

Here's an example of using the U4 bit field type:

```
use bobbin_bits::*;

// Implemented using a single exhaustive match statement
fn to_hex_char<V: Into<U4>>(v: V) -> char {
    let v = v.into();
    match v {
        U4::B0000 => '0',
        U4::B0001 => '1',
        U4::B0010 => '2',
        U4::B0011 => '3',
        U4::B0100 => '4',
        U4::B0101 => '5',
        U4::B0110 => '6',
        U4::B0111 => '7',
        U4::B1000 => '8',
        U4::B1001 => '9',
        U4::B1010 => 'a',
        U4::B1011 => 'b',
        U4::B1100 => 'c',
        U4::B1101 => 'd',
        U4::B1110 => 'e',
        U4::B1111 => 'f',
    }
}

// Call with a U4 bit field, no conversion or range checking is required.
let c = to_hex_char(U4::B1000);
assert_eq!(c, '8');

// Call with a i32, v.into() performs range check.
let c = to_hex_char(8);
assert_eq!(c, '8');

// Call with a u8, v.into() performs range check.
let c = to_hex_char(8_u8);
assert_eq!(c, '8');

// Perform range check from u32 outside of function
let v: U4 = 8u32.into();
let c = to_hex_char(v);
assert_eq!(c, '8');

// A function that will extract bits [4:7] from a u32 value
// without range checking
fn extract_u4(v: u32) -> U4 {
    unsafe {
        U4::from_u32_unchecked(v >> 4 & 0b1111)
    }
}

// No range checking needs to take place if a U4 is used
// through the computation
let c = to_hex_char(extract_u4(0b0000_0000_1000_0000));
assert_eq!(c, '8');

```

Using the U12 and U13 types:

```
use bobbin_bits::*;

fn double_sample<V: Into<U12>>(v: V) -> U13 {
    let v = v.into();
    // Extracts into u16, multiplies, then wraps into U13
    // Performs range checking when creating the U13 value
    U13::from(v.value() * 2)
}

// Range checking takes place within double_sample()
let v = double_sample(1000);

// Unfortunately, no literal form for U13, so range checking
// happens when constructing U13 value from u16
assert_eq!(v, U13::from(2000));

// When converting from types that cannot overflow the range (such as u8),
// no range checking is needed.
assert_eq!(double_sample(100), U13::from(200u8));

// You can always access the underlying representation of the value
assert_eq!(v.value(), 2000u16);

```

Using the R4 range type, which supports values 0 to 3:

```
use bobbin_bits::*;

// Using R4 in an exhaustive match
fn get_port_name<I: Into<R4>>(index: I) -> &'static str {
    let index = index.into();
    match index {
        R4::X0 => "PORTA",
        R4::X1 => "PORTB",
        R4::X2 => "PORTC",
        R4::X3 => "PORTD",
    }
}

pub const PORT_ADDR: [u32;4] = [0x1000_0000, 0x1000_2000, 0x1000_3000, 0x1000_4000];

// Using a lookup table
fn get_port_address<I: Into<R4>>(index: I) -> u32 {
    // Is the optimizing compiler smart enough to eliminate the
    // bounds check here?
    PORT_ADDR[index.into() as usize]
}

// From<i32> is implemented, range check happens in get_port_name()
let n = get_port_name(2);
assert_eq!(n, "PORTC");

// Using R4::X2 does not need a range check
let n = get_port_name(R4::X2);
assert_eq!(n, "PORTC");
```