#![feature(macro_rules)]

pub mod numeric;

#[cfg(test)]
mod test {
    use numeric::{CryptoU64, CryptoI64, CryptoU32, CryptoI32, CryptoU8, CryptoI8};

    #[test]
    fn wrap_unwrap() {
        assert_eq!(CryptoU64::new(0x5040302010).value, 0x5040302010);
        assert_eq!(CryptoI64::new(-0x5040302010).value, -0x5040302010);
        assert_eq!(CryptoU32::new(0x40302010).value, 0x40302010);
        assert_eq!(CryptoI32::new(-0x40302010).value, -0x40302010);
        assert_eq!(CryptoU8::new(0x12).value, 0x12);
        assert_eq!(CryptoI8::new(-0x23).value, -0x23);
    }

    #[test]
    #[allow(unsigned_negation)]
    fn u8_arithmetic() {
        for &(a, b) in [(4,5), (0x80, 0xff), (0x00, 0x45), (0xff, 0xfe), (0x20, 0x20)].iter() {
            //assert_eq!((CryptoU32::new(a) * CryptoU32::new(b)).value, a * b);
            assert_eq!((CryptoU32::new(a) + CryptoU32::new(b)).value, a + b);
            assert_eq!((CryptoU32::new(a) - CryptoU32::new(b)).value, a - b);
            assert_eq!((CryptoU32::new(a) ^ CryptoU32::new(b)).value, a ^ b);
            assert_eq!((CryptoU32::new(a) | CryptoU32::new(b)).value, a | b);
            assert_eq!((!CryptoU32::new(a)).value, !a);
            assert_eq!((!CryptoU32::new(b)).value, !b);
            assert_eq!((-CryptoU32::new(a)).value, -a);
            assert_eq!((-CryptoU32::new(b)).value, -b);
        }
    }

    #[test]
    #[allow(unsigned_negation)]
    fn u32_arithmetic() {
        for &(a, b) in [(4,5), (0xffffffff, 0xffffffff), (0x80000000, 2), (0x7fffffff, 0x12345678)].iter() {
            assert_eq!((CryptoU32::new(a) * CryptoU32::new(b)).value, a * b);
            assert_eq!((CryptoU32::new(a) + CryptoU32::new(b)).value, a + b);
            assert_eq!((CryptoU32::new(a) - CryptoU32::new(b)).value, a - b);
            assert_eq!((CryptoU32::new(a) ^ CryptoU32::new(b)).value, a ^ b);
            assert_eq!((CryptoU32::new(a) | CryptoU32::new(b)).value, a | b);
            assert_eq!((!CryptoU32::new(a)).value, !a);
            assert_eq!((!CryptoU32::new(b)).value, !b);
            assert_eq!((-CryptoU32::new(a)).value, -a);
            assert_eq!((-CryptoU32::new(b)).value, -b);
        }
    }

    #[test]
    fn i32_arithmetic() {
        for &(a, b) in [(4,5), (-1, -1), (-0x80000000, 2), (0x7fffffff, 0x12345678)].iter() {
            assert_eq!((CryptoI32::new(a) * CryptoI32::new(b)).value, a * b);
            assert_eq!((CryptoI32::new(a) + CryptoI32::new(b)).value, a + b);
            assert_eq!((CryptoI32::new(a) - CryptoI32::new(b)).value, a - b);
            assert_eq!((CryptoI32::new(a) ^ CryptoI32::new(b)).value, a ^ b);
            assert_eq!((CryptoI32::new(a) | CryptoI32::new(b)).value, a | b);
            assert_eq!((!CryptoI32::new(a)).value, !a);
            assert_eq!((!CryptoI32::new(b)).value, !b);
            assert_eq!((-CryptoI32::new(a)).value, -a);
            assert_eq!((-CryptoI32::new(b)).value, -b);
        }
    }

    #[test]
    #[allow(unsigned_negation)]
    fn u64_arithmetic() {
        for &(a, b) in [(4,5), (0xffffffffffffffff, 0xffffffffffffffff), (0x180000000, 2), (0x7fffffff, 0x12345678)].iter() {
            assert_eq!((CryptoU64::new(a) * CryptoU64::new(b)).value, a * b);
            assert_eq!((CryptoU64::new(a) + CryptoU64::new(b)).value, a + b);
            assert_eq!((CryptoU64::new(a) - CryptoU64::new(b)).value, a - b);
            assert_eq!((CryptoU64::new(a) ^ CryptoU64::new(b)).value, a ^ b);
            assert_eq!((CryptoU64::new(a) | CryptoU64::new(b)).value, a | b);
            assert_eq!((!CryptoU64::new(a)).value, !a);
            assert_eq!((!CryptoU64::new(b)).value, !b);
            assert_eq!((-CryptoU64::new(a)).value, -a);
            assert_eq!((-CryptoU64::new(b)).value, -b);
        }
    }

    #[test]
    fn i64_arithmetic() {
        for &(a, b) in [(4,5), (-1, -1), (-0x80000000, 2), (0x7fffffff, 0x12345678)].iter() {
            assert_eq!((CryptoI64::new(a) * CryptoI64::new(b)).value, a * b);
            assert_eq!((CryptoI64::new(a) + CryptoI64::new(b)).value, a + b);
            assert_eq!((CryptoI64::new(a) - CryptoI64::new(b)).value, a - b);
            assert_eq!((CryptoI64::new(a) ^ CryptoI64::new(b)).value, a ^ b);
            assert_eq!((CryptoI64::new(a) | CryptoI64::new(b)).value, a | b);
            assert_eq!((!CryptoI64::new(a)).value, !a);
            assert_eq!((!CryptoI64::new(b)).value, !b);
            assert_eq!((-CryptoI64::new(a)).value, -a);
            assert_eq!((-CryptoI64::new(b)).value, -b);
        }
    }
}