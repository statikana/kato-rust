pub mod binary;
pub mod unary;

use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::types::Type;

use crate::type_layouts::Target;

pub trait BinaryOperatorInstructions {
    fn build(
        &self,
        lhs: ir::Value,
        lhs_type: &Type,
        rhs: ir::Value,
        rhs_type: &Type,
        result_type: &Type,
        target: &Target,
        builder: &mut FunctionBuilder,
    ) -> Option<ir::Value>;
}
