

use core::f64::math::powi;

use antlr_rust::tree::ParseTree;
use antlr_rust::token::*;
use antlr_rust::tree::ParseTreeVisitorCompat;
use generated::katovisitor::*;
use generated::katoparser::*;

use crate::ast::*;
use crate::datatype::{ Value, Bounded };
use crate::stdt;

type Result = Box<ASTNode>;

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
        let statements: Vec<ASTNode> = ctx
            .statement_all()
            .iter()
            .map(|statement| *self.visit(statement.as_ref()))
            .collect();
        Result::new(
            ASTNode::Scope(IScope{ statements })
        )
    }

    // skip visit_statement, will just visit child.

    fn visit_varDefinition(&mut self, ctx: &generated::katoparser::VarDefinitionContext<'input>) -> Self::Return {
        let name = String::from(ctx.variable().unwrap().get_text());
        let value = self.visit(ctx.expr().unwrap().as_ref());
        Result::new(
            ASTNode::Define(IDefine{ name, value })
        )
    }

    fn visit_funcDefinition(&mut self, ctx: &FuncDefinitionContext<'input>) -> Self::Return {
        let variables = ctx.variable_all();
        let name = variables[0].as_ref().get_text();
        let args = variables[1..].iter().map(
            |arg| arg.as_ref().get_text()
        ).collect();
        let body_node = *self.visit(ctx.scope().unwrap().as_ref());
        if let ASTNode::Scope( iscope ) = body_node {
            let body = Box::new(iscope);
            let value = Box::new(ASTNode::Function(IFunction { args, body }));
            Result::new(
                ASTNode::Define(IDefine { name, value })
            )
        } else {
            panic!("expected ASTNode::Scope from visiting scope of function {} but got {} instead", name, body_node)
        }
    }

    fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>) -> Self::Return {
        let exprs = ctx.expr_all();
        let called = self.visit(exprs[0].as_ref());
        let args: Vec<ASTNode> = exprs[1..].iter().map(
            |expr| *self.visit(expr.as_ref())
        ).collect();
        Result::new(ASTNode::Call( ICall { called, args } ))
    }

    fn visit_ExprVar(&mut self, ctx: &ExprVarContext<'input>) -> Self::Return {
        let name = ctx.variable().unwrap().get_text();
        Result::new(ASTNode::Get( IGet { name }))
    }

    fn visit_ExprLiteral(&mut self, ctx: &ExprLiteralContext<'input>) -> Self::Return {
        self.visit(ctx.literal().unwrap().as_ref())
    }

    // skip visit expr literal -- just visit the child.

    fn visit_ExprMulDiv(&mut self, ctx: &ExprMulDivContext<'input>) -> Self::Return {
        let op_string = (ctx.op.as_ref().unwrap()).get_text(); // since its a one-length str
        let op = match op_string {
            "*" => ASTOp::Mul,
            "/" => ASTOp::Div,
            other => panic!("Expected * or / as ExprMulDivContext.op text, got {}", other)
        };
        let lhs = self.visit(ctx.lhs.as_ref().unwrap().as_ref());
        let rhs = self.visit(ctx.rhs.as_ref().unwrap().as_ref());
        Result::new(ASTNode::BinaryOp(IBinaryOp { op: op, lhs, rhs }))        
    }

    fn visit_ExprAddSub(&mut self, ctx: &ExprAddSubContext<'input>) -> Self::Return {
        let op_string = (ctx.op.as_ref().unwrap()).get_text(); // since its a one-length str
        let op = match op_string {
            "+" => ASTOp::Add,
            "-" => ASTOp::Sub,
            other => panic!("Expected + or - as ExprAddSubContext.op text, got {}", other)
        };
        let lhs = self.visit(ctx.lhs.as_ref().unwrap().as_ref());
        let rhs = self.visit(ctx.rhs.as_ref().unwrap().as_ref());
        Result::new(ASTNode::BinaryOp(IBinaryOp { op, lhs, rhs }))
    }

    // auto visit_ExprParen

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        let name = ctx.ID().unwrap().get_text();
        Result::new(ASTNode::Get(IGet { name }))
    }

    // skip visit_control, will just visit children

    fn visit_condition(&mut self, ctx: &ConditionContext<'input>) -> Self::Return {
        let case = self.visit(ctx.case.as_ref().unwrap().as_ref());

        let yes_node = self.visit(ctx.yes.as_ref().unwrap().as_ref());
        let yes: Box<IScope>;
        if let ASTNode::Scope( iscope ) = *yes_node {
            yes = Box::new(iscope);
        } else {
            panic!("Expected ASTNode::Scope from visiting ConditionContext.yes")
        }

        let no: Box<IScope>;
        if ctx.no.is_none() {
            no = Box::new(IScope { statements: vec![] });
        } else {
            let no_node = self.visit(ctx.no.as_ref().unwrap().as_ref());
            if let ASTNode::Scope( iscope ) = *no_node {
                no = Box::new(iscope);
            } else {
                panic!("Expected ASTNode::Scope from visiting ConditionContext.no")
            }
        }
        Result::new(ASTNode::Condition(ICondition { case, yes, no }))
    }

    fn visit_exit(&mut self, ctx: &ExitContext<'input>) -> Self::Return {
        Result::new(ASTNode::Exit(IExit {  }))
    }

    fn visit_emit(&mut self, ctx: &EmitContext<'input>) -> Self::Return {
        let value = self.visit(ctx.expr().unwrap().as_ref());
        Result::new(ASTNode::Emit(IEmit { value }))
    }

    // auto visit_literal

    fn visit_LiteralBoolean(&mut self, ctx: &LiteralBooleanContext<'input>) -> Self::Return {
        let text = ctx.get_text();
        let state: bool = text == String::from("true");

        let value = Value::new(state);
        // chain:
        // AST Node -> Value -> Static -> Boolean -> Container of 1 byte -> data -> state
        Result::new(ASTNode::Value(ASTValue::Static(StaticLiteral::Bool(value))))
    }

    fn visit_LiteralInteger(&mut self, ctx: &LiteralIntegerContext<'input>) -> Self::Return {
        let text = ctx.get_text();
        let integer = text.parse::<i64>().unwrap();

        // Int32 when it fits in bounds, Int64 otherwise
        let small = i32::try_from(integer).ok().filter(|n| stdt::Int32::in_bounds(*n));
        let value = match small {
            Some(small) => ASTValue::Static(StaticLiteral::Int32(Value::new(small))),
            None => ASTValue::Static(StaticLiteral::Int64(Value::new(integer)))
        };

        Result::new(ASTNode::Value(value))
    }
}