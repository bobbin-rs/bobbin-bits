use core::mem::transmute;
use core::ops::Not;
use core::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum U1 {
    B0 = 0b0,
    B1 = 0b1,
}

impl From<u8> for U1 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other & !0b1 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U1> for u8 {
    #[inline]
    fn from(other: U1) -> u8 {
        other as u8
    }
}

impl From<u16> for U1 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other & !0b1 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U1> for u16 {
    #[inline]
    fn from(other: U1) -> u16 {
        other as u16
    }
}

impl From<u32> for U1 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other & !0b1 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U1> for u32 {
    #[inline]
    fn from(other: U1) -> u32 {
        other as u32
    }
}

impl From<usize> for U1 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other & !0b1 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U1> for usize {
    #[inline]
    fn from(other: U1) -> usize {
        other as usize
    }
}

impl From<i32> for U1 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other & !0b1 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U1> for i32 {
    #[inline]
    fn from(other: U1) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for U1 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other 
    }
}

impl U1 {
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as u8)
    }
}

impl fmt::Debug for U1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0b{:0width$b}", *self as u8, width=1)
    }
 }

impl fmt::Display for U1 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for U1 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum U2 {
    B00 = 0b00,
    B01 = 0b01,
    B10 = 0b10,
    B11 = 0b11,
}

impl From<u8> for U2 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other & !0b11 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U2> for u8 {
    #[inline]
    fn from(other: U2) -> u8 {
        other as u8
    }
}

impl From<u16> for U2 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other & !0b11 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U2> for u16 {
    #[inline]
    fn from(other: U2) -> u16 {
        other as u16
    }
}

impl From<u32> for U2 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other & !0b11 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U2> for u32 {
    #[inline]
    fn from(other: U2) -> u32 {
        other as u32
    }
}

impl From<usize> for U2 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other & !0b11 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U2> for usize {
    #[inline]
    fn from(other: U2) -> usize {
        other as usize
    }
}

impl From<i32> for U2 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other & !0b11 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U2> for i32 {
    #[inline]
    fn from(other: U2) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for U2 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other 
    }
}

impl U2 {
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as u8)
    }
}

impl fmt::Debug for U2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0b{:0width$b}", *self as u8, width=2)
    }
 }

impl fmt::Display for U2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for U2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
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

impl From<u8> for U3 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other & !0b111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U3> for u8 {
    #[inline]
    fn from(other: U3) -> u8 {
        other as u8
    }
}

impl From<u16> for U3 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other & !0b111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U3> for u16 {
    #[inline]
    fn from(other: U3) -> u16 {
        other as u16
    }
}

impl From<u32> for U3 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other & !0b111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U3> for u32 {
    #[inline]
    fn from(other: U3) -> u32 {
        other as u32
    }
}

impl From<usize> for U3 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other & !0b111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U3> for usize {
    #[inline]
    fn from(other: U3) -> usize {
        other as usize
    }
}

impl From<i32> for U3 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other & !0b111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U3> for i32 {
    #[inline]
    fn from(other: U3) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for U3 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other 
    }
}

impl U3 {
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as u8)
    }
}

impl fmt::Debug for U3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0b{:0width$b}", *self as u8, width=3)
    }
 }

impl fmt::Display for U3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for U3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum U4 {
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
}

impl From<u8> for U4 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other & !0b1111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U4> for u8 {
    #[inline]
    fn from(other: U4) -> u8 {
        other as u8
    }
}

impl From<u16> for U4 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other & !0b1111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U4> for u16 {
    #[inline]
    fn from(other: U4) -> u16 {
        other as u16
    }
}

impl From<u32> for U4 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other & !0b1111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U4> for u32 {
    #[inline]
    fn from(other: U4) -> u32 {
        other as u32
    }
}

impl From<usize> for U4 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other & !0b1111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U4> for usize {
    #[inline]
    fn from(other: U4) -> usize {
        other as usize
    }
}

impl From<i32> for U4 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other & !0b1111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U4> for i32 {
    #[inline]
    fn from(other: U4) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for U4 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other 
    }
}

impl U4 {
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as u8)
    }
}

impl fmt::Debug for U4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0b{:0width$b}", *self as u8, width=4)
    }
 }

impl fmt::Display for U4 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for U4 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum U5 {
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
}

impl From<u8> for U5 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other & !0b11111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U5> for u8 {
    #[inline]
    fn from(other: U5) -> u8 {
        other as u8
    }
}

impl From<u16> for U5 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other & !0b11111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U5> for u16 {
    #[inline]
    fn from(other: U5) -> u16 {
        other as u16
    }
}

impl From<u32> for U5 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other & !0b11111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U5> for u32 {
    #[inline]
    fn from(other: U5) -> u32 {
        other as u32
    }
}

impl From<usize> for U5 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other & !0b11111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U5> for usize {
    #[inline]
    fn from(other: U5) -> usize {
        other as usize
    }
}

impl From<i32> for U5 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other & !0b11111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U5> for i32 {
    #[inline]
    fn from(other: U5) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for U5 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other 
    }
}

impl U5 {
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as u8)
    }
}

impl fmt::Debug for U5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0b{:0width$b}", *self as u8, width=5)
    }
 }

impl fmt::Display for U5 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for U5 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum U6 {
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
}

impl From<u8> for U6 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other & !0b111111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U6> for u8 {
    #[inline]
    fn from(other: U6) -> u8 {
        other as u8
    }
}

impl From<u16> for U6 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other & !0b111111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U6> for u16 {
    #[inline]
    fn from(other: U6) -> u16 {
        other as u16
    }
}

impl From<u32> for U6 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other & !0b111111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U6> for u32 {
    #[inline]
    fn from(other: U6) -> u32 {
        other as u32
    }
}

impl From<usize> for U6 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other & !0b111111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U6> for usize {
    #[inline]
    fn from(other: U6) -> usize {
        other as usize
    }
}

impl From<i32> for U6 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other & !0b111111 == 0);
        unsafe { transmute(other as u8) }
    }
}

impl From<U6> for i32 {
    #[inline]
    fn from(other: U6) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for U6 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other 
    }
}

impl U6 {
    #[inline]
    pub fn value(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as u8)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as u8)
    }
}

impl fmt::Debug for U6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0b{:0width$b}", *self as u8, width=6)
    }
 }

impl fmt::Display for U6 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for U6 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u8).fmt(f)
    }
 }

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
