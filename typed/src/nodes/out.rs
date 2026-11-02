use ast::nodes::{ASTOut, Span};

use crate::typecheck::{Typecheck, TypeError, TypedNode};
use crate::types::Type;

impl Typecheck for ASTOut {
    // printing doesn't produce a value
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit)
    }
}
