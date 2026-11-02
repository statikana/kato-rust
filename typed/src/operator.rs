use ast::operator::{BinaryOperator, UnaryOperator};

use crate::types::{Type, TypeAlgebra};

// these map the Operators in `ast` to the operand's respective type algebra function (`.add`, `.div`, etc.)

pub trait BinaryTyping {
    /// the type of `lhs op rhs`. `None` means the operand types don't support it
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type>;
}

pub trait UnaryTyping {
    /// the type of `op subject`. `None` means the subject's type doesn't support it
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
