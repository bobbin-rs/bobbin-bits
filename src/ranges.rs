use core::mem::transmute;
use core::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R1 {
    X0 = 0x0,
}

impl From<u8> for R1 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x1);
        unsafe { transmute(other as usize) }
    }
}

impl From<R1> for u8 {
    #[inline]
    fn from(other: R1) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R1 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R1 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x1);
        unsafe { transmute(other as usize) }
    }
}

impl From<R1> for u16 {
    #[inline]
    fn from(other: R1) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R1 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R1 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x1);
        unsafe { transmute(other as usize) }
    }
}

impl From<R1> for u32 {
    #[inline]
    fn from(other: R1) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R1 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R1 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x1);
        unsafe { transmute(other as usize) }
    }
}

impl From<R1> for usize {
    #[inline]
    fn from(other: R1) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R1 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R1 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x1);
        unsafe { transmute(other as usize) }
    }
}

impl From<R1> for i32 {
    #[inline]
    fn from(other: R1) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R1 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R1 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R1 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R1 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R2 {
    X0 = 0x0,
    X1 = 0x1,
}

impl From<u8> for R2 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x2);
        unsafe { transmute(other as usize) }
    }
}

impl From<R2> for u8 {
    #[inline]
    fn from(other: R2) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R2 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R2 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x2);
        unsafe { transmute(other as usize) }
    }
}

impl From<R2> for u16 {
    #[inline]
    fn from(other: R2) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R2 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R2 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x2);
        unsafe { transmute(other as usize) }
    }
}

impl From<R2> for u32 {
    #[inline]
    fn from(other: R2) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R2 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R2 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x2);
        unsafe { transmute(other as usize) }
    }
}

impl From<R2> for usize {
    #[inline]
    fn from(other: R2) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R2 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R2 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x2);
        unsafe { transmute(other as usize) }
    }
}

impl From<R2> for i32 {
    #[inline]
    fn from(other: R2) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R2 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R2 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R3 {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
}

impl From<R2> for R3 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R3 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x3);
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for u8 {
    #[inline]
    fn from(other: R3) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R3 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R3 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x3);
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for u16 {
    #[inline]
    fn from(other: R3) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R3 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R3 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x3);
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for u32 {
    #[inline]
    fn from(other: R3) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R3 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R3 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x3);
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for usize {
    #[inline]
    fn from(other: R3) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R3 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R3 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x3);
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for i32 {
    #[inline]
    fn from(other: R3) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R3 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R3 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R4 {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
}

impl From<R2> for R4 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R4 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R4 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x4);
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for u8 {
    #[inline]
    fn from(other: R4) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R4 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R4 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x4);
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for u16 {
    #[inline]
    fn from(other: R4) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R4 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R4 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x4);
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for u32 {
    #[inline]
    fn from(other: R4) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R4 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R4 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x4);
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for usize {
    #[inline]
    fn from(other: R4) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R4 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R4 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x4);
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for i32 {
    #[inline]
    fn from(other: R4) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R4 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R4 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R4 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R4 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R5 {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
}

impl From<R2> for R5 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R5 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R5 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R5 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x5);
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for u8 {
    #[inline]
    fn from(other: R5) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R5 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R5 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x5);
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for u16 {
    #[inline]
    fn from(other: R5) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R5 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R5 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x5);
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for u32 {
    #[inline]
    fn from(other: R5) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R5 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R5 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x5);
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for usize {
    #[inline]
    fn from(other: R5) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R5 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R5 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x5);
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for i32 {
    #[inline]
    fn from(other: R5) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R5 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R5 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R5 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R5 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R6 {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
}

impl From<R2> for R6 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R6 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R6 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R6 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R6 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x6);
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for u8 {
    #[inline]
    fn from(other: R6) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R6 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R6 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x6);
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for u16 {
    #[inline]
    fn from(other: R6) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R6 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R6 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x6);
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for u32 {
    #[inline]
    fn from(other: R6) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R6 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R6 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x6);
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for usize {
    #[inline]
    fn from(other: R6) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R6 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R6 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x6);
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for i32 {
    #[inline]
    fn from(other: R6) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R6 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R6 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R6 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R6 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R7 {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
}

impl From<R2> for R7 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R7 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R7 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R7 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R7 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R7 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x7);
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for u8 {
    #[inline]
    fn from(other: R7) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R7 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R7 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x7);
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for u16 {
    #[inline]
    fn from(other: R7) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R7 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R7 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x7);
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for u32 {
    #[inline]
    fn from(other: R7) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R7 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R7 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x7);
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for usize {
    #[inline]
    fn from(other: R7) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R7 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R7 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x7);
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for i32 {
    #[inline]
    fn from(other: R7) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R7 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R7 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R7 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R7 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R7 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R8 {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
}

impl From<R2> for R8 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R8 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R8 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R8 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R8 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R8 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R8 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x8);
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for u8 {
    #[inline]
    fn from(other: R8) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R8 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R8 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x8);
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for u16 {
    #[inline]
    fn from(other: R8) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R8 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R8 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x8);
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for u32 {
    #[inline]
    fn from(other: R8) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R8 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R8 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x8);
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for usize {
    #[inline]
    fn from(other: R8) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R8 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R8 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x8);
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for i32 {
    #[inline]
    fn from(other: R8) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R8 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R8 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R8 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R8 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R9 {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
}

impl From<R2> for R9 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R9 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R9 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R9 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R9 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R9 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R9 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R9 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x9);
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for u8 {
    #[inline]
    fn from(other: R9) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R9 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R9 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x9);
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for u16 {
    #[inline]
    fn from(other: R9) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R9 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R9 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x9);
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for u32 {
    #[inline]
    fn from(other: R9) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R9 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R9 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x9);
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for usize {
    #[inline]
    fn from(other: R9) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R9 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R9 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x9);
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for i32 {
    #[inline]
    fn from(other: R9) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R9 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R9 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R9 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R9 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R10 {
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
}

impl From<R2> for R10 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R10 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R10 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R10 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R10 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R10 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R10 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R10 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R10 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0xa);
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for u8 {
    #[inline]
    fn from(other: R10) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R10 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R10 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0xa);
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for u16 {
    #[inline]
    fn from(other: R10) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R10 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R10 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0xa);
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for u32 {
    #[inline]
    fn from(other: R10) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R10 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R10 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0xa);
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for usize {
    #[inline]
    fn from(other: R10) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R10 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R10 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0xa);
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for i32 {
    #[inline]
    fn from(other: R10) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R10 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R10 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R10 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R10 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R10 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R11 {
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
}

impl From<R2> for R11 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R11 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R11 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R11 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R11 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R11 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R11 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R11 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R11 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R11 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0xb);
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for u8 {
    #[inline]
    fn from(other: R11) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R11 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R11 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0xb);
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for u16 {
    #[inline]
    fn from(other: R11) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R11 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R11 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0xb);
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for u32 {
    #[inline]
    fn from(other: R11) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R11 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R11 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0xb);
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for usize {
    #[inline]
    fn from(other: R11) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R11 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R11 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0xb);
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for i32 {
    #[inline]
    fn from(other: R11) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R11 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R11 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R11 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R11 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R11 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
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
    Xa = 0xa,
    Xb = 0xb,
}

impl From<R2> for R12 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R12 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R12 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R12 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R12 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R12 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R12 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R12 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R12 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R12 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R12 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0xc);
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for u8 {
    #[inline]
    fn from(other: R12) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R12 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R12 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0xc);
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for u16 {
    #[inline]
    fn from(other: R12) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R12 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R12 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0xc);
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for u32 {
    #[inline]
    fn from(other: R12) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R12 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R12 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0xc);
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for usize {
    #[inline]
    fn from(other: R12) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R12 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R12 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0xc);
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for i32 {
    #[inline]
    fn from(other: R12) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R12 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R12 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R12 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R12 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R13 {
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
}

impl From<R2> for R13 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R13 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R13 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R13 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R13 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R13 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R13 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R13 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R13 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R13 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R13 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R13 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0xd);
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for u8 {
    #[inline]
    fn from(other: R13) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R13 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R13 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0xd);
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for u16 {
    #[inline]
    fn from(other: R13) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R13 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R13 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0xd);
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for u32 {
    #[inline]
    fn from(other: R13) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R13 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R13 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0xd);
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for usize {
    #[inline]
    fn from(other: R13) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R13 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R13 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0xd);
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for i32 {
    #[inline]
    fn from(other: R13) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R13 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R13 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R13 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R13 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R13 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R14 {
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
}

impl From<R2> for R14 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R14 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R14 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R14 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R14 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R14 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R14 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R14 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R14 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R14 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R14 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R14 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R14 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0xe);
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for u8 {
    #[inline]
    fn from(other: R14) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R14 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R14 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0xe);
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for u16 {
    #[inline]
    fn from(other: R14) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R14 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R14 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0xe);
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for u32 {
    #[inline]
    fn from(other: R14) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R14 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R14 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0xe);
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for usize {
    #[inline]
    fn from(other: R14) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R14 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R14 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0xe);
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for i32 {
    #[inline]
    fn from(other: R14) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R14 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R14 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R14 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R14 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R14 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R15 {
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
}

impl From<R2> for R15 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R15 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R15 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R15 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R15 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R15 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R15 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R15 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R15 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R15 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R15 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R15 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R15 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R15 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0xf);
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for u8 {
    #[inline]
    fn from(other: R15) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R15 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R15 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0xf);
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for u16 {
    #[inline]
    fn from(other: R15) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R15 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R15 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0xf);
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for u32 {
    #[inline]
    fn from(other: R15) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R15 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R15 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0xf);
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for usize {
    #[inline]
    fn from(other: R15) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R15 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R15 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0xf);
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for i32 {
    #[inline]
    fn from(other: R15) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R15 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R15 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R15 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R15 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R16 {
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
}

impl From<R2> for R16 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R16 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R16 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R16 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R16 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R16 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R16 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R16 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R16 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R16 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R16 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R16 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R16 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R16 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R16 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x10);
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for u8 {
    #[inline]
    fn from(other: R16) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R16 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R16 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x10);
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for u16 {
    #[inline]
    fn from(other: R16) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R16 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R16 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x10);
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for u32 {
    #[inline]
    fn from(other: R16) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R16 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R16 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x10);
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for usize {
    #[inline]
    fn from(other: R16) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R16 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R16 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x10);
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for i32 {
    #[inline]
    fn from(other: R16) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R16 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R16 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:01x}", *self as u8)
    }
 }

impl fmt::Display for R16 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R16 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R17 {
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
}

impl From<R2> for R17 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R17 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R17 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R17 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R17 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R17 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R17 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R17 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R17 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R17 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R17 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R17 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R17 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R17 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R17 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R17 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x11);
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for u8 {
    #[inline]
    fn from(other: R17) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R17 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R17 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x11);
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for u16 {
    #[inline]
    fn from(other: R17) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R17 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R17 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x11);
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for u32 {
    #[inline]
    fn from(other: R17) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R17 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R17 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x11);
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for usize {
    #[inline]
    fn from(other: R17) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R17 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R17 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x11);
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for i32 {
    #[inline]
    fn from(other: R17) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R17 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R17 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R17 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R17 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R18 {
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
}

impl From<R2> for R18 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R18 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R18 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R18 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R18 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R18 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R18 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R18 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R18 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R18 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R18 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R18 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R18 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R18 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R18 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R18 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R18 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x12);
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for u8 {
    #[inline]
    fn from(other: R18) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R18 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R18 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x12);
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for u16 {
    #[inline]
    fn from(other: R18) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R18 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R18 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x12);
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for u32 {
    #[inline]
    fn from(other: R18) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R18 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R18 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x12);
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for usize {
    #[inline]
    fn from(other: R18) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R18 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R18 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x12);
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for i32 {
    #[inline]
    fn from(other: R18) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R18 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R18 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R18 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R18 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R18 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R19 {
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
}

impl From<R2> for R19 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R19 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R19 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R19 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R19 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R19 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R19 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R19 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R19 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R19 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R19 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R19 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R19 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R19 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R19 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R19 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R19 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R19 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x13);
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for u8 {
    #[inline]
    fn from(other: R19) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R19 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R19 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x13);
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for u16 {
    #[inline]
    fn from(other: R19) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R19 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R19 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x13);
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for u32 {
    #[inline]
    fn from(other: R19) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R19 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R19 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x13);
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for usize {
    #[inline]
    fn from(other: R19) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R19 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R19 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x13);
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for i32 {
    #[inline]
    fn from(other: R19) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R19 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R19 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R19 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R19 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R19 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R20 {
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
}

impl From<R2> for R20 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R20 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R20 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R20 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R20 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R20 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R20 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R20 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R20 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R20 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R20 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R20 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R20 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R20 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R20 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R20 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R20 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R20 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R20 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x14);
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for u8 {
    #[inline]
    fn from(other: R20) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R20 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R20 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x14);
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for u16 {
    #[inline]
    fn from(other: R20) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R20 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R20 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x14);
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for u32 {
    #[inline]
    fn from(other: R20) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R20 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R20 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x14);
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for usize {
    #[inline]
    fn from(other: R20) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R20 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R20 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x14);
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for i32 {
    #[inline]
    fn from(other: R20) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R20 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R20 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R20 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R20 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R20 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R21 {
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
}

impl From<R2> for R21 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R21 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R21 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R21 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R21 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R21 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R21 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R21 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R21 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R21 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R21 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R21 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R21 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R21 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R21 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R21 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R21 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R21 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R21 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R21 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x15);
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for u8 {
    #[inline]
    fn from(other: R21) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R21 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R21 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x15);
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for u16 {
    #[inline]
    fn from(other: R21) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R21 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R21 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x15);
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for u32 {
    #[inline]
    fn from(other: R21) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R21 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R21 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x15);
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for usize {
    #[inline]
    fn from(other: R21) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R21 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R21 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x15);
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for i32 {
    #[inline]
    fn from(other: R21) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R21 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R21 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R21 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R21 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R21 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R22 {
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
}

impl From<R2> for R22 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R22 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R22 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R22 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R22 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R22 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R22 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R22 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R22 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R22 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R22 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R22 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R22 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R22 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R22 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R22 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R22 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R22 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R22 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R22 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R22 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x16);
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for u8 {
    #[inline]
    fn from(other: R22) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R22 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R22 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x16);
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for u16 {
    #[inline]
    fn from(other: R22) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R22 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R22 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x16);
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for u32 {
    #[inline]
    fn from(other: R22) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R22 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R22 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x16);
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for usize {
    #[inline]
    fn from(other: R22) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R22 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R22 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x16);
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for i32 {
    #[inline]
    fn from(other: R22) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R22 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R22 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R22 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R22 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R22 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R23 {
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
}

impl From<R2> for R23 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R23 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R23 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R23 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R23 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R23 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R23 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R23 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R23 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R23 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R23 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R23 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R23 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R23 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R23 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R23 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R23 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R23 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R23 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R23 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R23 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R23 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x17);
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for u8 {
    #[inline]
    fn from(other: R23) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R23 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R23 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x17);
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for u16 {
    #[inline]
    fn from(other: R23) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R23 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R23 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x17);
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for u32 {
    #[inline]
    fn from(other: R23) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R23 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R23 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x17);
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for usize {
    #[inline]
    fn from(other: R23) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R23 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R23 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x17);
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for i32 {
    #[inline]
    fn from(other: R23) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R23 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R23 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R23 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R23 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R23 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R24 {
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
}

impl From<R2> for R24 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R24 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R24 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R24 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R24 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R24 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R24 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R24 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R24 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R24 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R24 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R24 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R24 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R24 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R24 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R24 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R24 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R24 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R24 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R24 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R24 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R24 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R24 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x18);
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for u8 {
    #[inline]
    fn from(other: R24) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R24 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R24 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x18);
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for u16 {
    #[inline]
    fn from(other: R24) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R24 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R24 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x18);
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for u32 {
    #[inline]
    fn from(other: R24) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R24 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R24 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x18);
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for usize {
    #[inline]
    fn from(other: R24) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R24 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R24 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x18);
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for i32 {
    #[inline]
    fn from(other: R24) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R24 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R24 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R24 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R24 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R25 {
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
}

impl From<R2> for R25 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R25 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R25 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R25 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R25 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R25 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R25 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R25 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R25 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R25 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R25 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R25 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R25 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R25 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R25 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R25 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R25 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R25 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R25 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R25 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R25 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R25 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R25 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R25 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x19);
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for u8 {
    #[inline]
    fn from(other: R25) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R25 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R25 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x19);
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for u16 {
    #[inline]
    fn from(other: R25) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R25 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R25 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x19);
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for u32 {
    #[inline]
    fn from(other: R25) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R25 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R25 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x19);
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for usize {
    #[inline]
    fn from(other: R25) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R25 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R25 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x19);
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for i32 {
    #[inline]
    fn from(other: R25) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R25 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R25 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R25 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R25 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R25 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R26 {
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
}

impl From<R2> for R26 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R26 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R26 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R26 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R26 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R26 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R26 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R26 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R26 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R26 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R26 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R26 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R26 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R26 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R26 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R26 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R26 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R26 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R26 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R26 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R26 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R26 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R26 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for R26 {
    #[inline]
    fn from(other: R25) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R26 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x1a);
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for u8 {
    #[inline]
    fn from(other: R26) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R26 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R26 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x1a);
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for u16 {
    #[inline]
    fn from(other: R26) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R26 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R26 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x1a);
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for u32 {
    #[inline]
    fn from(other: R26) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R26 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R26 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x1a);
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for usize {
    #[inline]
    fn from(other: R26) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R26 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R26 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x1a);
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for i32 {
    #[inline]
    fn from(other: R26) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R26 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R26 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R26 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R26 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R26 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R27 {
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
}

impl From<R2> for R27 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R27 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R27 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R27 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R27 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R27 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R27 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R27 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R27 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R27 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R27 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R27 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R27 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R27 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R27 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R27 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R27 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R27 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R27 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R27 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R27 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R27 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R27 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for R27 {
    #[inline]
    fn from(other: R25) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for R27 {
    #[inline]
    fn from(other: R26) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R27 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x1b);
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for u8 {
    #[inline]
    fn from(other: R27) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R27 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R27 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x1b);
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for u16 {
    #[inline]
    fn from(other: R27) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R27 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R27 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x1b);
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for u32 {
    #[inline]
    fn from(other: R27) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R27 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R27 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x1b);
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for usize {
    #[inline]
    fn from(other: R27) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R27 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R27 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x1b);
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for i32 {
    #[inline]
    fn from(other: R27) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R27 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R27 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R27 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R27 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R27 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R28 {
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
}

impl From<R2> for R28 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R28 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R28 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R28 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R28 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R28 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R28 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R28 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R28 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R28 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R28 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R28 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R28 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R28 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R28 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R28 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R28 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R28 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R28 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R28 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R28 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R28 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R28 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for R28 {
    #[inline]
    fn from(other: R25) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for R28 {
    #[inline]
    fn from(other: R26) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for R28 {
    #[inline]
    fn from(other: R27) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R28 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x1c);
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for u8 {
    #[inline]
    fn from(other: R28) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R28 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R28 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x1c);
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for u16 {
    #[inline]
    fn from(other: R28) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R28 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R28 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x1c);
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for u32 {
    #[inline]
    fn from(other: R28) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R28 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R28 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x1c);
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for usize {
    #[inline]
    fn from(other: R28) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R28 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R28 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x1c);
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for i32 {
    #[inline]
    fn from(other: R28) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R28 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R28 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R28 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R28 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R28 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R29 {
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
}

impl From<R2> for R29 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R29 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R29 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R29 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R29 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R29 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R29 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R29 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R29 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R29 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R29 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R29 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R29 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R29 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R29 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R29 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R29 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R29 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R29 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R29 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R29 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R29 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R29 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for R29 {
    #[inline]
    fn from(other: R25) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for R29 {
    #[inline]
    fn from(other: R26) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for R29 {
    #[inline]
    fn from(other: R27) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for R29 {
    #[inline]
    fn from(other: R28) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R29 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x1d);
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for u8 {
    #[inline]
    fn from(other: R29) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R29 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R29 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x1d);
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for u16 {
    #[inline]
    fn from(other: R29) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R29 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R29 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x1d);
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for u32 {
    #[inline]
    fn from(other: R29) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R29 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R29 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x1d);
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for usize {
    #[inline]
    fn from(other: R29) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R29 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R29 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x1d);
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for i32 {
    #[inline]
    fn from(other: R29) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R29 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R29 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R29 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R29 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R29 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R30 {
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
}

impl From<R2> for R30 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R30 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R30 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R30 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R30 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R30 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R30 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R30 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R30 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R30 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R30 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R30 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R30 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R30 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R30 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R30 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R30 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R30 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R30 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R30 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R30 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R30 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R30 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for R30 {
    #[inline]
    fn from(other: R25) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for R30 {
    #[inline]
    fn from(other: R26) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for R30 {
    #[inline]
    fn from(other: R27) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for R30 {
    #[inline]
    fn from(other: R28) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for R30 {
    #[inline]
    fn from(other: R29) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R30 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x1e);
        unsafe { transmute(other as usize) }
    }
}

impl From<R30> for u8 {
    #[inline]
    fn from(other: R30) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R30 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R30 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x1e);
        unsafe { transmute(other as usize) }
    }
}

impl From<R30> for u16 {
    #[inline]
    fn from(other: R30) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R30 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R30 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x1e);
        unsafe { transmute(other as usize) }
    }
}

impl From<R30> for u32 {
    #[inline]
    fn from(other: R30) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R30 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R30 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x1e);
        unsafe { transmute(other as usize) }
    }
}

impl From<R30> for usize {
    #[inline]
    fn from(other: R30) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R30 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R30 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x1e);
        unsafe { transmute(other as usize) }
    }
}

impl From<R30> for i32 {
    #[inline]
    fn from(other: R30) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R30 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R30 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R30 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R30 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R30 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R31 {
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
}

impl From<R2> for R31 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R31 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R31 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R31 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R31 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R31 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R31 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R31 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R31 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R31 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R31 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R31 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R31 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R31 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R31 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R31 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R31 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R31 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R31 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R31 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R31 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R31 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R31 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for R31 {
    #[inline]
    fn from(other: R25) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for R31 {
    #[inline]
    fn from(other: R26) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for R31 {
    #[inline]
    fn from(other: R27) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for R31 {
    #[inline]
    fn from(other: R28) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for R31 {
    #[inline]
    fn from(other: R29) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R30> for R31 {
    #[inline]
    fn from(other: R30) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R31 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x1f);
        unsafe { transmute(other as usize) }
    }
}

impl From<R31> for u8 {
    #[inline]
    fn from(other: R31) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R31 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R31 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x1f);
        unsafe { transmute(other as usize) }
    }
}

impl From<R31> for u16 {
    #[inline]
    fn from(other: R31) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R31 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R31 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x1f);
        unsafe { transmute(other as usize) }
    }
}

impl From<R31> for u32 {
    #[inline]
    fn from(other: R31) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R31 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R31 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x1f);
        unsafe { transmute(other as usize) }
    }
}

impl From<R31> for usize {
    #[inline]
    fn from(other: R31) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R31 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R31 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x1f);
        unsafe { transmute(other as usize) }
    }
}

impl From<R31> for i32 {
    #[inline]
    fn from(other: R31) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R31 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R31 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R31 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R31 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R31 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum R32 {
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
}

impl From<R2> for R32 {
    #[inline]
    fn from(other: R2) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R3> for R32 {
    #[inline]
    fn from(other: R3) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R4> for R32 {
    #[inline]
    fn from(other: R4) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R5> for R32 {
    #[inline]
    fn from(other: R5) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R6> for R32 {
    #[inline]
    fn from(other: R6) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R7> for R32 {
    #[inline]
    fn from(other: R7) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R8> for R32 {
    #[inline]
    fn from(other: R8) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R9> for R32 {
    #[inline]
    fn from(other: R9) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R10> for R32 {
    #[inline]
    fn from(other: R10) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R11> for R32 {
    #[inline]
    fn from(other: R11) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R12> for R32 {
    #[inline]
    fn from(other: R12) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R13> for R32 {
    #[inline]
    fn from(other: R13) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R14> for R32 {
    #[inline]
    fn from(other: R14) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R15> for R32 {
    #[inline]
    fn from(other: R15) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R16> for R32 {
    #[inline]
    fn from(other: R16) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R17> for R32 {
    #[inline]
    fn from(other: R17) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R18> for R32 {
    #[inline]
    fn from(other: R18) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R19> for R32 {
    #[inline]
    fn from(other: R19) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R20> for R32 {
    #[inline]
    fn from(other: R20) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R21> for R32 {
    #[inline]
    fn from(other: R21) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R22> for R32 {
    #[inline]
    fn from(other: R22) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R23> for R32 {
    #[inline]
    fn from(other: R23) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R24> for R32 {
    #[inline]
    fn from(other: R24) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R25> for R32 {
    #[inline]
    fn from(other: R25) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R26> for R32 {
    #[inline]
    fn from(other: R26) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R27> for R32 {
    #[inline]
    fn from(other: R27) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R28> for R32 {
    #[inline]
    fn from(other: R28) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R29> for R32 {
    #[inline]
    fn from(other: R29) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R30> for R32 {
    #[inline]
    fn from(other: R30) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<R31> for R32 {
    #[inline]
    fn from(other: R31) -> Self {
        unsafe { transmute(other as usize) }
    }
}

impl From<u8> for R32 {
    #[inline]
    fn from(other: u8) -> Self {
        assert!(other < 0x20);
        unsafe { transmute(other as usize) }
    }
}

impl From<R32> for u8 {
    #[inline]
    fn from(other: R32) -> u8 {
        other as u8
    }
}

impl PartialEq<u8> for R32 {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u16> for R32 {
    #[inline]
    fn from(other: u16) -> Self {
        assert!(other < 0x20);
        unsafe { transmute(other as usize) }
    }
}

impl From<R32> for u16 {
    #[inline]
    fn from(other: R32) -> u16 {
        other as u16
    }
}

impl PartialEq<u16> for R32 {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        *self as usize == *other as usize
    }
}

impl From<u32> for R32 {
    #[inline]
    fn from(other: u32) -> Self {
        assert!(other < 0x20);
        unsafe { transmute(other as usize) }
    }
}

impl From<R32> for u32 {
    #[inline]
    fn from(other: R32) -> u32 {
        other as u32
    }
}

impl PartialEq<u32> for R32 {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        *self as usize == *other as usize
    }
}

impl From<usize> for R32 {
    #[inline]
    fn from(other: usize) -> Self {
        assert!(other < 0x20);
        unsafe { transmute(other as usize) }
    }
}

impl From<R32> for usize {
    #[inline]
    fn from(other: R32) -> usize {
        other as usize
    }
}

impl PartialEq<usize> for R32 {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other as usize
    }
}

impl From<i32> for R32 {
    #[inline]
    fn from(other: i32) -> Self {
        assert!(other >= 0);
        assert!(other < 0x20);
        unsafe { transmute(other as usize) }
    }
}

impl From<R32> for i32 {
    #[inline]
    fn from(other: R32) -> i32 {
        other as i32
    }
}

impl PartialEq<i32> for R32 {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        *self as usize == *other as usize
    }
}

impl R32 {
    #[inline]
    pub fn value(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(other: u8) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u16_unchecked(other: u16) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_u32_unchecked(other: u32) -> Self {
        transmute(other as usize)
    }

    #[inline]
    pub unsafe fn from_usize_unchecked(other: usize) -> Self {
        transmute(other as usize)
    }
}

impl fmt::Debug for R32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x}", *self as u8)
    }
 }

impl fmt::Display for R32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

impl fmt::LowerHex for R32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       (*self as u8).fmt(f)
    }
 }

