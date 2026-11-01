use crate::ast::{ NodeData, Node, Span, BinaryOperator, UrnaryOperator };
use crate::datatype::{ Type, IntegerType, FloatType, Numeric };

/// A `Node` with a `dtype: Type` field, used by `typecheck`
pub struct TypedNode { pub data: NodeData, pub span: Span, pub dtype: Type }

#[derive(Debug)]
pub enum TypeError {
    UnsupportedUrnaryOperation { op: UrnaryOperator, subject: Type, span: Span },
    UnsupportedBinaryOperation { op: BinaryOperator, lhs: Type, rhs: Type, span: Span },
}

// turns a Node into a TypedNode
pub fn typecheck(node: Node) -> Result<TypedNode, TypeError> {
    let dtype = type_of(&node)?;
    Ok(TypedNode{ data: node.data, span: node.span, dtype })
}

/// the type `node` evaluates to
fn type_of(node: &Node) -> Result<Type, TypeError> {
    let span = node.span;
    let dtype = match &node.data {
        // statements are checked, but don't produce a value
        NodeData::Scope{ statements } => {
            for statement in statements {
                type_of(statement)?;
            }
            Type::Unit
        },
        NodeData::Out(value) => {
            type_of(value)?;
            Type::Unit
        },
        NodeData::BinaryOperation { op, lhs, rhs } => {
            let (lhs, rhs) = (type_of(lhs)?, type_of(rhs)?);
            lhs.binary_result(*op, &rhs)
                .ok_or(TypeError::UnsupportedBinaryOperation { op: *op, lhs, rhs, span })?
        },
        NodeData::UrnaryOperation { op, subject } => {
            let subject = type_of(subject)?;
            subject.unary_result(*op)
                .ok_or(TypeError::UnsupportedUrnaryOperation { op: *op, subject, span })?
        },
        NodeData::Boolean(_) => Type::Boolean,
        NodeData::Integer(_) => Type::Integer(IntegerType{ signed: true, nbytes: 8 }), // TODO: add actual ints
        NodeData::Float(_) => Type::Float(FloatType{ nbytes: 8 }), // TODO: add actual floats
        NodeData::Unit() => Type::Unit,
    };
    Ok(dtype)
}
