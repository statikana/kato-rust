use crate::datatype::Value;
use crate::stdt::*;
use crate::mem::*;

pub enum StaticLiteral {
    Bool(Value<Bool>),
    Int32(Value<Int32>),
    Int64(Value<Int64>),
    Float32(Value<Float32>),
    Float64(Value<Float64>)
}


pub enum ASTValue {
    Static(StaticLiteral),
    Dyn(Handle)
}


pub enum ASTOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

// Instructions:
pub struct IScope { pub statements: Vec<ASTNode> }
pub struct IGet { pub name: String }
pub struct IDefine { pub name: String, pub value: Box<ASTNode> }
pub struct IFunction { pub args: Vec<String>, pub body: Box<IScope> }
pub struct ICall { pub called: Box<ASTNode>, pub args: Vec<ASTNode> }
pub struct IBinaryOp { pub op: ASTOp, pub lhs: Box<ASTNode>, pub rhs: Box<ASTNode> }
pub struct ICondition { pub case: Box<ASTNode>, pub yes: Box<IScope>, pub no: Box<IScope> }
pub struct IExit {}
pub struct IEmit { pub value: Box<ASTNode> }



pub enum ASTNode {
    // Values, make room for this in memory
    Null,
    Value(ASTValue),
    Function(IFunction),

    // Organization, change the way the code runs
    Condition(ICondition),
    Exit(IExit),
    Emit(IEmit),
    Scope(IScope),
    // Instructions, do this to the memory
    Get(IGet),
    Define(IDefine),
    Call(ICall),
    BinaryOp(IBinaryOp),
}

impl Default for ASTNode {
    fn default() -> Self { ASTNode::Null{} }
}

impl std::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}", std::any::type_name::<ASTNode>());
        Result::Ok(())
    }
}