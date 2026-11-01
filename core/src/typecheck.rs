use crate::ast::{ Node, Span, BinaryOperator, UrnaryOperator };
use crate::datatype::Type;

/// A typed view over a `Node` and its children, produced by `typecheck`
pub struct TypedNode<'a> { pub node: &'a Node, pub dtype: Type, pub children: Vec<TypedNode<'a>> }

#[derive(Debug)]
pub enum TypeError {
    UnsupportedUrnaryOperation { op: UrnaryOperator, subject: Type, span: Span },
    UnsupportedBinaryOperation { op: BinaryOperator, lhs: Type, rhs: Type, span: Span },
}

// types the children first, then lets the node decide its own type from theirs
pub fn typecheck(node: &Node) -> Result<TypedNode<'_>, TypeError> {
    let children = node.data.children()
        .into_iter()
        .map(typecheck)
        .collect::<Result<Vec<_>, _>>()?;
    let dtype = node.data.typecheck(&children, node.span)?;
    Ok(TypedNode{ node, dtype, children })
}
