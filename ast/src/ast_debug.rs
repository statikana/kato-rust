use std::fmt;

use crate::nodes::ASTNode;

// Debug impls for the AST, kept out of nodes/mod.rs so the type definitions stay readable.
// each node kind formats itself through `NodeData::fmt_labeled`; these just build the label.

impl fmt::Debug for ASTNode {
    // the span is folded into the node's label, e.g. `Integer@3..4(5)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = format!(
            "{}@{}..{}",
            self.kind.behavior().name(),
            self.span.start,
            self.span.end
        );
        let children = self.kind.behavior().children();
        let children: Vec<&dyn fmt::Debug> =
            children.iter().map(|c| *c as &dyn fmt::Debug).collect();
        self.kind.behavior().fmt_labeled(&label, &children, f)
    }
}
