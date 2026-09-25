use crate::datatype::{ SizeSpec, Container, Typed };

// macro for making size specs
macro_rules! define_size_spec {
    ($name:ident, $size:expr) => {
        pub struct $name;
        impl SizeSpec for $name {
            const SIZE: usize = $size;
            type ContainerType = Container<{Self::SIZE}>;
            
            fn new_container() -> Container<{Self::SIZE}> {
                Self::ContainerType::new()
            }
        }
    };
}

macro_rules! define_type {
    ($name:ident, $size:ident) => {
        pub struct $name;
        impl Typed for $name {
            type Size = $size;
        }
    };
}


// Size markers
define_size_spec!(Size1, 1);
define_size_spec!(Size8, 8);
define_size_spec!(Size16, 16);
define_size_spec!(Size32, 32);
define_size_spec!(Size64, 64);
define_size_spec!(Size128, 128);
define_size_spec!(Size256, 256);

define_type!(Bool, Size1);

define_type!(Int32, Size32);
define_type!(Float32, Size32);

define_type!(Int64, Size64);
define_type!(Float64, Size64);
