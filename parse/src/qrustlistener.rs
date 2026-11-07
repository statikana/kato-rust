#![allow(nonstandard_style)]
// Generated from Qrust.g4 by ANTLR 4.8
use super::qrustparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait QrustListener<'input>: ParseTreeListener<'input, QrustParserContextType> {
    /**
     * Enter a parse tree produced by {@link QrustParser#scope}.
     * @param ctx the parse tree
     */
    fn enter_scope(&mut self, _ctx: &ScopeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link QrustParser#scope}.
     * @param ctx the parse tree
     */
    fn exit_scope(&mut self, _ctx: &ScopeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code expr_out}
     * labeled alternative in {@link QrustParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_expr_out(&mut self, _ctx: &Expr_outContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code expr_out}
     * labeled alternative in {@link QrustParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_expr_out(&mut self, _ctx: &Expr_outContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code expr_term}
     * labeled alternative in {@link QrustParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_expr_term(&mut self, _ctx: &Expr_termContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code expr_term}
     * labeled alternative in {@link QrustParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_expr_term(&mut self, _ctx: &Expr_termContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code term_add_sub}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn enter_term_add_sub(&mut self, _ctx: &Term_add_subContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code term_add_sub}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn exit_term_add_sub(&mut self, _ctx: &Term_add_subContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code term_literal}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn enter_term_literal(&mut self, _ctx: &Term_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code term_literal}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn exit_term_literal(&mut self, _ctx: &Term_literalContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code term_neg}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn enter_term_neg(&mut self, _ctx: &Term_negContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code term_neg}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn exit_term_neg(&mut self, _ctx: &Term_negContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code term_mul_div}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn enter_term_mul_div(&mut self, _ctx: &Term_mul_divContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code term_mul_div}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn exit_term_mul_div(&mut self, _ctx: &Term_mul_divContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code term_paren}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn enter_term_paren(&mut self, _ctx: &Term_parenContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code term_paren}
     * labeled alternative in {@link QrustParser#term}.
     * @param ctx the parse tree
     */
    fn exit_term_paren(&mut self, _ctx: &Term_parenContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code value_boolean}
     * labeled alternative in {@link QrustParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_value_boolean(&mut self, _ctx: &Value_booleanContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code value_boolean}
     * labeled alternative in {@link QrustParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_value_boolean(&mut self, _ctx: &Value_booleanContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code value_numeric}
     * labeled alternative in {@link QrustParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_value_numeric(&mut self, _ctx: &Value_numericContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code value_numeric}
     * labeled alternative in {@link QrustParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_value_numeric(&mut self, _ctx: &Value_numericContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code value_matrix}
     * labeled alternative in {@link QrustParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_value_matrix(&mut self, _ctx: &Value_matrixContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code value_matrix}
     * labeled alternative in {@link QrustParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_value_matrix(&mut self, _ctx: &Value_matrixContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code boolean_true}
     * labeled alternative in {@link QrustParser#boolean}.
     * @param ctx the parse tree
     */
    fn enter_boolean_true(&mut self, _ctx: &Boolean_trueContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code boolean_true}
     * labeled alternative in {@link QrustParser#boolean}.
     * @param ctx the parse tree
     */
    fn exit_boolean_true(&mut self, _ctx: &Boolean_trueContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code boolean_false}
     * labeled alternative in {@link QrustParser#boolean}.
     * @param ctx the parse tree
     */
    fn enter_boolean_false(&mut self, _ctx: &Boolean_falseContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code boolean_false}
     * labeled alternative in {@link QrustParser#boolean}.
     * @param ctx the parse tree
     */
    fn exit_boolean_false(&mut self, _ctx: &Boolean_falseContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code numeric_integer}
     * labeled alternative in {@link QrustParser#numeric}.
     * @param ctx the parse tree
     */
    fn enter_numeric_integer(&mut self, _ctx: &Numeric_integerContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code numeric_integer}
     * labeled alternative in {@link QrustParser#numeric}.
     * @param ctx the parse tree
     */
    fn exit_numeric_integer(&mut self, _ctx: &Numeric_integerContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code numeric_real}
     * labeled alternative in {@link QrustParser#numeric}.
     * @param ctx the parse tree
     */
    fn enter_numeric_real(&mut self, _ctx: &Numeric_realContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code numeric_real}
     * labeled alternative in {@link QrustParser#numeric}.
     * @param ctx the parse tree
     */
    fn exit_numeric_real(&mut self, _ctx: &Numeric_realContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link QrustParser#matrix}.
     * @param ctx the parse tree
     */
    fn enter_matrix(&mut self, _ctx: &MatrixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link QrustParser#matrix}.
     * @param ctx the parse tree
     */
    fn exit_matrix(&mut self, _ctx: &MatrixContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : QrustListener<'input> }
