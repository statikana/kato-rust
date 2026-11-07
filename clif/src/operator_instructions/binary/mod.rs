pub mod add;

use ast::operator::BinaryOperator;
use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::{typecheck::TypedNode, types::Type};

use super::BinaryOperatorInstructions;
use crate::{Codegen, type_layouts::Target};

impl BinaryOperatorInstructions for BinaryOperator {
    fn build(
        &self,
        lhs_value: ir::Value,
        lhs_type: &Type,
        rhs_value: ir::Value,
        rhs_type: &Type,
        result_type: &Type,
        target: &Target,
        builder: &mut FunctionBuilder,
    ) -> Option<ir::Value> {
        match self {
            BinaryOperator::Add => {
                add::build(lhs_value, lhs_type, rhs_value, rhs_type, result_type, target, builder)
            }
            BinaryOperator::Sub => todo!(),
            BinaryOperator::Mul => todo!(),
            BinaryOperator::Div => todo!(),
        }
    }
}

impl Codegen<'_, '_> {
    pub(crate) fn build_binary_operation(
        &mut self,
        result_type: &Type,
        op: BinaryOperator,
        lhs: &TypedNode,
        rhs: &TypedNode
    ) -> Option<ir::Value> {
        let lhs_value = self.build_node(lhs)?;
        let rhs_value = self.build_node(rhs)?;
        op.build(lhs_value, &lhs.dtype, rhs_value, &rhs.dtype, result_type, self.target, self.builder)
    }
}