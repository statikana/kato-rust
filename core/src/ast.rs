#[derive(Clone, Copy, Debug)]
pub struct Span { pub start: u32, pub end: u32 }

pub struct Node { pub data: NodeData, pub span: Span }

pub enum NodeData {
    Scope{ statements: Vec<Node> },
    BinaryOperation { op: BinaryOperator, lhs: Box<Node>, rhs: Box<Node> },
    UrnaryOperation {op: UrnaryOperator, subject: Box<Node> },
    Out(Box<Node>),
    Boolean(bool),
    Integer(i128),
    Float(f64),
    Unit()
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator { Add, Sub, Mul, Div }

#[derive(Clone, Copy, Debug)]
pub enum UrnaryOperator { Negate }