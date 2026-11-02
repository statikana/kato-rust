use std::fmt;

use crate::datatype::{ Type, FloatType };
use crate::typecheck::{ TypedNode, TypeError };
use super::{ NodeBehavior, Span };

pub struct ASTFloat(pub f64);

impl NodeBehavior for ASTFloat {
    fn name(&self) -> &'static str { "Float" }

    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Float(FloatType{ nbytes: 8 })) // TODO: add actual floats
    }

    fn fmt_labeled(&self, label: &str, _children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(label).field(&self.0).finish()
    }
}
