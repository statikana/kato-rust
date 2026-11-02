use std::fmt;

use crate::nodes::{ASTNode, NodeBehavior};

pub struct ASTMatrix {
    pub elements: Vec<ASTNode>,
}

impl NodeBehavior for ASTMatrix {
    fn name(&self) -> &'static str {
        "Matrix"
    }

    // since iterators create reference, iter().collect() can be
    // used to create references of each child
    fn children(&self) -> Vec<&ASTNode> {
        self.elements.iter().collect()
    }

    fn fmt_labeled(
        &self,
        label: &str,
        children: &[&dyn fmt::Debug],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct(label).field("elements", &children).finish()
    }
}
