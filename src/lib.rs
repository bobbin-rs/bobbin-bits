#![no_std]

pub mod enums;
pub mod structs;
pub mod ranges;

pub use enums::*;
pub use structs::*;
pub use ranges::*;


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
