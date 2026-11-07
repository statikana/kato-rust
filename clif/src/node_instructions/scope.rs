use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;
use typed::types::Type;

use crate::type_layouts::Target;

pub fn build(
    _statements: &[TypedNode],
    _dtype: &Type,
    _target: &Target,
    _builder: &mut FunctionBuilder,
) -> Option<ir::Value> {
    panic!();
}
