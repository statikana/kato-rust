use std::fmt;

use super::{ Type, TypeAlgebra, HasLayout, Layout, Target };

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PtrType { pub pointee: Box<Type> }

// no arithmetic on pointers
impl TypeAlgebra for PtrType {}

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
