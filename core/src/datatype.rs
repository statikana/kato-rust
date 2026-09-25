
trait SizeSpec {
    const SIZE: usize;
    type ContainerType;
    
    fn new_container() -> Self::ContainerType;
}

// Size markers
struct Size32;
impl SizeSpec for Size32 {
    const SIZE: usize = 32;
    type ContainerType = Container<{Self::SIZE}>;
    
    fn new_container() -> Container<{Self::SIZE}> {
        Self::ContainerType::new()
    }
}

struct Size64;
impl SizeSpec for Size64 {
    const SIZE: usize = 64;
    type ContainerType = Container<{Self::SIZE}>;
    
    fn new_container() -> Container<{Self::SIZE}> {
        Self::ContainerType::new()
    }
}

// Generic container
struct Container<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Container<N> {
    fn new() -> Self {
        Container { data: [0; N] }
    }
}

trait Typed {
    type Size: SizeSpec;
}

struct Int64;
impl Typed for Int64 {
    type Size = Size64;
}

struct Int32;
impl Typed for Int32 {
    type Size = Size32;
}

struct Value<T: Typed> where [(); T::Size::SIZE]: {
    container: Container<{T::Size::SIZE}>
}

fn main() {
    let _c32 = Size32::new_container();  // Container<32>
    let _c64 = Size64::new_container();  // Container<64>

    println!("{}", Size32::SIZE);       // 32
    println!("{}", Size64::SIZE);       // 64
}