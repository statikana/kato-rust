use std::fmt;

use crate::nodes::{ASTNode, NodeBehavior};
use crate::operator::BinaryOperator;

pub struct ASTBinaryOperation {
    pub op: BinaryOperator,
    pub lhs: Box<ASTNode>,
    pub rhs: Box<ASTNode>,
}

impl NodeBehavior for ASTBinaryOperation {
    fn name(&self) -> &'static str {
        "BinaryOperation"
    }

    fn children(&self) -> Vec<&ASTNode> {
        vec![&self.lhs, &self.rhs]
    }

    fn fmt_labeled(
        &self,
        label: &str,
        children: &[&dyn fmt::Debug],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct(label)
            .field("op", &self.op)
            .field("lhs", children[0])
            .field("rhs", children[1])
            .finish()
    }
}
