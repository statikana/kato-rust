use std::rc::Rc;

use antlr_rust::token::Token;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::tree::{ParseTree, Tree};
use parse::qrustparser::*;
use parse::qrustvisitor::*;

use crate::nodes::{ASTNode, NodeKind, Span};
use crate::operator::{BinaryOperator, UnaryOperator};

type R = Box<ASTNode>;

fn get_span<'input>(ctx: &impl QrustParserContext<'input>) -> Span {
    Span {
        start: ctx.start().get_start() as u32,
        end: ctx.stop().get_stop() as u32 + 1,
    }
}
// prints the rule being visited, its span, and its source text (debug builds only)
macro_rules! debug_visit {
    ($name:expr, $ctx:expr) => {
        // if cfg!(debug_assertions) {
        //     let span = get_span($ctx);
        //     eprintln!("[visit] {:<20} span={}..{} text={:?}", $name, span.start, span.end, $ctx.get_text());
        // }
    };
}

impl Default for R {
    fn default() -> Self {
        Box::new(ASTNode::new(
            NodeKind::Integer(69),
            Span { start: 0, end: 1 },
        ))
    }
}

pub struct ASTVisitor {
    pub temp_result: R, // stores the result of the most recent visit, instead of having functions do regular returns. used by antlr4rust, not me.
}
impl ASTVisitor {
    fn get<'input>(&mut self, x: &Option<Rc<impl QrustParserContext<'input>>>) -> R {
        self.visit(x.as_ref().unwrap().as_ref())
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for ASTVisitor {
    type Return = R; // each function returns this value
    type Node = QrustParserContextType; // each node is represented as this
    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.temp_result
    }
}

impl<'input> QrustVisitorCompat<'input> for ASTVisitor {
    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
        debug_visit!("visit_scope", ctx);
        let mut results = vec![];
        for expr in ctx.expr_all() {
            let result = *self.visit(expr.as_ref());
            results.push(result);
        }
        R::new(ASTNode::new(
            NodeKind::Scope {
                statements: results,
            },
            get_span(ctx),
        ))
    }

    fn visit_expr_out(&mut self, ctx: &Expr_outContext<'input>) -> Self::Return {
        debug_visit!("visit_expr_out", ctx);
        println!("OUT: '{:?}'", *self.get(&ctx.term()));
        R::new(ASTNode::new(NodeKind::Unit, get_span(ctx)))
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
                "*" => BinaryOperator::Mul,
                "/" => BinaryOperator::Div,
                _ => panic!("Unexpected operator for MUL/DIV"),
            };
            R::new(ASTNode::new(
                NodeKind::BinaryOperation { op, lhs, rhs },
                get_span(ctx),
            ))
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
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Sub,
                _ => panic!("Unexpected operator for ADD/SUB"),
            };
            R::new(ASTNode::new(
                NodeKind::BinaryOperation { op, lhs, rhs },
                get_span(ctx),
            ))
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
        R::new(ASTNode::new(
            NodeKind::UnaryOperation {
                op: UnaryOperator::Negate,
                subject,
            },
            get_span(ctx),
        ))
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
        R::new(ASTNode::new(NodeKind::Boolean(true), get_span(ctx)))
    }

    fn visit_boolean_false(&mut self, ctx: &Boolean_falseContext<'input>) -> Self::Return {
        debug_visit!("visit_boolean_false", ctx);
        R::new(ASTNode::new(NodeKind::Boolean(false), get_span(ctx)))
    }

    fn visit_numeric_integer(&mut self, ctx: &Numeric_integerContext<'input>) -> Self::Return {
        debug_visit!("visit_numeric_integer", ctx);
        type IntT = i128; // uhh seems odd
        R::new(ASTNode::new(
            NodeKind::Integer(ctx.get_text().parse::<IntT>().unwrap()),
            get_span(ctx),
        ))
    }

    fn visit_numeric_real(&mut self, ctx: &Numeric_realContext<'input>) -> Self::Return {
        debug_visit!("visit_numeric_real", ctx);
        type FloatT = f64; // ...
        R::new(ASTNode::new(
            NodeKind::Float(ctx.get_text().parse::<FloatT>().unwrap()),
            get_span(ctx),
        ))
    }

    fn visit_value_matrix(&mut self, ctx: &Value_matrixContext<'input>) -> Self::Return {
        self.get(&ctx.matrix())
    }

    fn visit_matrix(&mut self, ctx: &MatrixContext<'input>) -> Self::Return {
        // the typechecker for matrix handles collapsing matrices
        // R::new
        //
        let elements: Vec<ASTNode> = ctx
            .term_all()
            .iter()
            .map(|elem| *self.visit(elem.as_ref()))
            .collect();
        R::new(ASTNode::new(NodeKind::Matrix { elements }, get_span(ctx)))
    }
}
