pub mod node_instructions;
pub mod operator_instructions;
pub mod type_layouts;

use ast::nodes::NodeKind;
use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use typed::typecheck::TypedNode;

use crate::type_layouts::Target;

fn main(entry: TypedNode) {}


struct Codegen<'a, 'b> {
    pub builder: &'a mut FunctionBuilder<'b>,
    pub target: &'b Target
}


impl Codegen<'_, '_> {
    /// general entry point for deciding how to build a node
    pub fn build_node(&mut self, node: &TypedNode) -> Option<ir::Value> {
        // map node => instructions
        match &node.kind {
            NodeKind::Integer(value) => self.build_integer(&node.dtype, *value),
            NodeKind::Float(value) => self.build_float(&node.dtype, *value),
            NodeKind::Boolean(value) => self.build_boolean(&node.dtype, *value),

            NodeKind::BinaryOperation { op, lhs, rhs } => self.build_binary_operation(&node.dtype, *op, lhs, rhs),
            NodeKind::UnaryOperation { op, subject } => self.build_unary_operation(&node.dtype, *op, subject),
            NodeKind::Matrix { elements } => self.build_matrix(&node.dtype, elements),
            NodeKind::Unit => self.build_unit(&node.dtype),

            NodeKind::Scope { statements } => self.build_scope(&node.dtype, statements)
        }
    }
}