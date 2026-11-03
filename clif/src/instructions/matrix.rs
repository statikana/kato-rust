use super::Clif;
use ast::nodes::binary_operation::ASTBinaryOperation;
use ast::operator::BinaryOperator::*;
use cranelift_codegen::ir::{AbiParam, Signature};
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;

impl Clif for ast::nodes::ASTMatrix {
    fn build(&self, typed: &TypedNode, builder: &mut FunctionBuilder) {}
}
