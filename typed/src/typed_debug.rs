use std::fmt;

use crate::typecheck::TypedNode;

// Debug impl for the typed AST, the counterpart of `ast::ast_debug`.
// each node kind formats itself through `NodeBehavior::fmt_labeled`; this just builds the label.

impl fmt::Debug for TypedNode<'_> {
    // like `ASTNode`, with the type appended to the label, e.g. `Integer@3..4: i64(5)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (data, span) = (self.node.kind.behavior(), self.node.span);
        let label = format!(
            "{}@{}..{}: {}",
            data.name(),
            span.start,
            span.end,
            self.dtype
        );
        let children: Vec<&dyn fmt::Debug> =
            self.children.iter().map(|c| c as &dyn fmt::Debug).collect();
        data.fmt_labeled(&label, &children, f)
    }
}
