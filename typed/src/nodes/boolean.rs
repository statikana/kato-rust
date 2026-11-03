use ast::nodes::{ASTBoolean, Span};

use crate::typecheck::{TypeError, Typecheck, TypedNode};
use crate::types::{BooleanType, Type};

impl Typecheck for ASTBoolean {
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Boolean(BooleanType{}))
    }
}
