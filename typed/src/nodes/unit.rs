use ast::nodes::{ASTUnit, Span};

use crate::typecheck::{TypeError, Typecheck, TypedNode};
use crate::types::{ Type, UnitType };

impl Typecheck for ASTUnit {
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit(UnitType{}))
    }
}
