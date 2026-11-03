pub mod binary_operation;
pub mod boolean;
pub mod float;
pub mod integer;
pub mod matrix;
pub mod scope;
pub mod unary_operation;
pub mod unit;

use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;

use ast::nodes::NodeKind;
use typed::typecheck::TypedNode;

use crate::type_layouts::Target;

// https://docs.rs/cranelift-frontend/latest/cranelift_frontend/index.html
trait Instuctions {
    /// Adds instructions to the bulder to execute this node
    fn build(
        &self,
        typed: &TypedNode,
        target: &Target,
        builder: &mut FunctionBuilder,
    ) -> Option<ir::Value>;

    /// What clif type this node will result in needing, used to predict
    fn get_clif_type(&self, typed: &TypedNode, target: &Target) -> Option<ir::Type>;
}

macro_rules! impl_clif_for_nodes {
    ($($variant:ident($node:ty)),* $(,)?) => {
        impl Instuctions for NodeKind {
            fn build(&self, typed: &TypedNode, target: &Target, builder: &mut FunctionBuilder) -> Option<ir::Value> {
                match self {
                    $(NodeKind::$variant(n) => n.build(typed, target, builder)),*
                }
            }
            fn get_clif_type(&self, typed: &TypedNode, target: &Target) -> Option<ir::Type> {
                match self {
                    $(NodeKind::$variant(n) => n.get_clif_type(typed, target)),*
                }
            }
        }
    };
}

ast::for_each_node_kind!(impl_clif_for_nodes);
