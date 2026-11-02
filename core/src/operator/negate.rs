use crate::types::{ Type, TypeAlgebra };
use super::UnaryOperator;

pub struct Negate;

impl UnaryOperator for Negate {
    fn name(&self) -> &'static str { "Negate" }
    fn symbol(&self) -> &'static str { "-" }
    fn result_type(&self, subject: &Type) -> Option<Type> { subject.negate() }
}
