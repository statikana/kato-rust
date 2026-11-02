use std::fmt;

use crate::nodes::{ASTNode, NodeBehavior};
use crate::operator::UnaryOperator;

pub struct ASTUnaryOperation {
    pub op: UnaryOperator,
    pub subject: Box<ASTNode>,
}

impl NodeBehavior for ASTUnaryOperation {
    fn name(&self) -> &'static str {
        "UnaryOperation"
    }

    fn children(&self) -> Vec<&ASTNode> {
        vec![&self.subject]
    }

    fn fmt_labeled(
        &self,
        label: &str,
        children: &[&dyn fmt::Debug],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct(label)
            .field("op", &&self.op)
            .field("subject", children[0])
            .finish()
    }
}
