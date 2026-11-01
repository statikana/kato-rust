use std::cmp::max;
use std::fmt;

use crate::ast::{ BinaryOperator, UrnaryOperator };
use super::{ Type, Numeric, HasLayout, Layout, Target };

#[derive(Clone, Debug)]
pub struct IntegerType { pub signed: bool, pub nbytes: usize }

impl Numeric for IntegerType {
    // negation only makes sense for signed integers
    fn unary_result(&self, op: UrnaryOperator) -> Option<Type> {
        match op {
            UrnaryOperator::Negate if self.signed => Some(Type::Integer(self.clone())),
            UrnaryOperator::Negate => None,
        }
    }

    // integer with integer: signed if either side is, widened to the larger side
    fn binary_result(&self, _op: BinaryOperator, rhs: &Type) -> Option<Type> {
        match rhs {
            Type::Integer(rhs) => Some(Type::Integer(IntegerType {
                signed: self.signed || rhs.signed,
                nbytes: max(self.nbytes, rhs.nbytes),
            })),
            _ => None,
        }
    }
}

impl HasLayout for IntegerType {
    fn layout(&self, _target: &Target) -> Option<Layout> {
        Some(Layout::primitive(self.nbytes))
    }
}

impl fmt::Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.signed { "i" } else { "u" }, self.nbytes * 8)
    }
}
