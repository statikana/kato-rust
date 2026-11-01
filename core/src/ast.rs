use std::fmt;

use crate::typecheck::{ TypedNode, TypeError };
use crate::datatype::Type;

mod scope;
mod binary_operation;
mod unary_operation;
mod out;
mod boolean;
mod integer;
mod float;
mod unit;

pub use scope::Scope;
pub use binary_operation::BinaryOperation;
pub use unary_operation::UnaryOperation;
pub use out::Out;
pub use boolean::Boolean;
pub use integer::Integer;
pub use float::Float;
pub use unit::Unit;

#[derive(Clone, Copy, Debug)]
pub struct Span { pub start: u32, pub end: u32 }


// untyped (before-typechecker) node
pub struct Node { pub data: Box<dyn NodeData>, pub span: Span }

impl Node {
    pub fn new(data: impl NodeData + 'static, span: Span) -> Node {
        Node{ data: Box::new(data), span }
    }
}

/// behaviour of a single kind of node
pub trait NodeData {
    /// used as the node's label when printing
    fn name(&self) -> &'static str;

    /// child nodes, in evaluation order
    fn children(&self) -> Vec<&Node> { Vec::new() }

    /// the type this node evaluates to. `children` are the typed `children()`, in the same order
    fn typecheck(&self, children: &[TypedNode], span: Span) -> Result<Type, TypeError>;

    /// formats this node under `label`. `children` are the (typed or untyped) `children()`, in the same order
    fn fmt_labeled(&self, label: &str, children: &[&dyn fmt::Debug], f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple(label);
        for child in children {
            tuple.field(child);
        }
        tuple.finish()
    }

    fn leaf(&self) -> bool {
        self.children().len() == 0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator { Add, Sub, Mul, Div }

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator { Negate }
