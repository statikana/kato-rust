use std::fmt;

use crate::nodes::{Node, Span};

// Debug impls for the AST, kept out of nodes/mod.rs so the type definitions stay readable.
// each node kind formats itself through `NodeKind::fmt_labeled`; these just build the label.

pub trait DebugTypeAnnotation {
    fn format_label(&self, name: &str, span: Span) -> String;
}

impl DebugTypeAnnotation for () {
    // the span is folded into the node's label, e.g. `Integer@3..4(5)`
    fn format_label(&self, name: &str, span: Span) -> String {
        format!("{}@{}..{}", name, span.start, span.end)
    }
}

impl<T: DebugTypeAnnotation> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.dtype.format_label(self.kind.name(), self.span);
        self.kind.fmt_labeled(&label, f)
    }
}
