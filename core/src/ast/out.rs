use crate::datatype::Type;
use crate::typecheck::{ TypedNode, TypeError };
use super::{ Node, NodeData, Span };

pub struct Out { pub value: Box<Node> }

impl NodeData for Out {
    fn name(&self) -> &'static str { "Out" }

    fn children(&self) -> Vec<&Node> {
        vec![&self.value]
    }

    // printing doesn't produce a value
    fn typecheck(&self, _children: &[TypedNode], _span: Span) -> Result<Type, TypeError> {
        Ok(Type::Unit)
    }
}
