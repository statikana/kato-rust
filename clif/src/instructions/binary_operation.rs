use super::Clif;
use cranelift_codegen::ir::{AbiParam, Signature, Value};
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;

impl Clif for ast::nodes::ASTBinaryOperation {
    fn build(&self, typed: &TypedNode, builder: &mut FunctionBuilder) -> Value {
        
    }
}
