use crate::datatype::{ Type, TypeAlgebra };
use super::BinaryOperator;

pub struct Sub;

impl BinaryOperator for Sub {
    fn name(&self) -> &'static str { "Sub" }
    fn symbol(&self) -> &'static str { "-" }
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type> { lhs.sub(rhs) }
}
