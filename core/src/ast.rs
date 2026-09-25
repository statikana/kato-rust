use crate::datatype::Value;
use crate::std::*;


pub enum StaticLiteral {
    Int32(Value<Int32>),
    Int64(Value<Int64>),
}

pub enum ASTNode {
    Null,

    StaticLiteral(Box<StaticLiteral>),

    Scope {
        statements: Vec<ASTNode>,
    },

    VarGet {
        name: String,
    },

    VarDef {
        name: String,
        value: Box<ASTNode>
    },

    FuncDef {
        name: String,
        params: Vec<ASTNode>,
        body: Box<ASTNode>,
    },


    FuncCall {
        name: String,
        args: Vec<ASTNode>,
    },

    BinaryOp {
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },

    IfThenElse {
        condition: Box<ASTNode>,
        then: Box<ASTNode>, 
        else_: Box<ASTNode>
    },

    Return_ {
        value: Option<Box<ASTNode>>
    },

    Emit {
        value: Box<ASTNode>
    }
}