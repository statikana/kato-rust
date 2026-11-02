use std::fmt;

use crate::types::Type;
use crate::typecheck::{ TypedNode, TypeError };
use super::{ ASTNode, NodeBehavior, Span };

pub struct ASTScope { pub statements: Vec<ASTNode> }

impl NodeBehavior for ASTScope {
    fn name(&self) -> &'static str { "Scope" }

    fn children(&self) -> Vec<&ASTNode> {
        self.statements.iter().collect()
    }

    // statements are checked, but don't produce a value
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit)
    }

    fn fmt_labeled(&self, label: &str, children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(label).field(&children).finish()
    }
}
