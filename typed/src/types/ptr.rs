use std::fmt;

use super::{Type, TypeAlgebra};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PtrType {
    pub pointee: Box<Type>,
}

// no arithmetic on pointers
impl TypeAlgebra for PtrType {}

impl fmt::Display for PtrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*{}", self.pointee)
    }
}
