use std::fmt;

use crate::datatype::{ FloatType, MatrixType, Type };
use crate::typecheck::{ TypedNode, TypeError };
use super::{ ASTNode, NodeBehavior, Span };


pub struct ASTMatrix{ pub elements: Vec<ASTNode> }


impl NodeBehavior for ASTMatrix {
    fn name(&self) -> &'static str { "Matrix" }

    // since iterators create reference, iter().collect() can be
    // used to create references of each child
    fn children(&self) -> Vec<&ASTNode> { self.elements.iter().collect() }

    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        // all elements of the same type
        // yes this breaks if the matrix is empty
        let elem_dtype = &children.first().unwrap().dtype;
        if !children.iter().all(|elem| &elem.dtype == elem_dtype) {
            return Err(TypeError::NonuniformMatrixType { span });
        }
        
        // we need to now collapse the matrix as best we can
        // e.g. a matrix[I] of dtype matrix[J] should be represented as matrix[I, J]
        // since the elements should be already well-typed, we dont need
        // recursion here.
        let mut shape = vec![self.elements.len()];

        // we may ignore the dtype of the elements?
        if let Type::Matrix(MatrixType{ shape: elem_shape, dtype: _ })= elem_dtype {
            shape.extend(elem_shape);
        }
        Ok(Type::Matrix(MatrixType { shape, dtype: Box::new(elem_dtype.clone()) }))
    }
}