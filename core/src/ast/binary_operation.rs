use std::fmt;

use crate::datatype::{ Type, Numeric };
use crate::typecheck::{ TypedNode, TypeError };
use super::{ Node, NodeData, Span, BinaryOperator };

pub struct BinaryOperation { pub op: BinaryOperator, pub lhs: Box<Node>, pub rhs: Box<Node> }

impl NodeData for BinaryOperation {
    fn name(&self) -> &'static str { "BinaryOperation" }

    fn children(&self) -> Vec<&Node> {
        vec![&self.lhs, &self.rhs]
    }

    // the operand types decide the result
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        let (lhs, rhs) = (&children[0].dtype, &children[1].dtype);
        // TODO: implementation of reverse operations is possible here
        lhs.binary_result(self.op, rhs)
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
