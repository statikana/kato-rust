use std::fmt;

use super::{HasLayout, Layout, Target, Type, TypeAlgebra};

// type algebra
// layout
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

impl HasLayout for MatrixType {
    fn layout(&self, target: &Target) -> Option<Layout> {
        // the bytes of the matrix is just the total size of its elements (n_elements * element_size)
        // the align is the align if its elements
        let element_layout = self.dtype.layout(target)?;

        let n_elements = self.shape.iter().fold(1, |cur, dim_size| cur * dim_size);
        let element_size = element_layout.bytes;
        let bytes = n_elements * element_size;

        let align = element_layout.align;
        Some(Layout { bytes, align })
    }
}

impl fmt::Display for MatrixType {
    // shape, then element type, e.g. `[2, 3]f64`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.shape, self.dtype)
    }
}
