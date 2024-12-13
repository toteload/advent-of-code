pub trait Integer<T>:
    Copy + From<u8> + PartialEq + PartialOrd + 
Add<Output=T> +
Sub<Output=T> +
Div<Output=T> +
Mul<Output=T> +
Rem<Output=T> +
Shr<Output=T> +
Rem<Output=T> +
BitAnd<Output=T> +
{
    const ZERO: T;
    const ONE: T;
}

trait Unsigned<T>: Integer<T> {}
trait Signed<T>: Integer<T> + Neg<Output = T> {}

macro_rules! impl_integer {
    ($($t:ty)*) => ($(
        impl Integer<$t> for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
        }
    )*)
}

macro_rules! impl_empty_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {}
    )*)
}

impl_integer!(i8 i16 i32 i64 i128 u8 u16 u32 u64 usize);
impl_empty_trait!(Unsigned for u8 u16 u32 u64 usize);
impl_empty_trait!(Signed for i8 i16 i32 i64);

