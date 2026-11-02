use ast::nodes::{ASTFloat, Span};

use crate::typecheck::{Typecheck, TypeError, TypedNode};
use crate::types::{FloatType, Type};

impl Typecheck for ASTFloat {
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Float(FloatType { nbytes: 8 })) // TODO: add actual floats
    }
}
