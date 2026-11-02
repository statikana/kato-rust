use crate::datatype::{ Type, TypeAlgebra };
use super::BinaryOperator;

pub struct Div;

impl BinaryOperator for Div {
    fn name(&self) -> &'static str { "Div" }
    fn symbol(&self) -> &'static str { "/" }
    fn result_type(&self, lhs: &Type, rhs: &Type) -> Option<Type> { lhs.div(rhs) }
}
