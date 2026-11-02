use crate::types::{ Type, TypeAlgebra };
use super::BinaryOperator;

pub struct Add;

impl BinaryOperator for Add {
    fn name(&self) -> &'static str { "Add" }
    fn symbol(&self) -> &'static str { "+" }
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type> { lhs.add(rhs) }
}
