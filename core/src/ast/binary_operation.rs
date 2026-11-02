use std::fmt;

use crate::datatype::Type;
use crate::operator::BinaryOperator;
use crate::typecheck::{ TypedNode, TypeError };
use super::{ ASTNode, NodeBehavior, Span };

pub struct ASTBinaryOperation { pub op: &'static dyn BinaryOperator, pub lhs: Box<ASTNode>, pub rhs: Box<ASTNode> }

impl NodeBehavior for ASTBinaryOperation {
    fn name(&self) -> &'static str { "BinaryOperation" }

    fn children(&self) -> Vec<&ASTNode> {
        vec![&self.lhs, &self.rhs]
    }

    // the operator decides the result from the operand types
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        let (lhs, rhs) = (&children[0].dtype, &children[1].dtype);
        // TODO: implementation of reverse operations is possible here
        self.op.result_type(lhs, rhs)
            .ok_or(TypeError::UnsupportedBinaryOperation { op: self.op, lhs: lhs.clone(), rhs: rhs.clone(), span })
    }

    fn fmt_labeled(&self, label: &str, children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(label)
            .field("op", &self.op)
            .field("lhs", children[0])
            .field("rhs", children[1])
            .finish()
    }
}
