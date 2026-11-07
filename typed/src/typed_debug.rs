use ast::ast_debug::DebugTypeAnnotation;
use ast::nodes::Span;

use crate::types::Type;

// Debug label formatting for the typed AST, the counterpart of `ast::ast_debug`.
// each node kind formats itself through `NodeKind::fmt_labeled`; this just builds the label.

impl DebugTypeAnnotation for Type {
    // like `ASTNode`, with the type appended to the label, e.g. `Integer@3..4: i64(5)`
    fn format_label(&self, name: &str, span: Span) -> String {
        format!("{}@{}..{}: {}", name, span.start, span.end, self)
    }
}
