use std::fmt;

use crate::ast::{ BinaryOperator, UrnaryOperator };
use crate::util::typename;

mod integer;
mod float;
mod ptr;
mod matrix;

pub use integer::IntegerType;
pub use float::FloatType;
pub use ptr::PtrType;
pub use matrix::MatrixType;


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
        Layout{ bytes, align: bytes }
    }
}

/// info about target system
pub struct Target {
    /// either `ptr_size=4` (32-bit systems) or `ptr_size=8` (64-bit systems)
    pub ptr_size: usize
}

#[derive(Clone, Debug)]
pub enum Type {
    Unit,
    Boolean,
    Integer(IntegerType),
    Float(FloatType),
    Ptr(PtrType),
    Matrix(MatrixType),
}

/// arithmetic rules for a type. `None` means the operation isn't supported
pub trait Numeric {
    /// the type of `op self`
    fn unary_result(&self, op: UrnaryOperator) -> Option<Type>;
    /// the type of `self op rhs`
    fn binary_result(&self, op: BinaryOperator, rhs: &Type) -> Option<Type>;
}

/// memory rules for a type
pub trait HasLayout {
    /// the layout of the type on `target`. returns `None` if it has no layout or is too large for a stack (use dynamic)
    fn layout(&self, target: &Target) -> Option<Layout>;
}

impl Numeric for Type {
    fn unary_result(&self, op: UrnaryOperator) -> Option<Type> {
        match self {
            Type::Unit | Type::Boolean => None,
            Type::Integer(t) => t.unary_result(op),
            Type::Float(t) => t.unary_result(op),
            Type::Ptr(t) => t.unary_result(op),
            Type::Matrix(t) => t.unary_result(op),
        }
    }

    fn binary_result(&self, op: BinaryOperator, rhs: &Type) -> Option<Type> {
        match (self, rhs) {
            (Type::Matrix(t), _) => t.binary_result(op, rhs),
            // scalar op matrix: broadcast the scalar over the matrix's elements
            (_, Type::Matrix(m)) => Some(m.with_dtype(self.binary_result(op, &m.dtype)?)),

            (Type::Unit | Type::Boolean, _) => None,
            (Type::Integer(t), _) => t.binary_result(op, rhs),
            (Type::Float(t), _) => t.binary_result(op, rhs),
            (Type::Ptr(t), _) => t.binary_result(op, rhs),
        }
    }
}

impl HasLayout for Type {
    fn layout(&self, target: &Target) -> Option<Layout> {
        #[allow(unreachable_patterns)]
        match self {
            Type::Unit => Some(Layout{ bytes: 0, align: 1 }),
            Type::Boolean => Some(Layout::primitive(1)),
            Type::Integer(t) => t.layout(target),
            Type::Float(t) => t.layout(target),
            Type::Ptr(t) => t.layout(target),
            Type::Matrix(t) => t.layout(target),
            other => panic!("Unknown memory layout for type '{}'", typename(other))
        }
    }
}

// compact names, e.g. `i64`, `*f32`, `[2, 3]f64`
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unit => f.write_str("unit"),
            Type::Boolean => f.write_str("bool"),
            Type::Integer(t) => t.fmt(f),
            Type::Float(t) => t.fmt(f),
            Type::Ptr(t) => t.fmt(f),
            Type::Matrix(t) => t.fmt(f),
        }
    }
}
