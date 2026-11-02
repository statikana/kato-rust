use ast::nodes::{ASTInteger, Span};

use crate::typecheck::{Typecheck, TypeError, TypedNode};
use crate::types::{IntegerType, Type};

impl Typecheck for ASTInteger {
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Integer(IntegerType {
            signed: true,
            nbytes: 8,
        })) // TODO: add actual ints
    }
}
