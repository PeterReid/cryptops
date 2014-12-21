
macro_rules! define_numeric_type(
    ($name:ident, $inner_type:ident) => (
        #[deriving(Copy, Show)]
        pub struct $name {
            pub value: $inner_type
        }

        impl $name {
            pub   fn new(value: $inner_type) -> $name {
                $name{value: value}
            }
        }
    )
)

define_numeric_type!(CryptoU64, u64)
define_numeric_type!(CryptoI64, i64)
define_numeric_type!(CryptoU32, u32)
define_numeric_type!(CryptoI32, i32)
define_numeric_type!(CryptoU8, u8)
define_numeric_type!(CryptoI8, i8)

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! define_binary_op(
    ($thetype:ident, $thetrait:ident, $function:ident, $asm:expr) => (
        impl $thetrait<$thetype, $thetype> for $thetype {
            fn $function(self, other: $thetype) -> $thetype {
                let mut ret;
                unsafe {
                    asm!($asm : "=r"(ret) : "r"(other.value), "0"(self.value) : "cc");
                }
                $thetype::new(ret)
            }
        }
    )
)

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! define_unary_op(
    ($thetype:ident, $thetrait:ident, $function:ident, $asm:expr) => (
        impl $thetrait<$thetype> for $thetype {
            fn $function(&self) -> $thetype {
                let mut ret;
                unsafe {
                    asm!($asm : "=r"(ret) : "r"(self.value) : "cc");
                }
                $thetype::new(ret)
            }
        }
    )
)

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoU32, Add, add, "add $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoI32, Add, add, "add $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoU32, Sub, sub, "sub $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoI32, Sub, sub, "sub $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoU32, Mul, mul, "imul $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoI32, Mul, mul, "imul $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoU32, BitXor, bitxor, "xor $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoI32, BitXor, bitxor, "xor $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoU32, BitOr, bitor, "or $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoI32, BitOr, bitor, "or $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoU32, BitAnd, bitand, "and $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoI32, BitAnd, bitand, "and $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_unary_op!(CryptoU32, Not, not, "not $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_unary_op!(CryptoI32, Not, not, "not $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_unary_op!(CryptoU32, Neg, neg, "neg $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_unary_op!(CryptoI32, Neg, neg, "neg $0")

