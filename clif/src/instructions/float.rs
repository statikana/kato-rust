use crate::layout::Target;

use super::Clif;

use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;

impl Clif for ast::nodes::ASTFloat {
    fn build(&self, typed: &TypedNode, target: &Target, builder: &mut FunctionBuilder) -> Option<ir::Value> {
        panic!();
    }
    fn get_clif_type(&self, typed: &TypedNode, target: &Target) -> Option<ir::Type> {
        panic!()
    }
}
