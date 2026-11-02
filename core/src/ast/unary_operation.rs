use std::fmt;

use crate::types::Type;
use crate::operator::UnaryOperator;
use crate::typecheck::{ TypedNode, TypeError };
use super::{ ASTNode, NodeBehavior, Span };

pub struct ASTUnaryOperation { pub op: &'static dyn UnaryOperator, pub subject: Box<ASTNode> }

impl NodeBehavior for ASTUnaryOperation {
    fn name(&self) -> &'static str { "UnaryOperation" }

    fn children(&self) -> Vec<&ASTNode> {
        vec![&self.subject]
    }

    // the operator decides the result from the subject's type
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        let subject = &children[0].dtype;
        self.op.result_type(subject)
            .ok_or(TypeError::UnsupportedUnaryOperation { op: self.op, subject: subject.clone(), span })
    }

    fn fmt_labeled(&self, label: &str, children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(label)
            .field("op", &self.op)
            .field("subject", children[0])
            .finish()
    }
}
