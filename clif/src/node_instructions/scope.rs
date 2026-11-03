use crate::type_layouts::Target;

use super::Instuctions;

use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;

impl Instuctions for ast::nodes::ASTScope {
    fn build(
        &self,
        typed: &TypedNode,
        target: &Target,
        builder: &mut FunctionBuilder,
    ) -> Option<ir::Value> {
        panic!();
    }
    fn get_clif_type(&self, typed: &TypedNode, target: &Target) -> Option<ir::Type> {
        panic!()
    }
}
