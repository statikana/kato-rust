// datatype.rs

// Mostly stuff for static containers (known size at compile time: ints, floats, etc.)

// Fixed-width conversion. The const is a parameter rather than an associated const so an
// impl's signature can match the trait's exactly; the compiler will not relate an
// associated const to the impl's own const parameter, in either spelling.
pub trait LeArray<const N: usize>: Sized {
    fn to_le_array(&self) -> [u8; N];
    fn from_le_array(data: [u8; N]) -> Self;
}

// Streaming conversion, for anything variable-length. Encoding is always little-endian, so
// the same bytes mean the same value on every target.
pub trait ToBytes: Sized {
    const SIZE: usize;

    fn write_le(&self, out: &mut Vec<u8>);
    fn read_le(input: &mut &[u8]) -> Option<Self>;
}

pub trait SizeSpec {
    const SIZE: usize;
    // containers are byte-granular, so SIZE is in bits but storage rounds up
    const BYTES: usize = (Self::SIZE + 7) / 8;
    type ContainerType;

    fn new_container() -> Self::ContainerType;
}

// Generic container, BYTES is a byte count (see SizeSpec::BYTES)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Container<const BYTES: usize> {
    pub data: [u8; BYTES],
}

impl<const BYTES: usize> Container<BYTES> {
    pub fn new() -> Self {
        Container { data: [0; BYTES] }
    }
}

impl<const BYTES: usize> Default for Container<BYTES> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BYTES: usize> ToBytes for Container<BYTES> {
    const SIZE: usize = BYTES;
    fn write_le(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.data);
    }
    fn read_le(input: &mut &[u8]) -> Option<Self> {
        let (head, tail) = input.split_at_checked(BYTES)?;
        *input = tail;
        Some(Container { data: head.try_into().ok()? })
    }
}

// A type tag says how wide it is and which Rust scalar stands in for it.
// The tag owns the container <-> scalar conversion, so Value<T> itself stays generic.
pub trait Typed: Sized {
    type Size: SizeSpec;
    type Repr: ToBytes;

    fn from_container(container: Container<{<Self::Size as SizeSpec>::BYTES}>) -> Self::Repr;
    fn into_container(repr: Self::Repr) -> Container<{<Self::Size as SizeSpec>::BYTES}>;
}

pub struct Value<T: Typed> where [(); <T::Size as SizeSpec>::BYTES]: {
    pub container: Container<{T::Size::BYTES}>,
}

// Byte methods are inherent rather than a ToBytes impl: the compiler cannot check a
// generic-const array signature against Self::SIZE in a blanket impl, so these would
// never verify against the trait.
impl<T: Typed> Value<T> where [(); <T::Size as SizeSpec>::BYTES]: {
    pub fn new(repr: T::Repr) -> Self {
        Value { container: T::into_container(repr) }
    }

    pub fn as_repr(&self) -> T::Repr {
        T::from_container(self.container)
    }

    pub fn write_le(&self, out: &mut Vec<u8>) {
        self.container.write_le(out);
    }

    pub fn read_le(input: &mut &[u8]) -> Option<Self> {
        let container = Container::<{T::Size::BYTES}>::read_le(input)?;
        Some(Value { container })
    }
}

impl<T: Typed> PartialEq for Value<T> where [(); <T::Size as SizeSpec>::BYTES]: {
    fn eq(&self, other: &Self) -> bool {
        self.container == other.container
    }
}

impl<T: Typed> std::fmt::Debug for Value<T> where [(); <T::Size as SizeSpec>::BYTES]: {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value({:?})", self.container.data)
    }
}

pub const fn sym_max(bits: usize) -> i128 {
    (1i128 << (bits / 2)) - 1
}

pub const fn sym_min(bits: usize) -> i128 {
    -(1i128 << (bits / 2))
}

pub trait Bounded {
    type Value;

    const MIN: Self::Value;
    const MAX: Self::Value;

    fn in_bounds(value: Self::Value) -> bool
    where
        Self::Value: PartialOrd,
    {
        value >= Self::MIN && value <= Self::MAX
    }
}
