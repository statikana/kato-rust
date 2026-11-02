use std::fmt;

use crate::nodes::{ASTNode, NodeBehavior};

pub struct ASTScope {
    pub statements: Vec<ASTNode>,
}

impl NodeBehavior for ASTScope {
    fn name(&self) -> &'static str {
        "Scope"
    }

    fn children(&self) -> Vec<&ASTNode> {
        self.statements.iter().collect()
    }

    fn fmt_labeled(
        &self,
        label: &str,
        children: &[&dyn fmt::Debug],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_tuple(label).field(&children).finish()
    }
}
