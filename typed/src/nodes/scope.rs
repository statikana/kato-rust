use ast::nodes::{ASTScope, Span};

use crate::typecheck::{TypeError, Typecheck, TypedNode};
use crate::types::{Type, UnitType};

impl Typecheck for ASTScope {
    // statements are checked, but don't produce a value
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit(UnitType {}))
    }
}
