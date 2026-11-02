use crate::ast::{ ASTNode, Span };
use crate::operator::{ BinaryOperator, UnaryOperator };
use crate::types::Type;

/// A typed view over a `Node` and its children, produced by `typecheck`
pub struct TypedNode<'a> { pub node: &'a ASTNode, pub dtype: Type, pub children: Vec<TypedNode<'a>> }

#[derive(Debug)]
pub enum TypeError {
    UnsupportedUnaryOperation { op: &'static dyn UnaryOperator, subject: Type, span: Span },
    UnsupportedBinaryOperation { op: &'static dyn BinaryOperator, lhs: Type, rhs: Type, span: Span },
    NonuniformMatrixType { span: Span }
}

// types the children first, then lets the node decide its own type from theirs
pub fn typecheck(node: &ASTNode) -> Result<TypedNode<'_>, TypeError> {
    let children = node.data.children()
        .into_iter()
        .map(typecheck)
        .collect::<Result<Vec<_>, _>>()?;
    let dtype = node.data.typecheck(&children, node.span)?;
    Ok(TypedNode{ node, dtype, children })
}
