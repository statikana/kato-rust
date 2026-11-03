use ast::nodes::{ASTBinaryOperation, Span};

use crate::operator::BinaryTyping;
use crate::typecheck::{TypeError, Typecheck, TypedNode};
use crate::types::Type;

impl Typecheck for ASTBinaryOperation {
    // the operator decides the result from the operand types
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        let (lhs, rhs) = (&children[0].dtype, &children[1].dtype);
        // TODO: implementation of reverse operations is possible here
        self.op
            .result_type(lhs, rhs)
            .ok_or(TypeError::UnsupportedBinaryOperation {
                op: self.op,
                lhs: lhs.clone(),
                rhs: rhs.clone(),
                span,
            })
    }
}
