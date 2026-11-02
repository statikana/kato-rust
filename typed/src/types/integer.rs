use std::cmp::max;
use std::fmt;

use super::{HasLayout, Layout, Target, Type, TypeAlgebra};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerType {
    pub signed: bool,
    pub nbytes: usize,
}

impl IntegerType {
    // integer with integer: signed if either side is, widened to the larger side
    fn arithmetic(&self, rhs: &Type) -> Option<Type> {
        match rhs {
            Type::Integer(rhs) => Some(Type::Integer(IntegerType {
                signed: self.signed || rhs.signed,
                nbytes: max(self.nbytes, rhs.nbytes),
            })),
            _ => None,
        }
    }
}

impl TypeAlgebra for IntegerType {
    // negation only makes sense for signed integers
    fn negate(&self) -> Option<Type> {
        self.signed.then(|| Type::Integer(self.clone()))
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

impl HasLayout for IntegerType {
    fn layout(&self, _target: &Target) -> Option<Layout> {
        Some(Layout::primitive(self.nbytes))
    }
}

impl fmt::Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.signed { "i" } else { "u" },
            self.nbytes * 8
        )
    }
}
