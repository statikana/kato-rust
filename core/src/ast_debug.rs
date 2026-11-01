use std::fmt;

use crate::ast::{ Node, NodeData};
use crate::typing::TypedNode;
use crate::util::typename;

// manual Debug impls for the AST, kept out of ast.rs so the type definitions stay readable.

impl fmt::Debug for Node {
    // the span is folded into the node's label, e.g. `Integer@3..4(5)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = format!("{}@{}..{}", self.data.name(), self.span.start, self.span.end);
        fmt_labeled(&self.data, &label, f)
    }
}

impl fmt::Debug for TypedNode {
    // like `Node`, with the type appended to the label, e.g. `Integer@3..4: i64(5)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = format!("{}@{}..{}: {}", self.data.name(), self.span.start, self.span.end, self.dtype);
        fmt_labeled(&self.data, &label, f)
    }
}

/// formats `data` with `label` in place of its variant name
fn fmt_labeled(data: &NodeData, label: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    #[allow(unreachable_patterns)]
    match data {
        NodeData::Scope { statements } => f.debug_tuple(label).field(statements).finish(),
        NodeData::BinaryOperation { op, lhs, rhs } => f.debug_struct(label)
            .field("op", op)
            .field("lhs", lhs)
            .field("rhs", rhs)
            .finish(),
        NodeData::UrnaryOperation { op, subject } => f.debug_struct(label)
            .field("op", op)
            .field("subject", subject)
            .finish(),
        NodeData::Out(value) => f.debug_tuple(label).field(value).finish(),
        NodeData::Boolean(b) => f.debug_tuple(label).field(b).finish(),
        NodeData::Integer(i) => f.debug_tuple(label).field(i).finish(),
        NodeData::Float(x) => f.debug_tuple(label).field(x).finish(),
        NodeData::Unit() => f.write_str(label),
        _ => f.write_str(label),
    }
}

impl fmt::Debug for NodeData {
    // printed without a span; wrap in a Node to get positions
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(unreachable_patterns)]
        match self {
            NodeData::Scope { statements } => f.debug_tuple("Scope").field(statements).finish(),
            NodeData::BinaryOperation { op, lhs, rhs } => f.debug_struct("BinaryOperation")
                .field("op", op)
                .field("lhs", lhs)
                .field("rhs", rhs)
                .finish(),
            NodeData::UrnaryOperation { op, subject } => f.debug_struct("UrnaryOperation")
                .field("op", op)
                .field("subject", subject)
                .finish(),
            NodeData::Out(value) => f.debug_tuple("Out").field(value).finish(),
            NodeData::Boolean(b) => f.debug_tuple("Boolean").field(b).finish(),
            NodeData::Integer(i) => f.debug_tuple("Integer").field(i).finish(),
            NodeData::Float(x) => f.debug_tuple("Float").field(x).finish(),
            NodeData::Unit() => f.write_str("Unit"),
            _ => f.write_str(&("unknown: ".to_owned() + typename(self))),
        }
    } 
}

impl NodeData {
    fn name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match self {
            NodeData::Scope { .. } => "Scope",
            NodeData::BinaryOperation { .. } => "BinaryOperation",
            NodeData::UrnaryOperation { .. } => "UrnaryOperation",
            NodeData::Out(_) => "Out",
            NodeData::Boolean(_) => "Boolean",
            NodeData::Integer(_) => "Integer",
            NodeData::Float(_) => "Float",
            NodeData::Unit() => "Unit",
            _ => typename(self),
        }
    }
}
