#![allow(nonstandard_style)]
// Generated from Kato.g4 by ANTLR 4.8
use super::katoparser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link KatoParser}.
 */
pub trait KatoVisitor<'input>: ParseTreeVisitor<'input, KatoParserContextType> {
    /**
     * Visit a parse tree produced by {@link KatoParser#scope}.
     * @param ctx the parse tree
     */
    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code expr_out}
     * labeled alternative in {@link KatoParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_expr_out(&mut self, ctx: &Expr_outContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code expr_term}
     * labeled alternative in {@link KatoParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_expr_term(&mut self, ctx: &Expr_termContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_add_sub}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_add_sub(&mut self, ctx: &Term_add_subContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_literal}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_literal(&mut self, ctx: &Term_literalContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_neg}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_neg(&mut self, ctx: &Term_negContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_mul_div}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_mul_div(&mut self, ctx: &Term_mul_divContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_paren}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_paren(&mut self, ctx: &Term_parenContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code value_boolean}
     * labeled alternative in {@link KatoParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_value_boolean(&mut self, ctx: &Value_booleanContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code value_numeric}
     * labeled alternative in {@link KatoParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_value_numeric(&mut self, ctx: &Value_numericContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code value_matrix}
     * labeled alternative in {@link KatoParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_value_matrix(&mut self, ctx: &Value_matrixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code boolean_true}
     * labeled alternative in {@link KatoParser#boolean}.
     * @param ctx the parse tree
     */
    fn visit_boolean_true(&mut self, ctx: &Boolean_trueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code boolean_false}
     * labeled alternative in {@link KatoParser#boolean}.
     * @param ctx the parse tree
     */
    fn visit_boolean_false(&mut self, ctx: &Boolean_falseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code numeric_integer}
     * labeled alternative in {@link KatoParser#numeric}.
     * @param ctx the parse tree
     */
    fn visit_numeric_integer(&mut self, ctx: &Numeric_integerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code numeric_real}
     * labeled alternative in {@link KatoParser#numeric}.
     * @param ctx the parse tree
     */
    fn visit_numeric_real(&mut self, ctx: &Numeric_realContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link KatoParser#matrix}.
     * @param ctx the parse tree
     */
    fn visit_matrix(&mut self, ctx: &MatrixContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait KatoVisitorCompat<'input>:
    ParseTreeVisitorCompat<'input, Node = KatoParserContextType>
{
    /**
     * Visit a parse tree produced by {@link KatoParser#scope}.
     * @param ctx the parse tree
     */
    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code expr_out}
     * labeled alternative in {@link KatoParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_expr_out(&mut self, ctx: &Expr_outContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code expr_term}
     * labeled alternative in {@link KatoParser#expr}.
     * @param ctx the parse tree
     */
    fn visit_expr_term(&mut self, ctx: &Expr_termContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_add_sub}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_add_sub(&mut self, ctx: &Term_add_subContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_literal}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_literal(&mut self, ctx: &Term_literalContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_neg}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_neg(&mut self, ctx: &Term_negContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_mul_div}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_mul_div(&mut self, ctx: &Term_mul_divContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code term_paren}
     * labeled alternative in {@link KatoParser#term}.
     * @param ctx the parse tree
     */
    fn visit_term_paren(&mut self, ctx: &Term_parenContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code value_boolean}
     * labeled alternative in {@link KatoParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_value_boolean(&mut self, ctx: &Value_booleanContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code value_numeric}
     * labeled alternative in {@link KatoParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_value_numeric(&mut self, ctx: &Value_numericContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code value_matrix}
     * labeled alternative in {@link KatoParser#literal}.
     * @param ctx the parse tree
     */
    fn visit_value_matrix(&mut self, ctx: &Value_matrixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code boolean_true}
     * labeled alternative in {@link KatoParser#boolean}.
     * @param ctx the parse tree
     */
    fn visit_boolean_true(&mut self, ctx: &Boolean_trueContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code boolean_false}
     * labeled alternative in {@link KatoParser#boolean}.
     * @param ctx the parse tree
     */
    fn visit_boolean_false(&mut self, ctx: &Boolean_falseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code numeric_integer}
     * labeled alternative in {@link KatoParser#numeric}.
     * @param ctx the parse tree
     */
    fn visit_numeric_integer(&mut self, ctx: &Numeric_integerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code numeric_real}
     * labeled alternative in {@link KatoParser#numeric}.
     * @param ctx the parse tree
     */
    fn visit_numeric_real(&mut self, ctx: &Numeric_realContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link KatoParser#matrix}.
     * @param ctx the parse tree
     */
    fn visit_matrix(&mut self, ctx: &MatrixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> KatoVisitor<'input> for T
where
    T: KatoVisitorCompat<'input>,
{
    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_scope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expr_out(&mut self, ctx: &Expr_outContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_expr_out(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expr_term(&mut self, ctx: &Expr_termContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_expr_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_term_add_sub(&mut self, ctx: &Term_add_subContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_term_add_sub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_term_literal(&mut self, ctx: &Term_literalContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_term_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_term_neg(&mut self, ctx: &Term_negContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_term_neg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_term_mul_div(&mut self, ctx: &Term_mul_divContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_term_mul_div(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_term_paren(&mut self, ctx: &Term_parenContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_term_paren(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_value_boolean(&mut self, ctx: &Value_booleanContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_value_boolean(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_value_numeric(&mut self, ctx: &Value_numericContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_value_numeric(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_value_matrix(&mut self, ctx: &Value_matrixContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_value_matrix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_boolean_true(&mut self, ctx: &Boolean_trueContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_boolean_true(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_boolean_false(&mut self, ctx: &Boolean_falseContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_boolean_false(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numeric_integer(&mut self, ctx: &Numeric_integerContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_numeric_integer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numeric_real(&mut self, ctx: &Numeric_realContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_numeric_real(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_matrix(&mut self, ctx: &MatrixContext<'input>) {
        let result = <Self as KatoVisitorCompat>::visit_matrix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
