pub mod float;
pub mod integer;
pub mod matrix;
pub mod ptr;

use cranelift_codegen::ir;
use cranelift_codegen::ir::types::I8;
use typed::types::Type;

/// represents rules for how an object must be treated in memory
pub struct Layout {
    /// size of object in bits
    size: usize,

    /// align of memory addr in bits
    align: usize,
    pub clif_type: ir::Type,
}

/// info about target system
pub struct Target {
    pub pointer_size: usize,
    pub register_width: usize,
}

/// memory rules for a type
pub trait HasLayout {
    /// the layout of the type on `target`. returns `None` if it has no layout or is too large for a stack (use dynamic)
    fn layout(&self, target: &Target) -> Option<Layout>;
}

impl HasLayout for Type {
    fn layout(&self, target: &Target) -> Option<Layout> {
        match self {
            Type::Unit => None,
            Type::Boolean => Some(Layout {
                size: 1,
                align: 1,
                clif_type: I8,
            }),
            Type::Integer(t) => t.layout(target),
            Type::Float(t) => t.layout(target),
            Type::Ptr(t) => t.layout(target),
            Type::Matrix(t) => t.layout(target),
        }
    }
}
