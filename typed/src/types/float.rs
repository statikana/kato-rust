use std::cmp::max;
use std::fmt;

use super::{Type, TypeAlgebra};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FloatType {
    pub nbytes: usize,
}

impl FloatType {
    // float with float: widened to the larger side
    fn arithmetic(&self, rhs: &Type) -> Option<Type> {
        match rhs {
            Type::Float(rhs) => Some(Type::Float(FloatType {
                nbytes: max(self.nbytes, rhs.nbytes),
            })),
            _ => None,
        }
    }
}

impl TypeAlgebra for FloatType {
    fn negate(&self) -> Option<Type> {
        Some(Type::Float(self.clone()))
    }

    fn add(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
    fn sub(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
    fn mul(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
    fn div(&self, rhs: &Type) -> Option<Type> {
        self.arithmetic(rhs)
    }
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.nbytes * 8)
    }
}
