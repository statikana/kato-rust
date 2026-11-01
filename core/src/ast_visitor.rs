use std::rc::Rc;

use antlr_rust::token::Token;
use antlr_rust::tree::{ Tree, ParseTree };
use antlr_rust::tree::ParseTreeVisitorCompat;
use generated::katovisitor::*;
use generated::katoparser::*;

use crate::ast::BinaryOperator::*;
use crate::ast::UrnaryOperator::*;
use crate::ast::{ Node, Span, Scope, BinaryOperation, UrnaryOperation, Boolean, Integer, Float, Unit };


type R = Box<Node>;


fn get_span<'input>(ctx: &impl KatoParserContext<'input>) -> Span {
    Span { start: ctx.start().get_start() as u32, end: ctx.stop().get_stop() as u32 + 1 }
}
// prints the rule being visited, its span, and its source text (debug builds only)
macro_rules! debug_visit {
    ($name:expr, $ctx:expr) => {
        if cfg!(debug_assertions) {
            let span = get_span($ctx);
            eprintln!("[visit] {:<20} span={}..{} text={:?}", $name, span.start, span.end, $ctx.get_text());
        }
    };
}

impl Default for R {
    fn default() -> Self {
        Box::new(Node::new(Integer(69), Span{ start: 0, end: 1 }))
    }
}

pub struct ASTVisitor {
    pub temp_result: R // stores the result of the most recent visit, instead of having functions do regular returns. used by antlr4rust, not me.
} impl ASTVisitor {
    fn get<'input>(&mut self, x: &Option<Rc<impl KatoParserContext<'input>>>) -> R {
        self.visit(x.as_ref().unwrap().as_ref())
    }

}

impl<'input> ParseTreeVisitorCompat<'input> for ASTVisitor {
    type Return = R; // each function returns this value
    type Node = KatoParserContextType; // each node is represented as this
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.temp_result
    }
}

impl<'input> KatoVisitorCompat<'input> for ASTVisitor {

    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
        debug_visit!("visit_scope", ctx);
        let mut results = vec![];
        for expr in ctx.expr_all() {
            let result = *self.visit(expr.as_ref());
            results.push(result);
        }
        R::new(Node::new(Scope{ statements: results }, get_span(ctx)))
    }

    fn visit_expr_out(&mut self, ctx: &Expr_outContext<'input>) -> Self::Return {
        debug_visit!("visit_expr_out", ctx);
        println!("OUT: '{:?}'", *self.get(&ctx.term()));
        R::new(Node::new(Unit, get_span(ctx)))
    }

    fn visit_expr_term(&mut self, ctx: &Expr_termContext<'input>) -> Self::Return {
        debug_visit!("visit_expr_term", ctx);
        self.get(&ctx.term()) // relinquish
    }

    fn visit_term_literal(&mut self, ctx: &Term_literalContext<'input>) -> Self::Return {
        debug_visit!("visit_term_literal", ctx);
        self.get(&ctx.literal()) // relinquish
    }

    fn visit_term_mul_div(&mut self, ctx: &Term_mul_divContext<'input>) -> Self::Return {
        debug_visit!("visit_term_mul_div", ctx);
        let lhs = self.get(&ctx.lhs);
        let rhs = self.get(&ctx.rhs);


        // try to get child at index 1 (should be the operator)
        // if it succeeds, return the resulting bin operation
        if let Some(op_child) = ctx.get_child(1) {
            let op = match op_child.get_text().as_str() {
                "*" => Mul,
                "/" => Div,
                _ => panic!("Unexpected operator for MUL/DIV")
            };
            R::new(Node::new(BinaryOperation { op, lhs, rhs }, get_span(ctx)))
        } else {
            panic!("Bigggg error")
        }
    }

    fn visit_term_add_sub(&mut self, ctx: &Term_add_subContext<'input>) -> Self::Return {
        debug_visit!("visit_term_add_sub", ctx);
        let lhs = self.get(&ctx.lhs);
        let rhs = self.get(&ctx.rhs);

        if let Some(op_child) = ctx.get_child(1) {
            let op = match op_child.get_text().as_str() {
                "+" => Add,
                "-" => Sub,
                _ => panic!("Unexpected operator for ADD/SUB")
            };
            R::new(Node::new(BinaryOperation { op, lhs, rhs }, get_span(ctx)))
        } else {
            panic!("Bigggg error")
        }
    }

    fn visit_term_paren(&mut self, ctx: &Term_parenContext<'input>) -> Self::Return {
        debug_visit!("visit_term_paren", ctx);
        self.get(&ctx.term()) // relinquish
    }

    fn visit_term_neg(&mut self, ctx: &Term_negContext<'input>) -> Self::Return {
        debug_visit!("visit_term_neg", ctx);
        let subject = self.get(&ctx.term());
        let op = Negate;
        R::new(Node::new(UrnaryOperation { op: Negate, subject: subject }, get_span(ctx)))
    }

    fn visit_value_boolean(&mut self, ctx: &Value_booleanContext<'input>) -> Self::Return {
        debug_visit!("visit_value_boolean", ctx);
        self.get(&ctx.boolean()) // relinqish
    }

    fn visit_value_numeric(&mut self, ctx: &Value_numericContext<'input>) -> Self::Return {
        debug_visit!("visit_value_numeric", ctx);
        self.get(&ctx.numeric()) // relinqish
    }

    fn visit_boolean_true(&mut self, ctx: &Boolean_trueContext<'input>) -> Self::Return {
        debug_visit!("visit_boolean_true", ctx);
        R::new(Node::new(Boolean(true), get_span(ctx)))
    }

    fn visit_boolean_false(&mut self, ctx: &Boolean_falseContext<'input>) -> Self::Return {
        debug_visit!("visit_boolean_false", ctx);
        R::new(Node::new(Boolean(false), get_span(ctx)))
    }

    fn visit_numeric_integer(&mut self, ctx: &Numeric_integerContext<'input>) -> Self::Return {
        debug_visit!("visit_numeric_integer", ctx);
        type IntT = i128; // uhh seems odd
        R::new(Node::new(Integer(ctx.get_text().parse::<IntT>().unwrap()), get_span(ctx)))
    }

    fn visit_numeric_real(&mut self, ctx: &Numeric_realContext<'input>) -> Self::Return {  
        debug_visit!("visit_numeric_real", ctx);
        type FloatT = f64; // ...
        R::new(Node::new(Float(ctx.get_text().parse::<FloatT>().unwrap()), get_span(ctx)))
    }
}
