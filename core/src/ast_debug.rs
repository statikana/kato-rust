use std::fmt;

use crate::ast::Node;
use crate::typecheck::TypedNode;

// Debug impls for the AST, kept out of ast.rs so the type definitions stay readable.
// each node kind formats itself through `NodeData::fmt_labeled`; these just build the label.

impl fmt::Debug for Node {
    // the span is folded into the node's label, e.g. `Integer@3..4(5)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = format!("{}@{}..{}", self.data.name(), self.span.start, self.span.end);
        let children = self.data.children();
        let children: Vec<&dyn fmt::Debug> = children.iter().map(|c| *c as &dyn fmt::Debug).collect();
        self.data.fmt_labeled(&label, &children, f)
    }
}

impl fmt::Debug for TypedNode<'_> {
    // like `Node`, with the type appended to the label, e.g. `Integer@3..4: i64(5)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (data, span) = (&self.node.data, self.node.span);
        let label = format!("{}@{}..{}: {}", data.name(), span.start, span.end, self.dtype);
        let children: Vec<&dyn fmt::Debug> = self.children.iter().map(|c| c as &dyn fmt::Debug).collect();
        data.fmt_labeled(&label, &children, f)
    }
}
