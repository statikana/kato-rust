pub enum ASTNodeKind {
    Null,
    Scope,
    VarDef,
    FuncDef,
    FuncCall,
        // ExprCall,
    Variable,
        // ExprVar,
    Literal,
        // ExprLiteral,
        // ExprScope,
    ArithMul,
    ArithDiv,
        // ExprMulDiv,
    ArithAdd,
    ArithSub,
        // ExprAddSub,
    Paren,
        // ExprParen,
    IfElseThen,
    Return_,
    Emit, 
}

pub struct ASTNode {
    pub kind: ASTNodeKind,
    pub data: Vec<ASTNode>,
    pub id: Option<String> // used for things which have an identifier in the scope which needs to be known to the executor (like variables, functions, etc.)
}

impl Default for ASTNode {
    // Creates a ASTNode with ASTNodeKind::Null and data: vec![] with no id
    fn default() -> ASTNode {
        ASTNode {
            kind: ASTNodeKind::Null,
            data: vec![],
            id: None
        }
    }
}

