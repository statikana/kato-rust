use crate::nodes::{ASTNode, NodeBehavior};

pub struct ASTOut {
    pub value: Box<ASTNode>,
}

impl NodeBehavior for ASTOut {
    fn name(&self) -> &'static str {
        "Out"
    }

    fn children(&self) -> Vec<&ASTNode> {
        vec![&self.value]
    }
}
