use ast::nodes::{ASTBoolean, Span};

use crate::typecheck::{Typecheck, TypeError, TypedNode};
use crate::types::Type;

impl Typecheck for ASTBoolean {
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Boolean)
    }
}
