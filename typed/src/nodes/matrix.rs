use ast::nodes::{ASTMatrix, Span};

use crate::typecheck::{Typecheck, TypeError, TypedNode};
use crate::types::{MatrixType, Type};

impl Typecheck for ASTMatrix {
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        // all elements of the same type
        // yes this breaks if the matrix is empty
        let mut dtype = &children.first().unwrap().dtype;
        println!("{}", dtype);
        if !children.iter().all(|elem| &elem.dtype == dtype) {
            return Err(TypeError::NonuniformMatrixType { span });
        }

        // we need to now collapse the matrix as best we can
        // e.g. a matrix[I] of dtype matrix[J] should be represented as matrix[I, J]
        // since the elements should be already well-typed, we dont need
        // recursion here.
        let mut shape = vec![self.elements.len()];

        // we may ignore the dtype of the elements?
        if let Type::Matrix(MatrixType {
            shape: elem_shape,
            dtype: inner_dtype,
        }) = dtype
        {
            shape.extend(elem_shape);
            dtype = inner_dtype; // if we collapse, adopt the inner matrix's type

        }
        Ok(Type::Matrix(MatrixType {
            shape,
            dtype: Box::new(dtype.clone()),
        }))
    }
}
