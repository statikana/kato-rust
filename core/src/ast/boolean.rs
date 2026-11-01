use std::fmt;

use crate::datatype::{ Type };
use crate::typecheck::{ TypedNode, TypeError };
use super::{ NodeData, Span };

pub struct Boolean(pub bool);

impl NodeData for Boolean {
    fn name(&self) -> &'static str { "Boolean" }

    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Boolean)
    }

    fn fmt_labeled(&self, label: &str, _children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(label).field(&self.0).finish()
    }
}
