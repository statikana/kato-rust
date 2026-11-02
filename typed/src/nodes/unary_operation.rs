use ast::nodes::{ASTUnaryOperation, Span};

use crate::operator::UnaryTyping;
use crate::typecheck::{Typecheck, TypeError, TypedNode};
use crate::types::Type;

impl Typecheck for ASTUnaryOperation {
    // the operator decides the result from the subject's type
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        let subject = &children[0].dtype;
        self.op
            .result_type(subject)
            .ok_or(TypeError::UnsupportedUnaryOperation {
                op: self.op,
                subject: subject.clone(),
                span,
            })
    }
}
