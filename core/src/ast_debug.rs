use std::fmt;

use crate::ast::{ Node, Data, Scope };

// manual Debug impls for the AST, kept out of ast.rs so the type definitions stay readable.
// built on the std debug builders, so `{:#?}` gives an indented tree.

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.statements).finish()
    }
}

impl fmt::Debug for Node {
    // the span is folded into the node's label, e.g. `Integer@3..4(5)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = format!("{}@{}..{}", self.data.name(), self.span.start, self.span.end);
        match &self.data {
            Data::Scope(scope) => f.debug_tuple(&label).field(scope).finish(),
            Data::BinOperation { op, lhs, rhs } => f.debug_struct(&label)
                .field("op", op)
                .field("lhs", lhs)
                .field("rhs", rhs)
                .finish(),
            Data::UrnaryOperation { op, subject } => f.debug_struct(&label)
                .field("op", op)
                .field("subject", subject)
                .finish(),
            Data::Out(value) => f.debug_tuple(&label).field(value).finish(),
            Data::Boolean(b) => f.debug_tuple(&label).field(b).finish(),
            Data::Integer(i) => f.debug_tuple(&label).field(i).finish(),
            Data::Float(x) => f.debug_tuple(&label).field(x).finish(),
            Data::Null() => f.write_str(&label),
        }
    }
}

impl fmt::Debug for Data {
    // printed without a span; wrap in a Node to get positions
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Scope(scope) => f.debug_tuple("Scope").field(scope).finish(),
            Data::BinOperation { op, lhs, rhs } => f.debug_struct("BinOperation")
                .field("op", op)
                .field("lhs", lhs)
                .field("rhs", rhs)
                .finish(),
            Data::UrnaryOperation { op, subject } => f.debug_struct("UrnaryOperation")
                .field("op", op)
                .field("subject", subject)
                .finish(),
            Data::Out(value) => f.debug_tuple("Out").field(value).finish(),
            Data::Boolean(b) => f.debug_tuple("Boolean").field(b).finish(),
            Data::Integer(i) => f.debug_tuple("Integer").field(i).finish(),
            Data::Float(x) => f.debug_tuple("Float").field(x).finish(),
            Data::Null() => f.write_str("Null"),
        }
    }
}

impl Data {
    fn name(&self) -> &'static str {
        match self {
            Data::Scope(_) => "Scope",
            Data::BinOperation { .. } => "BinOperation",
            Data::UrnaryOperation { .. } => "UrnaryOperation",
            Data::Out(_) => "Out",
            Data::Boolean(_) => "Boolean",
            Data::Integer(_) => "Integer",
            Data::Float(_) => "Float",
            Data::Null() => "Null",
        }
    }
}
