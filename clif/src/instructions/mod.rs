pub mod binary_operation;
pub mod boolean;
pub mod float;
pub mod integer;
pub mod matrix;
pub mod scope;
pub mod unary_operation;
pub mod unit;

use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

use ast::nodes::NodeKind;
use typed::typecheck::TypedNode;

// https://docs.rs/cranelift-frontend/latest/cranelift_frontend/index.html
trait Clif {
    /// Adds instructions to the bulder to execute this node.
    fn build(&self, typed: &TypedNode, builder: &mut FunctionBuilder) -> Value;
}

macro_rules! impl_clif_for_nodes {
    ($($variant:ident($node:ty)),* $(,)?) => {
        impl Clif for NodeKind {
            fn build(&self, typed: &TypedNode, builder: &mut FunctionBuilder) -> Value {
                match self {
                    $(NodeKind::$variant(n) => n.build(typed, builder)),*
                }
            }
        }
    };
}

ast::for_each_node_kind!(impl_clif_for_nodes);
