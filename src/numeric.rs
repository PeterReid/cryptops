
macro_rules! define_numeric_type(
    ($name:ident, $condition_name:ident, $inner_type:ident) => (
        #[deriving(Copy, Show)]
        pub struct $name {
            pub value: $inner_type
        }

        impl $name {
            pub   fn new(value: $inner_type) -> $name {
                $name{value: value}
            }
        }

        #[deriving(Copy, Show)]
        pub struct $condition_name {
            pub mask: $inner_type
        }
    )
)

define_numeric_type!(CryptoU64, CryptoU64Condition, u64)
define_numeric_type!(CryptoI64, CryptoI64Condition, i64)
define_numeric_type!(CryptoU32, CryptoU32Condition, u32)
define_numeric_type!(CryptoI32, CryptoI32Condition, i32)
define_numeric_type!(CryptoU8,  CryptoU8Condition,  u8 )
define_numeric_type!(CryptoI8,  CryptoI8Condition,  i8 )

#[cfg(target_arch = "x86_64")]
impl CryptoU32Condition {
    pub fn equal(a: CryptoU32, b: CryptoU32) -> CryptoU32Condition {
        let ones: u32 = 0xffffffff;
        let mask: u32;
        let zeros: u32 = 0;
        unsafe {
            asm!("cmp $2, $3
                  cmove $1, $0"
                  : "=r"(mask) : "r"(ones), "r"(a.value), "r"(b.value), "0"(zeros) : "cc");
        }
        CryptoU32Condition{mask: mask}
    }

    pub fn select(&self, a: CryptoU32, b: CryptoU32) -> CryptoU32 {
        let ret: u32;
        let zero: u32 = 0;
        unsafe {
            asm!("cmp $3, $4
                  cmove $1, $0"
                  : "=r"(ret) : "r"(b.value), "0"(a.value), "r"(zero), "r"(self.mask) : "cc")
        }
        CryptoU32::new(ret)
    }
}




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
macro_rules! simple_x86_binary_ops(
    ($trait_name:ident, $function_name:ident, $asm:expr) => (
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_binary_op!(CryptoU8, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_binary_op!(CryptoI8, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_binary_op!(CryptoU32, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_binary_op!(CryptoI32, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86_64"))]
        define_binary_op!(CryptoU64, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86_64"))]
        define_binary_op!(CryptoI64, $trait_name, $function_name, $asm)
    )
)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! simple_x86_unary_ops(
    ($trait_name:ident, $function_name:ident, $asm:expr) => (
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_unary_op!(CryptoU8, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_unary_op!(CryptoI8, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_unary_op!(CryptoU32, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        define_unary_op!(CryptoI32, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86_64"))]
        define_unary_op!(CryptoU64, $trait_name, $function_name, $asm)

        #[cfg(any(target_arch = "x86_64"))]
        define_unary_op!(CryptoI64, $trait_name, $function_name, $asm)
    )
)



// addition
simple_x86_binary_ops!(Add, add, "add $1, $0")

// subtraction
simple_x86_binary_ops!(Sub, sub, "sub $1, $0")

// multiplication
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoU32, Mul, mul, "imul $1, $0")

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
define_binary_op!(CryptoI32, Mul, mul, "imul $1, $0")

#[cfg(any(target_arch = "x86_64"))]
define_binary_op!(CryptoU64, Mul, mul, "imul $1, $0")

#[cfg(any(target_arch = "x86_64"))]
define_binary_op!(CryptoI64, Mul, mul, "imul $1, $0")


// xor
simple_x86_binary_ops!(BitXor, bitxor, "xor $1, $0")

// or
simple_x86_binary_ops!(BitOr, bitor, "or $1, $0")

// and
simple_x86_binary_ops!(BitAnd, bitand, "and $1, $0")

simple_x86_unary_ops!(Not, not, "not $0")
simple_x86_unary_ops!(Neg, neg, "neg $0")


