use std::fmt;

use crate::datatype::Type;
use crate::typecheck::{ TypedNode, TypeError };
use super::{ NodeData, Span };

pub struct Unit;

impl NodeData for Unit {
    fn name(&self) -> &'static str { "Unit" }

    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit)
    }

    fn fmt_labeled(&self, label: &str, _children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(label)
    }
}
