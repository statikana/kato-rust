use std::fmt;

use crate::datatype::Type;
use crate::typecheck::{ TypedNode, TypeError };
use super::{ Node, NodeData, Span };

pub struct Scope { pub statements: Vec<Node> }

impl NodeData for Scope {
    fn name(&self) -> &'static str { "Scope" }

    fn children(&self) -> Vec<&Node> {
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
