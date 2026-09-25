

use antlr_rust::tree::ParseTree;
use antlr_rust::token::*;
use antlr_rust::tree::ParseTreeVisitorCompat;
use generated::katovisitor::*;
use generated::katoparser::*;

use crate::ast::{self, ASTNode};

type Result = Box<ast::ASTNode>;

pub struct ASTVisitor {
    pub temp_result: Result // stores the result of the most recent visit, instead of having functions do regular returns. used by antlr4rust, not me.
}

impl<'input> ParseTreeVisitorCompat<'input> for ASTVisitor {
    type Return = Result; // each function returns this value
    type Node = KatoParserContextType; // each node is represented as this
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.temp_result
    }
}

// We use the KatoVisitorCompat trait instead of KatoVisitor so we can return values from functions
impl<'input> KatoVisitorCompat<'input> for ASTVisitor {

    fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
        let result = self.visit(ctx.scope().unwrap().as_ref());
        Result::new(*result)
    }    

    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
        let resolved_statements: Vec<ast::ASTNode> = ctx
            .statement_all()
            .iter()
            .map(|statement| *self.visit(statement.as_ref()))
            .collect();
        Result::new(ast::ASTNode{
            kind: ast::ASTNodeKind::Scope,
            data: resolved_statements,
            id: None
        })
    }

    // skip visit_statement, will just visit child.

    fn visit_varDefinition(&mut self, ctx: &generated::katoparser::VarDefinitionContext<'input>) -> Self::Return {
        let expr_binding  = ctx.expr().unwrap();
        let expr_id = String::from(ctx.variable().unwrap().get_text());
        Result::new(ast::ASTNode {
            kind: ast::ASTNodeKind::VarDef,
            data: vec![*self.visit(expr_binding.as_ref())],
            id: Some(expr_id)
        })
    }

    fn visit_funcDefinition(&mut self, ctx: &FuncDefinitionContext<'input>) -> Self::Return {
        let variables = ctx.variable_all();

        let func_id = self.visit(variables[0].as_ref()).id.unwrap();

        let mut data = vec![*self.visit(ctx.scope().unwrap().as_ref())];
        data.extend(
            variables[1..].iter().map(|var_ctx| *self.visit(var_ctx.as_ref()))
        );
        
        Result::new(ASTNode { kind: ast::ASTNodeKind::FuncDef, data, id: Some(func_id) })
    }

    fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>) -> Self::Return {
        let exprs = ctx.expr_all();
        let data = exprs.iter().map(
            |expr_ctx| *self.visit(expr_ctx.as_ref())
        ).collect();

        Result::new(ASTNode { kind: ast::ASTNodeKind::FuncCall, data, id: None })
    }

    fn visit_ExprVar(&mut self, ctx: &ExprVarContext<'input>) -> Self::Return {
        let var = *self.visit(ctx.variable().unwrap().as_ref());

        Result::new(ASTNode { kind: ast::ASTNodeKind::Variable, data: vec![], id: Some(String::from(var.id.unwrap()))})
    }

    fn visit_ExprLiteral(&mut self, ctx: &ExprLiteralContext<'input>) -> Self::Return {
        self.visit(ctx.literal().unwrap().as_ref())
    }

    fn visit_ExprScope(&mut self, ctx: &ExprScopeContext<'input>) -> Self::Return {
        self.visit(ctx.scope().unwrap().as_ref())
    }

    fn visit_ExprMulDiv(&mut self, ctx: &ExprMulDivContext<'input>) -> Self::Return {
        let op = (ctx.op.as_ref().unwrap()).get_text().chars().next().unwrap(); // since its a one-length str
        let lhs = *self.visit(ctx.lhs.as_ref().unwrap().as_ref());
        let rhs = *self.visit(ctx.rhs.as_ref().unwrap().as_ref());
        let data = vec![lhs, rhs];

        let kind = match op {
            '*' => ast::ASTNodeKind::ArithMul,
            '/' => ast::ASTNodeKind::ArithDiv,
            _ => panic!("Unexpected operator")
        };
        Result::new(ASTNode { kind, data, id: None })
    }

    fn visit_ExprAddSub(&mut self, ctx: &ExprAddSubContext<'input>) -> Self::Return {
        let op = (ctx.op.as_ref().unwrap()).get_text().chars().next().unwrap(); // since its a one-length str
        let lhs = *self.visit(ctx.lhs.as_ref().unwrap().as_ref());
        let rhs = *self.visit(ctx.rhs.as_ref().unwrap().as_ref());
        let data = vec![lhs, rhs];

        let kind = match op {
            '+' => ast::ASTNodeKind::ArithAdd,
            '-' => ast::ASTNodeKind::ArithSub,
            _ => panic!("Unexpected operator")
        };
        Result::new(ASTNode { kind, data, id: None })
    }

    fn visit_ExprParen(&mut self, ctx: &ExprParenContext<'input>) -> Self::Return {
        self.visit(ctx.expr().unwrap().as_ref())
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        Result::new( ASTNode { kind: ast::ASTNodeKind::Variable, data: vec![], id: Some(id) })
    }

    // skip visit_control, will just visit children

    fn visit_if_then_else(&mut self, ctx: &If_then_elseContext<'input>) -> Self::Return {
        let condition = *self.visit(ctx.condition.as_ref().unwrap().as_ref());
        let then = *self.visit(ctx.then.as_ref().unwrap().as_ref()); // scope
        
        let else_: ASTNode;
        if ctx.else_.is_some() {
            else_ = *self.visit(ctx.else_.as_ref().unwrap().as_ref()); // scope
        } else {
            else_ = ASTNode { kind: ast::ASTNodeKind::Scope, data: vec![], id: None };
        }
        Result::new( ASTNode { kind: ast::ASTNodeKind::IfElseThen, data: vec![condition, then, else_], id: None })
    }

    fn visit_return_(&mut self, ctx: &Return_Context<'input>) -> Self::Return {
        let value: ASTNode;
        if ctx.expr().is_some() {
            value = *self.visit(ctx.expr().unwrap().as_ref());
        } else {
            value = ASTNode::default(); // null
        }
        Result::new( ASTNode { kind: ast::ASTNodeKind::Return_, data: vec![value], id: None } )
    }

    fn visit_emit_(&mut self, ctx: &Emit_Context<'input>) -> Self::Return {
        let value  = *self.visit(ctx.expr().unwrap().as_ref());
        Result::new( ASTNode { kind: ast::ASTNodeKind::Emit, data: vec![value], id: None } )
    }

    // auto visit_literal

    fn visit_LiteralBoolean(&mut self, ctx: &LiteralBooleanContext<'input>) -> Self::Return {
        let text = ctx.get_text();
        let value: bool = text == String::from("true");
        Result::new( ASTNode { kind: ast::ASTNodeKind::Literal, data: (), id: () })
    }
}