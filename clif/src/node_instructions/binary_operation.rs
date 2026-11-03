use crate::type_layouts::{HasLayout, Target};

use super::Instuctions;
use ast::nodes::NodeBehavior;
use cranelift_codegen::ir::AbiParam;
use cranelift_codegen::{ir, isa};
use cranelift_frontend::FunctionBuilder;
use typed::operator::BinaryTyping;
use typed::typecheck::{Typecheck, TypedNode};

use typed::types::Type;

impl Instuctions for ast::nodes::ASTBinaryOperation {
    fn build(
        &self,
        typed: &TypedNode,
        target: &Target,
        builder: &mut FunctionBuilder,
    ) -> Option<ir::Value> {
        panic!()
    }

    fn get_clif_type(&self, typed: &TypedNode, target: &Target) -> Option<ir::Type> {
        let lhs_type = &typed.children[0].dtype;
        let rhs_type = &typed.children[1].dtype;

        let return_type = self.op.result_type(lhs_type, rhs_type).unwrap();
        return Some(return_type.layout(target)?.clif_type);
    }
}
