use crate::types::{ Type, TypeAlgebra };
use super::BinaryOperator;

pub struct Mul;

impl BinaryOperator for Mul {
    fn name(&self) -> &'static str { "Mul" }
    fn symbol(&self) -> &'static str { "*" }
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type> { lhs.mul(rhs) }
}
