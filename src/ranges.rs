macro_rules! impl_range {
    ($id:ident, $max:expr, $($a:ident = $b:expr,) *) => (        
        #[derive(PartialEq, Eq, Clone, Copy)]
        #[repr(usize)]
        pub enum $id {
            $($a = $b, )*
        }
        impl From<u8> for $id {
            #[inline]
            fn from(other: u8) -> Self {
                assert!(other < $max);
                unsafe { transmute(other as usize) }
            }
        }

        impl From<$id> for u8 {
            #[inline]
            fn from(other: $id) -> u8 {
                other as u8
            }
        }

        impl PartialEq<u8> for $id {
            #[inline]
            fn eq(&self, other: &u8) -> bool {
                *self as usize == *other as usize
            }
        }

        impl From<u16> for $id {
            #[inline]
            fn from(other: u16) -> Self {
                assert!(other < $max);
                unsafe { transmute(other as usize) }
            }
        }

        impl From<$id> for u16 {
            #[inline]
            fn from(other: $id) -> u16 {
                other as u16
            }
        }

        impl PartialEq<u16> for $id {
            #[inline]
            fn eq(&self, other: &u16) -> bool {
                *self as usize == *other as usize
            }
        }

        impl From<u32> for $id {
            #[inline]
            fn from(other: u32) -> Self {
                assert!(other < $max);
                unsafe { transmute(other as usize) }
            }
        }

        impl From<$id> for u32 {
            #[inline]
            fn from(other: $id) -> u32 {
                other as u32
            }
        }

        impl PartialEq<u32> for $id {
            #[inline]
            fn eq(&self, other: &u32) -> bool {
                *self as usize == *other as usize
            }
        }

        impl From<usize> for $id {
            #[inline]
            fn from(other: usize) -> Self {
                assert!(other < $max);
                unsafe { transmute(other as usize) }
            }
        }

        impl From<$id> for usize {
            #[inline]
            fn from(other: $id) -> usize {
                other as usize
            }
        }

        impl PartialEq<usize> for $id {
            #[inline]
            fn eq(&self, other: &usize) -> bool {
                *self as usize == *other as usize
            }
        }

        impl From<i32> for $id {
            #[inline]
            fn from(other: i32) -> Self {
                assert!(other >= 0);
                assert!(other < $max);
                unsafe { transmute(other as usize) }
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
                *self as usize == *other as usize
            }
        }

        impl $id {
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

        impl fmt::Debug for $id {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:x}", *self as u8)
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

macro_rules! impl_range_from {
    ($id:ident $(, $other:ident)* ) => (
        $(
        impl From<$other> for $id {
            #[inline]
            fn from(other: $other) -> Self {
                unsafe { transmute(other as usize) }
            }
        }
        )*
    )
}