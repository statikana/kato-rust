pub mod float;
pub mod integer;
pub mod matrix;
pub mod ptr;
pub mod unit;
pub mod boolean;

use ast::for_each_node_kind;
use cranelift_codegen::ir;
use typed::types::Type;

/// represents rules for how an object must be treated in memory
pub struct Layout {
    /// size of object in bits
    size: usize,

    /// align of memory addr in bits
    align: usize,
    pub clif_type: ir::Type
}

/// info about target system
pub struct Target {
    pub pointer_size: usize,
    pub register_width: usize
}

/// memory rules for a type
pub trait HasLayout {
    /// the layout of the type on `target`. returns `None` if it has no layout or is too large for a stack (use dynamic)
    fn layout(&self, target: &Target) -> Option<Layout>;
}

// similar to the for_each_node_kind but implements layout information for each type (not node)
macro_rules! for_each_type_kind {
    ($callback:ident) => {
        $callback! {
            Boolean(BooleanType),
            Float(FloatType),
            Integer(IntegerType),
            Matrix(MatrixType),
            Ptr(PtrType),
            Unit(UnitType),
        }
    };
}

macro_rules! impl_layout_for_types {
    ($($type_variant:ident($type_struct:ty)),* $(,)?) => {
        impl HasLayout for Type {
            fn layout(&self, target: &Target) -> Option<Layout> {
                match self {
                    $(Type::$type_variant(t) => t.layout(target)),*
                }
            }
        }
    };
}

for_each_type_kind!(impl_layout_for_types);
