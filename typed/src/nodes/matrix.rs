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
        
        let mut shape = vec![self.elements.len()];
        
        /*
            we need to now collapse the matrix as best we can
            because `typecheck` is reverse recursive (the children are already typechecked),
            we may assume any further inner matrices have already been collapsed into this child.
    
            eg:
            [
                [ 1, 2, 3],
                [ 40, 50, 60],
            ]
            is interpreted as a matrix of shape=[2,], with dtype=(matrix of shape=[3,] and dtype=i64)
            after collapse: matrix of shape=[2,3] and dtype=i64
        */
        if let Type::Matrix(MatrixType {
            shape: elem_shape,
            dtype: inner_dtype,
        }) = dtype {
            shape.extend(elem_shape);
            dtype = inner_dtype;

        }
        Ok(Type::Matrix(MatrixType {
            shape,
            dtype: Box::new(dtype.clone()),
        }))
    }
}
