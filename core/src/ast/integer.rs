use std::fmt;

use crate::datatype::{ Type, IntegerType };
use crate::typecheck::{ TypedNode, TypeError };
use super::{ NodeData, Span };

pub struct Integer(pub i128);

impl NodeData for Integer {
    fn name(&self) -> &'static str { "Integer" }

    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Integer(IntegerType{ signed: true, nbytes: 8 })) // TODO: add actual ints
    }

    fn fmt_labeled(&self, label: &str, _children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(label).field(&self.0).finish()
    }
}
