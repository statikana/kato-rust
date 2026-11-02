use std::fmt;

use crate::typecheck::{ TypedNode, TypeError };
use crate::types::Type;

mod scope;
mod binary_operation;
mod unary_operation;
mod out;
mod boolean;
mod integer;
mod float;
mod unit;
mod matrix;

pub use scope::ASTScope;
pub use binary_operation::ASTBinaryOperation;
pub use unary_operation::ASTUnaryOperation;
pub use boolean::ASTBoolean;
pub use integer::ASTInteger;
pub use float::ASTFloat;
pub use unit::ASTUnit;
pub use matrix::ASTMatrix;

#[derive(Clone, Copy, Debug)]
pub struct Span { pub start: u32, pub end: u32 }


// ASTNode = Untyped node
// as opposed to a TypedNode, which is made by the typechecker
pub struct ASTNode { pub data: Box<dyn NodeBehavior>, pub span: Span }

impl ASTNode {
    pub fn new(data: impl NodeBehavior + 'static, span: Span) -> ASTNode {
        ASTNode{ data: Box::new(data), span }
    }
}

/// behaviour of a single kind of node
pub trait NodeBehavior {
    /// used as the node's label when printing
    fn name(&self) -> &'static str;

    /// child nodes, in evaluation order
    fn children(&self) -> Vec<&ASTNode> { Vec::new() }

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
