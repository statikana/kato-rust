use std::fmt;

use super::{Type, TypeAlgebra};

// type algebra
// display

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixType {
    pub shape: Vec<usize>,
    pub dtype: Box<Type>,
}

impl MatrixType {
    /// element-wise operations are performed on every element
    /// so the dtype of this array is the result of `self.dtype` and `rhs`
    /// under `op`
    fn elementwise(&self, op: fn(&Type, &Type) -> Option<Type>, rhs: &Type) -> Option<Type> {
        Some(Type::Matrix(MatrixType {
            shape: self.shape.clone(),
            dtype: Box::new(op(&*self.dtype, rhs).unwrap()),
        }))
    }
}

impl TypeAlgebra for MatrixType {
    // element-wise ops
    fn add(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::add, rhs)
    }
    fn sub(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::sub, rhs)
    }
    fn mul(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::mul, rhs)
    }
    fn div(&self, rhs: &Type) -> Option<Type> {
        self.elementwise(TypeAlgebra::div, rhs)
    }
}

impl fmt::Display for MatrixType {
    // shape, then element type, e.g. `[2, 3]f64`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.shape, self.dtype)
    }
}
