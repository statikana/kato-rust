use std::fmt;

use crate::operator::{BinaryOperator, UnaryOperator};

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

// Node<() > = Untyped node (ASTNode)
// Node<Type> = Typed node (TypedNode), made by the typechecker (see the `typed` crate)
pub struct Node<T = ()> {
    pub kind: NodeKind<T>,
    pub span: Span,
    pub dtype: T,
}

pub type ASTNode = Node<()>;

impl ASTNode {
    pub fn new(kind: NodeKind<()>, span: Span) -> ASTNode {
        Node {
            kind,
            span,
            dtype: (),
        }
    }
}

/// every kind of node. a closed set, so later passes (e.g. the typechecker) can `match` on it exhaustively
pub enum NodeKind<T = ()> {
    Scope {
        statements: Vec<Node<T>>,
    },
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<Node<T>>,
        rhs: Box<Node<T>>,
    },
    UnaryOperation {
        op: UnaryOperator,
        subject: Box<Node<T>>,
    },
    Boolean(bool),
    Integer(i128),
    Float(f64),
    Unit,
    Matrix {
        elements: Vec<Node<T>>,
    },
}

impl<T> NodeKind<T> {
    /// used as the node's label when printing
    pub fn name(&self) -> &'static str {
        match self {
            NodeKind::Scope { .. } => "Scope",
            NodeKind::BinaryOperation { .. } => "BinaryOperation",
            NodeKind::UnaryOperation { .. } => "UnaryOperation",
            NodeKind::Boolean(_) => "Boolean",
            NodeKind::Integer(_) => "Integer",
            NodeKind::Float(_) => "Float",
            NodeKind::Unit => "Unit",
            NodeKind::Matrix { .. } => "Matrix",
        }
    }
}

impl<T> NodeKind<T>
where
    Node<T>: fmt::Debug,
{
    /// formats this node under `label`
    pub fn fmt_labeled(&self, label: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::Scope { statements } => f.debug_tuple(label).field(statements).finish(),
            NodeKind::BinaryOperation { op, lhs, rhs } => f
                .debug_struct(label)
                .field("op", op)
                .field("lhs", lhs)
                .field("rhs", rhs)
                .finish(),
            NodeKind::UnaryOperation { op, subject } => f
                .debug_struct(label)
                .field("op", op)
                .field("subject", subject)
                .finish(),
            NodeKind::Boolean(v) => f.debug_tuple(label).field(v).finish(),
            NodeKind::Integer(v) => f.debug_tuple(label).field(v).finish(),
            NodeKind::Float(v) => f.debug_tuple(label).field(v).finish(),
            NodeKind::Unit => f.write_str(label),
            NodeKind::Matrix { elements } => {
                f.debug_struct(label).field("elements", elements).finish()
            }
        }
    }
}
