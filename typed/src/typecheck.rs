use ast::nodes::{ASTNode, Node, NodeKind, Span};
use ast::operator::{BinaryOperator, UnaryOperator};

use crate::operator::{BinaryTyping, UnaryTyping};
use crate::types::{FloatType, IntegerType, MatrixType, Type};

/// A typed `Node`, produced by `typecheck`
pub type TypedNode = Node<Type>;

#[derive(Debug)]
pub enum TypeError {
    UnsupportedUnaryOperation {
        op: UnaryOperator,
        subject: Type,
        span: Span,
    },
    UnsupportedBinaryOperation {
        op: BinaryOperator,
        lhs: Type,
        rhs: Type,
        span: Span,
    },
    NonuniformMatrixType {
        span: Span,
    },
}

// types the children first, then lets the node decide its own type from theirs
pub fn typecheck(node: &ASTNode) -> Result<TypedNode, TypeError> {
    let span = node.span;
    let (kind, dtype) = match &node.kind {
        // statements are checked, but don't produce a value
        NodeKind::Scope { statements } => {
            let statements = statements
                .iter()
                .map(typecheck)
                .collect::<Result<Vec<_>, _>>()?;
            (NodeKind::Scope { statements }, Type::Unit)
        }
        // the operator decides the result from the operand types
        NodeKind::BinaryOperation { op, lhs, rhs } => {
            let lhs = Box::new(typecheck(lhs)?);
            let rhs = Box::new(typecheck(rhs)?);
            // TODO: implementation of reverse operations is possible here
            let dtype = op.result_type(&lhs.dtype, &rhs.dtype).ok_or(
                TypeError::UnsupportedBinaryOperation {
                    op: *op,
                    lhs: lhs.dtype.clone(),
                    rhs: rhs.dtype.clone(),
                    span,
                },
            )?;
            (NodeKind::BinaryOperation { op: *op, lhs, rhs }, dtype)
        }
        // the operator decides the result from the subject's type
        NodeKind::UnaryOperation { op, subject } => {
            let subject = Box::new(typecheck(subject)?);
            let dtype =
                op.result_type(&subject.dtype)
                    .ok_or(TypeError::UnsupportedUnaryOperation {
                        op: *op,
                        subject: subject.dtype.clone(),
                        span,
                    })?;
            (NodeKind::UnaryOperation { op: *op, subject }, dtype)
        }
        NodeKind::Boolean(val) => (NodeKind::Boolean(*val), Type::Boolean),
        NodeKind::Integer(val) => (
            NodeKind::Integer(*val),
            Type::Integer(IntegerType {
                signed: true,
                nbytes: 8,
            }), // TODO: add actual ints
        ),
        NodeKind::Float(val) => (
            NodeKind::Float(*val),
            Type::Float(FloatType { nbytes: 8 }), // TODO: add actual floats
        ),
        NodeKind::Unit => (NodeKind::Unit, Type::Unit),
        NodeKind::Matrix { elements } => {
            let elements = elements
                .iter()
                .map(typecheck)
                .collect::<Result<Vec<_>, _>>()?;
            let dtype = typecheck_matrix(&elements, span)?;
            (NodeKind::Matrix { elements }, dtype)
        }
    };

    Ok(TypedNode { kind, span, dtype })
}

fn typecheck_matrix(children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
    // all elements of the same type
    // yes this breaks if the matrix is empty
    let mut dtype = &children.first().unwrap().dtype;
    println!("{}", dtype);
    if !children.iter().all(|elem| &elem.dtype == dtype) {
        return Err(TypeError::NonuniformMatrixType { span });
    }

    let mut shape = vec![children.len()];

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
    }) = dtype
    {
        shape.extend(elem_shape);
        dtype = inner_dtype;
    }
    Ok(Type::Matrix(MatrixType {
        shape,
        dtype: Box::new(dtype.clone()),
    }))
}
