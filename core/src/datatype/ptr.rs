use std::fmt;

use crate::ast::{ BinaryOperator, UrnaryOperator };
use super::{ Type, Numeric, HasLayout, Layout, Target };

#[derive(Clone, Debug)]
pub struct PtrType { pub pointee: Box<Type> }

impl Numeric for PtrType {
    fn unary_result(&self, _op: UrnaryOperator) -> Option<Type> { None }
    fn binary_result(&self, _op: BinaryOperator, _rhs: &Type) -> Option<Type> { None }
}

impl HasLayout for PtrType {
    fn layout(&self, target: &Target) -> Option<Layout> {
        Some(Layout::primitive(target.ptr_size))
    }
}

impl fmt::Display for PtrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*{}", self.pointee)
    }
}
