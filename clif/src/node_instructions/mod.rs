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

use crate::type_layouts::{HasLayout, Target};

// https://docs.rs/cranelift-frontend/latest/cranelift_frontend/index.html
pub trait NodeInstuctions {
    /// Adds instructions to the bulder to execute this node
    fn build(&self, target: &Target, builder: &mut FunctionBuilder) -> Option<ir::Value>;

    /// What clif type this node will result in needing, used to predict
    fn get_clif_type(&self, target: &Target) -> Option<ir::Type>;
}

impl NodeInstuctions for TypedNode {
    fn build(&self, target: &Target, builder: &mut FunctionBuilder) -> Option<ir::Value> {
        match &self.kind {
            NodeKind::Scope { statements } => {
                scope::build(statements, &self.dtype, target, builder)
            }
            NodeKind::BinaryOperation { op, lhs, rhs } => {
                binary_operation::build(*op, lhs, rhs, &self.dtype, target, builder)
            }
            NodeKind::UnaryOperation { op, subject } => {
                unary_operation::build(*op, subject, &self.dtype, target, builder)
            }
            NodeKind::Boolean(val) => boolean::build(*val, &self.dtype, target, builder),
            NodeKind::Integer(val) => integer::build(*val, &self.dtype, target, builder),
            NodeKind::Float(val) => float::build(*val, &self.dtype, target, builder),
            NodeKind::Unit => unit::build(&self.dtype, target, builder),
            NodeKind::Matrix { elements } => matrix::build(elements, &self.dtype, target, builder),
        }
    }

    fn get_clif_type(&self, target: &Target) -> Option<ir::Type> {
        self.dtype.layout(target).map(|l| l.clif_type)
    }
}
