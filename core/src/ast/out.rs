use crate::datatype::Type;
use crate::typecheck::{ TypedNode, TypeError };
use super::{ ASTNode, NodeBehavior, Span };

pub struct ASTOut { pub value: Box<ASTNode> }

impl NodeBehavior for ASTOut {
    fn name(&self) -> &'static str { "Out" }

    fn children(&self) -> Vec<&ASTNode> {
        vec![&self.value]
    }

    // printing doesn't produce a value
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit)
    }
}
