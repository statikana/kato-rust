pub mod binary_operation;
pub mod boolean;
pub mod float;
pub mod integer;
pub mod matrix;
pub mod scope;
pub mod unary_operation;
pub mod unit;

pub use binary_operation::ASTBinaryOperation;
pub use boolean::ASTBoolean;
pub use float::ASTFloat;
pub use integer::ASTInteger;
pub use matrix::ASTMatrix;
pub use scope::ASTScope;
pub use unary_operation::ASTUnaryOperation;
pub use unit::ASTUnit;

use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

// ASTNode = Untyped node
// as opposed to a TypedNode, which is made by the typechecker (see the `typed` crate)
pub struct ASTNode {
    pub kind: NodeKind,
    pub span: Span,
}

impl ASTNode {
    pub fn new(kind: impl Into<NodeKind>, span: Span) -> ASTNode {
        ASTNode {
            kind: kind.into(),
            span,
        }
    }
}

// declares `NodeKind` with one variant per node type, plus a `From` impl for each
// so that `ASTNode::new(ASTInteger(5), span)` works
macro_rules! define_node_kinds {
    ($($variant:ident($node:ty)),* $(,)?) => {
        /// every kind of node. a closed set, so later passes (e.g. the typechecker) can `match` on it exhaustively
        pub enum NodeKind {
            $($variant($node)),*
        }

        impl NodeKind {
            /// the behaviour shared by every node kind
            pub fn behavior(&self) -> &dyn NodeBehavior {
                match self {
                    $(NodeKind::$variant(n) => n),*
                }
            }
        }

        $(
            impl From<$node> for NodeKind {
                fn from(node: $node) -> NodeKind {
                    NodeKind::$variant(node)
                }
            }
        )*
    };
}

// this is where every AST Node is defined so it can get sent -> typing -> clif
//
#[macro_export]
macro_rules! for_each_node_kind {
    ($callback:ident) => {
        $callback! {
            Scope($crate::nodes::ASTScope),
            BinaryOperation($crate::nodes::ASTBinaryOperation),
            UnaryOperation($crate::nodes::ASTUnaryOperation),
            Boolean($crate::nodes::ASTBoolean),
            Integer($crate::nodes::ASTInteger),
            Float($crate::nodes::ASTFloat),
            Unit($crate::nodes::ASTUnit),
            Matrix($crate::nodes::ASTMatrix),
        }
    };
}

for_each_node_kind!(define_node_kinds);

/// behaviour of a single kind of node
pub trait NodeBehavior {
    /// used as the node's label when printing
    fn name(&self) -> &'static str;

    /// child nodes, in evaluation order
    fn children(&self) -> Vec<&ASTNode> {
        Vec::new()
    }

    /// formats this node under `label`. `children` are the (typed or untyped) `children()`, in the same order
    fn fmt_labeled(
        &self,
        label: &str,
        children: &[&dyn fmt::Debug],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut tuple = f.debug_tuple(label);
        for child in children {
            tuple.field(child);
        }
        tuple.finish()
    }
}
