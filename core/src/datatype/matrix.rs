use crate::ast::{ BinaryOperator, UrnaryOperator };
use super::{ Type, Numeric, HasLayout, Layout, Target };

#[derive(Clone, Debug)]
pub struct MatrixType { pub shape: Vec<usize>, pub dtype: Box<Type> }

impl MatrixType {
    pub(super) fn with_dtype(&self, dtype: Type) -> Type {
        Type::Matrix(MatrixType { shape: self.shape.clone(), dtype: Box::new(dtype) })
    }
}

impl Numeric for MatrixType {
    // elementwise: valid iff the element type supports it
    fn unary_result(&self, op: UrnaryOperator) -> Option<Type> {
        Some(self.with_dtype(self.dtype.unary_result(op)?))
    }

    // elementwise with a matrix of the same shape, or broadcast against a scalar
    fn binary_result(&self, op: BinaryOperator, rhs: &Type) -> Option<Type> {
        match rhs {
            Type::Matrix(rhs) if rhs.shape == self.shape => Some(self.with_dtype(self.dtype.binary_result(op, &rhs.dtype)?)),
            Type::Matrix(_) => None,
            scalar => Some(self.with_dtype(self.dtype.binary_result(op, scalar)?)),
        }
    }
}

impl HasLayout for MatrixType {
    // packed elements, aligned like a single element
    fn layout(&self, target: &Target) -> Option<Layout> {
        let dtype_layout = self.dtype.layout(target)?;
        let n_elem = self.shape.iter().try_fold(1 as usize, |item, &current| current.checked_mul(item))?;
        let size = n_elem.checked_mul(dtype_layout.bytes)?;
        Some(Layout{ bytes: size, align: dtype_layout.align })
    }
}
