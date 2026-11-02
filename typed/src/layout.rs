/// represents rules for how an object must be treated in memory
pub struct Layout {
    /// size of the object
    pub bytes: usize,

    /// necessary alignment of the address at which its stored. for small objects, this is usually just `bytes`
    pub align: usize,
}

impl Layout {
    /// a layout aligned to its own size
    pub fn primitive(bytes: usize) -> Layout {
        Layout {
            bytes,
            align: bytes,
        }
    }
}

/// info about target system
pub struct Target {
    /// either `ptr_size=4` (32-bit systems) or `ptr_size=8` (64-bit systems)
    pub ptr_size: usize,
}
