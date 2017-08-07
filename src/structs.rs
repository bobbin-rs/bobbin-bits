use core::fmt;

macro_rules! impl_u8 {
    ($B:ident, $m:expr) => (
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $B(u8);

        impl $B {
            pub fn value(&self) -> u8 {
                self.0
            }

            pub unsafe fn from_u8_unchecked(other: u8) -> Self {
                $B(other as u8)
            }

            pub unsafe fn from_u16_unchecked(other: u16) -> Self {
                $B(other as u8)
            }

            pub unsafe fn from_u32_unchecked(other: u32) -> Self {
                $B(other as u8)
            }

            pub unsafe fn from_usize_unchecked(other: usize) -> Self {
                $B(other as u8)
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

            pub unsafe fn from_u8_unchecked(other: u8) -> Self {
                $B(other as u16)
            }

            pub unsafe fn from_u16_unchecked(other: u16) -> Self {
                $B(other as u16)
            }

            pub unsafe fn from_u32_unchecked(other: u32) -> Self {
                $B(other as u16)
            }

            pub unsafe fn from_usize_unchecked(other: usize) -> Self {
                $B(other as u16)
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

            pub unsafe fn from_u8_unchecked(other: u8) -> Self {
                $B(other as u32)
            }

            pub unsafe fn from_u16_unchecked(other: u16) -> Self {
                $B(other as u32)
            }

            pub unsafe fn from_u32_unchecked(other: u32) -> Self {
                $B(other as u32)
            }

            pub unsafe fn from_usize_unchecked(other: usize) -> Self {
                $B(other as u32)
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

