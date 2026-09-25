
pub trait SizeSpec {
    const SIZE: usize;
    type ContainerType;
    
    fn new_container() -> Self::ContainerType;
}


// Generic container
pub struct Container<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Container<N> {
    pub fn new() -> Self {
        Container { data: [0; N] }
    }
}

pub trait Typed {
    type Size: SizeSpec;
}

pub struct Value<T: Typed> where [(); T::Size::SIZE]: {
    container: Container<{T::Size::SIZE}>
}