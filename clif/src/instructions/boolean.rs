use super::Clif;

use cranelift_codegen::ir::Value;
use cranelift_codegen::ir::types::I8;
use cranelift_codegen::ir::InstBuilder;
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;

impl Clif for ast::nodes::ASTBoolean {
    // we don't need the `typed` version of this node because it's pretty damn obvious
    // what type this ...
    fn build(&self, _typed: &TypedNode, builder: &mut FunctionBuilder) -> Value {
        builder.ins().iconst(I8, self.0 as i64)
    }
}
