use ast::nodes::{ASTUnit, Span};

use crate::typecheck::{Typecheck, TypeError, TypedNode};
use crate::types::Type;

impl Typecheck for ASTUnit {
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit)
    }
}
