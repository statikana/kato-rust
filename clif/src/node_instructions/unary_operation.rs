use ast::operator::UnaryOperator;
use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;
use typed::types::Type;

use crate::type_layouts::Target;

pub fn build(
    _op: UnaryOperator,
    _subject: &TypedNode,
    _result_type: &Type,
    _target: &Target,
    _builder: &mut FunctionBuilder,
) -> Option<ir::Value> {
    panic!();
}
