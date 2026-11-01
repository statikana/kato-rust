use std::fmt;

use crate::datatype::{ Type, Numeric };
use crate::typecheck::{ TypedNode, TypeError };
use super::{ Node, NodeData, Span, UrnaryOperator };

pub struct UrnaryOperation { pub op: UrnaryOperator, pub subject: Box<Node> }

impl NodeData for UrnaryOperation {
    fn name(&self) -> &'static str { "UrnaryOperation" }

    fn children(&self) -> Vec<&Node> {
        vec![&self.subject]
    }

    // the subject's type decides the result
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError> {
        let subject = &children[0].dtype;
        subject.unary_result(self.op)
            .ok_or(TypeError::UnsupportedUrnaryOperation { op: self.op, subject: subject.clone(), span })
    }

    fn fmt_labeled(&self, label: &str, children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(label)
            .field("op", &self.op)
            .field("subject", children[0])
            .finish()
    }
}
