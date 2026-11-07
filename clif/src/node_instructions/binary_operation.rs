use ast::operator::BinaryOperator;
use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;
use typed::types::Type;

use super::NodeInstuctions;
use crate::operator_instructions::BinaryOperatorInstructions;
use crate::type_layouts::Target;

pub fn build(
    op: BinaryOperator,
    lhs: &TypedNode,
    rhs: &TypedNode,
    result_type: &Type,
    target: &Target,
    builder: &mut FunctionBuilder,
) -> Option<ir::Value> {
    // this should return the type from get_clif_type
    // TODO: actually enforce this? ^

    // first need to build children
    // TODO: error on child with no value? for now just propagate
    let lhs_value = lhs.build(target, builder)?;
    let rhs_value = rhs.build(target, builder)?;

    op.build(
        lhs_value,
        &lhs.dtype,
        rhs_value,
        &rhs.dtype,
        result_type,
        target,
        builder,
    )
}
