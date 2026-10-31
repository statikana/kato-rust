use crate::util::typename;


/// represents rules for how an object must be treated in memory
struct Layout {
    /// size of the object
    bytes: usize,

    /// necessary alignment of the address at which its stored. for small objects, this is usually just `bytes`
    align: usize,
}

/// info about target system
struct Target {
    /// either `ptr_size=4` (32-bit systems) or `ptr_size=8` (64-bit systems)
    ptr_size: usize
}

enum Type {
    Unit,
    Bool,
    Nat { nbytes: usize },
    Int { signed: bool, nbytes: usize },
    Float { nbytes: usize },
    Ptr(Box<Type>),

    Matrix { shape: Vec<usize>, dtype: Box<Type> },
}

/// gets the layout of a `Type`. returns `None` if it has no layout or is too large for a stack (use dynamic)
fn layout_of(t: &Type, target: &Target) -> Option<Layout> {
    let primitive = |n| Some(Layout{ bytes: n, align: n });
    match t {
        Type::Unit => Some(Layout{ bytes: 0, align: 1 }),
        Type::Bool => primitive(1),
        Type::Nat{ nbytes } => primitive(*nbytes),
        Type::Int{ signed: _, nbytes } | Type::Float{ nbytes } => primitive(*nbytes),
        Type::Ptr(_) => primitive(target.ptr_size),

        Type::Matrix { shape, dtype } => {
            let dtype_layout = layout_of(&dtype, target)?;
            let n_elem = shape.iter().try_fold(1 as usize, |item, &current| current.checked_mul(item))?;
            let size = n_elem.checked_mul(dtype_layout.bytes)?;
            Some(Layout{ bytes: size, align: dtype_layout.align })
        },

        #[allow(unreachable_patterns)]
        other => panic!("Unknown memory layout for type '{}'", typename(other))
    }
}