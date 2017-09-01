macro_rules! impl_enum {
    ($id:ident, $width:expr, $mask:expr, $($a:ident = $b:expr,) *) => (        
        #[derive(PartialEq, Eq, Clone, Copy)]
        #[repr(u8)]
        pub enum $id {
            $($a = $b, )*
        }

        impl From<u8> for $id {
            #[inline]
            fn from(other: u8) -> Self {
                assert!(other & !$mask == 0);
                unsafe { transmute(other as u8) }
            }
        }

        impl From<$id> for u8 {
            #[inline]
            fn from(other: $id) -> u8 {
                other as u8
            }
        }

        impl From<u16> for $id {
            #[inline]
            fn from(other: u16) -> Self {
                assert!(other & !$mask == 0);
                unsafe { transmute(other as u8) }
            }
        }

        impl From<$id> for u16 {
            #[inline]
            fn from(other: $id) -> u16 {
                other as u16
            }
        }

        impl From<u32> for $id {
            #[inline]
            fn from(other: u32) -> Self {
                assert!(other & !$mask == 0);
                unsafe { transmute(other as u8) }
            }
        }

        impl From<$id> for u32 {
            #[inline]
            fn from(other: $id) -> u32 {
                other as u32
            }
        }

        impl From<usize> for $id {
            #[inline]
            fn from(other: usize) -> Self {
                assert!(other & !$mask == 0);
                unsafe { transmute(other as u8) }
            }
        }

        impl From<$id> for usize {
            #[inline]
            fn from(other: $id) -> usize {
                other as usize
            }
        }

        impl From<i32> for $id {
            #[inline]
            fn from(other: i32) -> Self {
                assert!(other >= 0);
                assert!(other & !$mask == 0);
                unsafe { transmute(other as u8) }
            }
        }

        impl From<$id> for i32 {
            #[inline]
            fn from(other: $id) -> i32 {
                other as i32
            }
        }

        impl PartialEq<i32> for $id {
            #[inline]
            fn eq(&self, other: &i32) -> bool {
                *self as i32 == *other 
            }
        }

        impl $id {
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

            #[inline]
            pub fn into_u8(self) -> u8 {
                self as u8
            }

            #[inline]
            pub fn into_u16(self) -> u16 {
                self as u16
            }

            #[inline]
            pub fn into_u32(self) -> u32 {
                self as u32
            }

            #[inline]
            pub fn into_usize(self) -> usize {
                self as usize
            }

            #[inline]
            pub fn into_i32(self) -> i32 {
                self as i32
            }
        }

        impl fmt::Debug for $id {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0b{:0width$b}", *self as u8, width=$width)
            }
        }

        impl fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                (*self as u8).fmt(f)
            }
        }

        impl fmt::LowerHex for $id {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                (*self as u8).fmt(f)
            }
        }        
    )
}