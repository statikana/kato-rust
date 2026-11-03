use std::fmt;

pub mod float;
pub mod integer;
pub mod matrix;
pub mod ptr;
pub mod unit;
pub mod boolean;

pub use float::FloatType;
pub use integer::IntegerType;
pub use matrix::MatrixType;
pub use ptr::PtrType;
pub use boolean::BooleanType;
pub use unit::UnitType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Unit(UnitType),
    Boolean(BooleanType),
    Integer(IntegerType),
    Float(FloatType),
    Ptr(PtrType),
    Matrix(MatrixType),
}

/// arithmetic rules for a type, one method per operator. `None` (the default) means the operation isn't supported
pub trait TypeAlgebra {
    /// the type of `-self`
    fn negate(&self) -> Option<Type> {
        None
    }
    /// the type of `self + rhs`
    fn add(&self, _rhs: &Type) -> Option<Type> {
        None
    }
    /// the type of `self - rhs`
    fn sub(&self, _rhs: &Type) -> Option<Type> {
        None
    }
    /// the type of `self * rhs`
    fn mul(&self, _rhs: &Type) -> Option<Type> {
        None
    }
    /// the type of `self / rhs`
    fn div(&self, _rhs: &Type) -> Option<Type> {
        None
    }
}

impl Type {
    /// the `Numeric` impl of the type inside, if it has one
    fn numeric(&self) -> Option<&dyn TypeAlgebra> {
        match self {
            Type::Unit(_) => None,
            Type::Boolean(_) => None,
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
    fn negate(&self) -> Option<Type> {
        self.numeric()?.negate()
    }
    fn add(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.add(rhs))
    }
    fn sub(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.sub(rhs))
    }
    fn mul(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.mul(rhs))
    }
    fn div(&self, rhs: &Type) -> Option<Type> {
        self.binary(rhs, |t, rhs| t.div(rhs))
    }
}

// compact names, e.g. `i64`, `*f32`, `[2, 3]f64`
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unit(t) => f.write_str("unit"),
            Type::Boolean(t) => f.write_str("bool"),
            Type::Integer(t) => t.fmt(f),
            Type::Float(t) => t.fmt(f),
            Type::Ptr(t) => t.fmt(f),
            Type::Matrix(t) => t.fmt(f),
        }
    }
}
