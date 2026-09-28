use crate::datatype::{ Bounded, Container, LeArray, SizeSpec, ToBytes, Typed, sym_max, sym_min };

// macro for making size specs
macro_rules! define_size_spec {
    ($name:ident, $bits:expr, $bytes:expr) => {
        pub struct $name;
        impl SizeSpec for $name {
            const SIZE: usize = $bits;
            const BYTES: usize = $bytes;
            type ContainerType = Container<$bytes>;

            fn new_container() -> Self::ContainerType {
                Self::ContainerType::new()
            }
        }
    };
}

macro_rules! define_type {
    // Type tag backed by a Rust scalar, no range
    ($name:ident, $size:ident, $scalar:ty) => {
        pub struct $name;
        impl Typed for $name {
            type Size = $size;
            type Repr = $scalar;
            // these two are where the size spec and the scalar have to agree: a scalar only
            // has a LeArray impl at its own width, so a mismatched pair like (Size64, i32)
            // fails to compile right here
            fn from_container(container: Container<{$size::BYTES}>) -> $scalar {
                <$scalar as LeArray<{$size::BYTES}>>::from_le_array(container.data)
            }
            fn into_container(repr: $scalar) -> Container<{$size::BYTES}> {
                Container { data: <$scalar as LeArray<{$size::BYTES}>>::to_le_array(&repr) }
            }
        }
    };
    // Same, and it also carries MIN/MAX
    ($name:ident, $size:ident, $scalar:ty, bounded) => {
        define_type!($name, $size, $scalar);
        impl Bounded for $name {
            type Value = $scalar;
            const MIN: $scalar = sym_min(<$size as SizeSpec>::SIZE) as $scalar;
            const MAX: $scalar = sym_max(<$size as SizeSpec>::SIZE) as $scalar;
        }
    };
}

macro_rules! impl_int_bytes {
    ($($t:ty => $bytes:expr),* $(,)?) => {$(
        impl LeArray<$bytes> for $t {
            fn to_le_array(&self) -> [u8; $bytes] { self.to_le_bytes() }
            fn from_le_array(data: [u8; $bytes]) -> Self { <$t>::from_le_bytes(data) }
        }
        impl ToBytes for $t {
            const SIZE: usize = $bytes;
            fn write_le(&self, out: &mut Vec<u8>) { out.extend_from_slice(&self.to_le_bytes()) }
            fn read_le(input: &mut &[u8]) -> Option<Self> {
                let (head, tail) = input.split_at_checked(Self::SIZE)?;
                *input = tail;
                Some(<$t>::from_le_bytes(head.try_into().ok()?))
            }
        }
    )*};
}

macro_rules! impl_float_bytes {
    ($($t:ty => $bytes:expr => $int:ty),* $(,)?) => {$(
        impl LeArray<$bytes> for $t {
            fn to_le_array(&self) -> [u8; $bytes] { self.to_bits().to_le_bytes() }
            fn from_le_array(data: [u8; $bytes]) -> Self {
                <$t>::from_bits(<$int>::from_le_bytes(data))
            }
        }
        impl ToBytes for $t {
            const SIZE: usize = $bytes;
            fn write_le(&self, out: &mut Vec<u8>) {
                out.extend_from_slice(&self.to_bits().to_le_bytes())
            }
            fn read_le(input: &mut &[u8]) -> Option<Self> {
                let (head, tail) = input.split_at_checked(Self::SIZE)?;
                *input = tail;
                Some(<$t>::from_bits(<$int>::from_le_bytes(head.try_into().ok()?)))
            }
        }
    )*};
}

impl LeArray<1> for bool {
    fn to_le_array(&self) -> [u8; 1] { [u8::from(*self)] }
    fn from_le_array(data: [u8; 1]) -> Self { data[0] != 0 }
}

impl ToBytes for bool {
    const SIZE: usize = 1;
    fn write_le(&self, out: &mut Vec<u8>) { out.push(u8::from(*self)) }
    fn read_le(input: &mut &[u8]) -> Option<Self> {
        let (head, tail) = input.split_at_checked(Self::SIZE)?;
        *input = tail;
        Some(head[0] != 0)
    }
}

impl_int_bytes!(u8 => 1, i8 => 1, u16 => 2, i16 => 2, u32 => 4, i32 => 4, u64 => 8, i64 => 8, u128 => 16, i128 => 16);
impl_float_bytes!(f32 => 4 => u32, f64 => 8 => u64);

// Size markers: bits, then bytes
define_size_spec!(Size1, 1, 1);
define_size_spec!(Size8, 8, 1);
define_size_spec!(Size16, 16, 2);
define_size_spec!(Size32, 32, 4);
define_size_spec!(Size64, 64, 8);
define_size_spec!(Size128, 128, 16);
define_size_spec!(Size256, 256, 32);


// Boolean
define_type!(Bool, Size1, bool);


// Integers

define_type!(Int16, Size16, i16, bounded);
define_type!(Int32, Size32, i32, bounded);
define_type!(Int64, Size64, i64, bounded);


// Floating point
// no Float16 yet: there is no f16 to use as a Repr

define_type!(Float32, Size32, f32);
define_type!(Float64, Size64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatype::Value;

    #[test]
    fn size_specs_round_bits_up_to_whole_bytes() {
        assert_eq!(Size1::BYTES, 1);
        assert_eq!(Size16::BYTES, 2);
        assert_eq!(Size32::BYTES, 4);
        assert_eq!(Size64::BYTES, 8);
        assert_eq!(Size256::BYTES, 32);
    }

    #[test]
    fn container_holds_one_array_per_byte() {
        let value = Value::<Int32>::new(0x0102_0304);
        assert_eq!(value.container.data, [0x04, 0x03, 0x02, 0x01]);
        assert_eq!(value.as_repr(), 0x0102_0304);
    }

    #[test]
    fn value_round_trips_through_a_cursor() {
        let mut out = Vec::new();
        Value::<Int64>::new(-2).write_le(&mut out);
        Value::<Float64>::new(0.5).write_le(&mut out);
        out.extend_from_slice(&[0xAA, 0xBB]); // trailing data the caller wants back

        let mut cursor = &out[..];
        assert_eq!(Value::<Int64>::read_le(&mut cursor).unwrap().as_repr(), -2);
        assert_eq!(Value::<Float64>::read_le(&mut cursor).unwrap().as_repr(), 0.5);
        assert_eq!(cursor, &[0xAA, 0xBB]);
    }

    #[test]
    fn truncated_input_reads_as_none() {
        let bytes = [0u8; 3];
        let mut cursor = &bytes[..];
        assert_eq!(Value::<Int32>::read_le(&mut cursor), None);
    }

    #[test]
    fn bool_stays_zero_or_one() {
        assert_eq!(Value::<Bool>::new(true).as_repr(), true);
        assert_eq!(Value::<Bool>::new(false).container.data, [0]);
        assert_eq!(bool::from_le_array([7]), true);
    }

    #[test]
    fn floats_survive_nan() {
        let mut out = Vec::new();
        Value::<Float64>::new(f64::NAN).write_le(&mut out);
        let mut cursor = &out[..];
        assert!(Value::<Float64>::read_le(&mut cursor).unwrap().as_repr().is_nan());
    }

    #[test]
    fn values_compare_by_their_bytes() {
        assert_eq!(Value::<Int32>::new(7), Value::<Int32>::new(7));
        assert_ne!(Value::<Int32>::new(7), Value::<Int32>::new(8));
    }
}
