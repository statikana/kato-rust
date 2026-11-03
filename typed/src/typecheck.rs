use ast::for_each_node_kind;
use ast::nodes::{ASTNode, NodeKind, Span};
use ast::operator::{BinaryOperator, UnaryOperator};

use crate::types::Type;

/// A typed view over a `Node` and its children, produced by `typecheck`
///
pub struct TypedNode<'a> {
    pub node: &'a ASTNode,
    pub dtype: Type,
    pub children: Vec<TypedNode<'a>>,
}

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

/// typing rules for a single kind of node
pub trait Typecheck {
    /// the type this node evaluates to. `children` are the typed `children()`, in the same order
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError>;
}

macro_rules! impl_typecheck_for_node_kind {
    ($($variant:ident($node:ty)),* $(,)?) => {
        impl Typecheck for NodeKind {
            fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
                match self {
                    $(NodeKind::$variant(n) => n.typecheck(children, span)),*
                }
            }
        }
    };
}

for_each_node_kind!(impl_typecheck_for_node_kind);

// types the children first, then lets the node decide its own type from theirs
pub fn typecheck(node: &ASTNode) -> Result<TypedNode<'_>, TypeError> {
    let children = node
        .kind
        .behavior()
        .children()
        .into_iter()
        .map(typecheck)
        .collect::<Result<Vec<_>, _>>()?;
    let dtype = node.kind.typecheck(&children, node.span)?;
    Ok(TypedNode {
        node,
        dtype,
        children,
    })
}
