use std::cmp::max;
use std::fmt;

use crate::ast::{ BinaryOperator, UrnaryOperator };
use super::{ Type, Numeric, HasLayout, Layout, Target };

#[derive(Clone, Debug)]
pub struct FloatType { pub nbytes: usize }

impl Numeric for FloatType {
    fn unary_result(&self, op: UrnaryOperator) -> Option<Type> {
        match op {
            UrnaryOperator::Negate => Some(Type::Float(self.clone())),
        }
    }

    // float with float: widened to the larger side
    fn binary_result(&self, _op: BinaryOperator, rhs: &Type) -> Option<Type> {
        match rhs {
            Type::Float(rhs) => Some(Type::Float(FloatType { nbytes: max(self.nbytes, rhs.nbytes) })),
            _ => None,
        }
    }
}

impl HasLayout for FloatType {
    fn layout(&self, _target: &Target) -> Option<Layout> {
        Some(Layout::primitive(self.nbytes))
    }
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.nbytes * 8)
    }
}
