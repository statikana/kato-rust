use std::fmt;


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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Unit,
    Boolean,
    Integer(IntegerType),
    Float(FloatType),
    Ptr(PtrType),
    Matrix(MatrixType),
}

/// arithmetic rules for a type, one method per operator. `None` (the default) means the operation isn't supported
pub trait TypeAlgebra {
    /// the type of `-self`
    fn negate(&self) -> Option<Type> { None }
    /// the type of `self + rhs`
    fn add(&self, _rhs: &Type) -> Option<Type> { None }
    /// the type of `self - rhs`
    fn sub(&self, _rhs: &Type) -> Option<Type> { None }
    /// the type of `self * rhs`
    fn mul(&self, _rhs: &Type) -> Option<Type> { None }
    /// the type of `self / rhs`
    fn div(&self, _rhs: &Type) -> Option<Type> { None }
}

/// memory rules for a type
pub trait HasLayout {
    /// the layout of the type on `target`. returns `None` if it has no layout or is too large for a stack (use dynamic)
    fn layout(&self, target: &Target) -> Option<Layout>;
}

impl Type {
    /// the `Numeric` impl of the type inside, if it has one
    fn numeric(&self) -> Option<&dyn TypeAlgebra> {
        match self {
            Type::Unit | Type::Boolean => None,
            Type::Integer(t) => Some(t),
            Type::Float(t) => Some(t),
            Type::Ptr(t) => Some(t),
            Type::Matrix(t) => Some(t),
        }
    }

    /// applies the binary operation `op` to `self` and `rhs`
    fn binary(&self, rhs: &Type, op: fn(&dyn TypeAlgebra, &Type) -> Option<Type>) -> Option<Type> {
        match (self, rhs) {
            // (Type::Matrix(_), _) => op(self.numeric()?, rhs),
            // // scalar op matrix: broadcast the scalar over the matrix's elements
            // (_, Type::Matrix(m)) => Some(m.with_dtype(self.binary(&m.dtype, op)?)),
            _ => op(self.numeric()?, rhs),
        }
    }
}

impl TypeAlgebra for Type {
    fn negate(&self) -> Option<Type> { self.numeric()?.negate() }
    fn add(&self, rhs: &Type) -> Option<Type> { self.binary(rhs, |t, rhs| t.add(rhs)) }
    fn sub(&self, rhs: &Type) -> Option<Type> { self.binary(rhs, |t, rhs| t.sub(rhs)) }
    fn mul(&self, rhs: &Type) -> Option<Type> { self.binary(rhs, |t, rhs| t.mul(rhs)) }
    fn div(&self, rhs: &Type) -> Option<Type> { self.binary(rhs, |t, rhs| t.div(rhs)) }
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
            // Type::Matrix(t) => t.layout(target),
            _other => panic!("Unknown memory layout")
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
