use ast::operator::{BinaryOperator, UnaryOperator};

use crate::types::{Type, TypeAlgebra};

/// these link the BinaryOperator enum in AST
/// to the methods called by the type algebra helper
pub trait BinaryTyping {
    /// the type of `lhs op rhs`. `None` means the operand types don't support it
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type>;
}

pub trait UnaryTyping {
    fn result_type(&self, subject: &Type) -> Option<Type>;
}

impl BinaryTyping for BinaryOperator {
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type> {
        match self {
            BinaryOperator::Add => lhs.add(rhs),
            BinaryOperator::Sub => lhs.sub(rhs),
            BinaryOperator::Mul => lhs.mul(rhs),
            BinaryOperator::Div => lhs.div(rhs),
        }
    }
}

impl UnaryTyping for UnaryOperator {
    fn result_type(&self, subject: &Type) -> Option<Type> {
        match self {
            UnaryOperator::Negate => subject.negate(),
        }
    }
}
