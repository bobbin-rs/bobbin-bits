#![no_std]

use core::ops::Not;
use core::fmt;

pub mod enums;
pub use enums::*;

macro_rules! impl_u8 {
    ($B:ident, $m:expr) => (
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $B(u8);

        impl $B {
            pub fn value(&self) -> u8 {
                self.0
            }
        }

        impl From<i32> for $B {
            fn from(other: i32) -> Self {
                assert!(other >= 0);
                assert!(other & !$m == 0);
                $B(other as u8)
            }
        }

        impl From<usize> for $B {
            fn from(other: usize) -> Self {
                assert!(other & !$m == 0);
                $B(other as u8)
            }
        }

        impl From<u32> for $B {
            fn from(other: u32) -> Self {
                assert!(other & !$m == 0);
                $B(other as u8)
            }
        }

        impl From<u16> for $B {
            fn from(other: u16) -> Self {
                assert!(other & !$m == 0);
                $B(other as u8)
            }
        }

        impl From<u8> for $B {
            fn from(other: u8) -> Self {
                $B(other as u8)
            }
        }

        impl From<$B> for u32 {
            fn from(other: $B) -> Self {
                other.0.into()
            }
        }

        impl From<$B> for u16 {
            fn from(other: $B) -> Self {
                other.0.into()
            }
        }

        impl From<$B> for u8 {
            fn from(other: $B) -> Self {
                other.0.into()
            }
        }

        impl PartialEq<i32> for $B {
            fn eq(&self, other: &i32) -> bool {
                self.0 as i32 == *other
            }
        }

        impl fmt::Debug for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if $m <= 0b1111 {
                    write!(f, "0x{:01x}", self.0)
                } else {
                    write!(f, "0x{:02x}", self.0)                    
                }
            }
        }

        impl fmt::Display for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)     
            }
        }

        impl fmt::LowerHex for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)     
            }
        }

    )
}


macro_rules! impl_u16 {
    ($B:ident, $m:expr) => (
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $B(u16);

        impl $B {
            pub fn value(&self) -> u16 {
                self.0
            }
        }

        impl From<i32> for $B {
            fn from(other: i32) -> Self {
                assert!(other >= 0);
                assert!(other & !$m == 0);
                $B(other as u16)
            }
        }

        impl From<usize> for $B {
            fn from(other: usize) -> Self {
                assert!(other & !$m == 0);
                $B(other as u16)
            }
        }

        impl From<u32> for $B {
            fn from(other: u32) -> Self {
                assert!(other & !$m == 0);
                $B(other as u16)
            }
        }

        impl From<u16> for $B {
            fn from(other: u16) -> Self {
                assert!(other & !$m == 0);
                $B(other as u16)
            }
        }

        impl From<u8> for $B {
            fn from(other: u8) -> Self {
                $B(other as u16)
            }
        }

        impl From<$B> for u32 {
            fn from(other: $B) -> Self {
                other.0.into()
            }
        }

        impl From<$B> for u16 {
            fn from(other: $B) -> Self {
                other.0.into()
            }
        }

        impl From<$B> for u8 {
            fn from(other: $B) -> Self {
                assert!((other.0 & !0xff) == 0);
                other.0 as u8
            }
        }

        impl PartialEq<i32> for $B {
            fn eq(&self, other: &i32) -> bool {
                self.0 as i32 == *other
            }
        }

        impl fmt::Debug for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if $m <= 0b1111_1111_1111 {
                    write!(f, "0x{:03x}", self.0)
                } else {
                    write!(f, "0x{:04x}", self.0)                    
                }
            }
        }

        impl fmt::Display for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)     
            }
        }

        impl fmt::LowerHex for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)     
            }
        }

    )
}


macro_rules! impl_u32 {
    ($B:ident, $m:expr) => (
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $B(u32);

        impl $B {
            pub fn value(&self) -> u32 {
                self.0
            }
        }

        impl From<i32> for $B {
            fn from(other: i32) -> Self {
                assert!(other > 0);
                assert!(other as u32 & !$m == 0);
                $B(other as u32)
            }
        }

        impl From<usize> for $B {
            fn from(other: usize) -> Self {
                assert!(other & !$m == 0);
                $B(other as u32)
            }
        }

        impl From<u32> for $B {
            fn from(other: u32) -> Self {
                assert!(other & !$m == 0);
                $B(other as u32)
            }
        }

        impl From<u16> for $B {
            fn from(other: u16) -> Self {
                $B(other as u32)
            }
        }

        impl From<u8> for $B {
            fn from(other: u8) -> Self {
                $B(other as u32)
            }
        }

        impl From<$B> for u32 {
            fn from(other: $B) -> Self {
                other.0.into()
            }
        }

        impl From<$B> for u16 {
            fn from(other: $B) -> Self {
                assert!((other.0 & !0xffff) == 0);
                other.0 as u16
            }
        }

        impl From<$B> for u8 {
            fn from(other: $B) -> Self {
                assert!((other.0 & !0xff) == 0);
                other.0 as u8
            }
        }

        impl PartialEq<i32> for $B {
            fn eq(&self, other: &i32) -> bool {
                self.0 == *other as u32
            }
        }

        impl fmt::Debug for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if $m <= 0b1111_1111_1111_1111_1111u32 {
                    write!(f, "0x{:05x}", self.0)
                } else if $m <= 0b1111_1111_1111_1111_1111_1111u32 {
                    write!(f, "0x{:06x}", self.0)                    
                } else if $m <= 0b1111_1111_1111_1111_1111_1111_1111u32 {
                    write!(f, "0x{:07x}", self.0)                    
                } else {
                    write!(f, "0x{:08x}", self.0)                    
                }
            }
        }

        impl fmt::Display for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)     
            }
        }

        impl fmt::LowerHex for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)     
            }
        }
    )
}

// impl_u8!(U5, 0b1_1111);
// impl_u8!(U6, 0b11_1111);
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

impl From<bool> for U1 {
    fn from(other: bool) -> U1 {
        U1::from(other as u8)
    }
}

impl U1 {
    pub fn is_set(&self) -> bool {
        *self as u8 != 0
    }
}

impl Not for U1 {
    type Output = U1;
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

        let x: U16 = U16::from(0b1111);
        assert_eq!(use_b16(x), x);
        assert_eq!(use_b16(0b1111), x);

        assert!(x == 0b1111);
        assert!(use_u16(x.into()) == 0b1111u16);

        assert!(U16::from(8).value() < 9);

    }

    // #[test]
    // fn test_debug() {
    //     assert_eq!(format!("{:?}",U1::from(0b1)),"0b1");
    //     assert_eq!(format!("{:?}",U2::from(0b11)),"0b11");
    //     assert_eq!(format!("{:?}",U3::from(0b111)),"0b111");
    //     assert_eq!(format!("{:?}",U4::from(0b1111)),"0b1111");
    //     assert_eq!(format!("{:?}",U5::from(0b11111)),"0b11111");
    //     assert_eq!(format!("{:?}",U6::from(0b111111)),"0b111111");
    //     assert_eq!(format!("{:?}",U8::from(0xffu16)),"0xff");
    //     assert_eq!(format!("{:?}",U12::from(0xfffu16)),"0xfff");
    //     assert_eq!(format!("{:?}",U16::from(0xffffu16)),"0xffff");
    //     assert_eq!(format!("{:?}",U20::from(0xfffffu32)),"0xfffff");
    //     assert_eq!(format!("{:?}",U24::from(0xffffffu32)),"0xffffff");
    //     assert_eq!(format!("{:?}",U28::from(0xfffffffu32)),"0xfffffff");
    //     assert_eq!(format!("{:?}",U32::from(0xffffffffu32)),"0xffffffff");
    // }

    // #[test]
    // fn test_display() {
    //     assert_eq!(format!("{}",U1::from(0b1)),"1");
    //     assert_eq!(format!("{}",U2::from(0b11)),"3");
    //     assert_eq!(format!("{}",U3::from(0b111)),"7");
    //     assert_eq!(format!("{}",U4::from(0b1111)),"15");
    //     assert_eq!(format!("{}",U5::from(0b11111)),"31");
    //     assert_eq!(format!("{}",U6::from(0b111111)),"63");
    //     assert_eq!(format!("{}",U8::from(0xffu16)),"255");
    //     assert_eq!(format!("{}",U12::from(0xfffu16)),"4095");
    //     assert_eq!(format!("{}",U16::from(0xffffu16)),"65535");
    //     assert_eq!(format!("{}",U20::from(0xfffffu32)),"1048575");
    //     assert_eq!(format!("{}",U24::from(0xffffffu32)),"16777215");
    //     assert_eq!(format!("{}",U28::from(0xfffffffu32)),"268435455");
    //     assert_eq!(format!("{}",U32::from(0xffffffffu32)),"4294967295");
    // }
}
