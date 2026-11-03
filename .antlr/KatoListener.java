// Generated from /home/ryan/Documents/github/statikana/kato-rust/Kato.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link KatoParser}.
 */
public interface KatoListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link KatoParser#program}.
	 * @param ctx the parse tree
	 */
	void enterProgram(KatoParser.ProgramContext ctx);
	/**
	 * Exit a parse tree produced by {@link KatoParser#program}.
	 * @param ctx the parse tree
	 */
	void exitProgram(KatoParser.ProgramContext ctx);
	/**
	 * Enter a parse tree produced by the {@code expr_out}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExpr_out(KatoParser.Expr_outContext ctx);
	/**
	 * Exit a parse tree produced by the {@code expr_out}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExpr_out(KatoParser.Expr_outContext ctx);
	/**
	 * Enter a parse tree produced by the {@code expr_term}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExpr_term(KatoParser.Expr_termContext ctx);
	/**
	 * Exit a parse tree produced by the {@code expr_term}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExpr_term(KatoParser.Expr_termContext ctx);
	/**
	 * Enter a parse tree produced by the {@code term_add_sub}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void enterTerm_add_sub(KatoParser.Term_add_subContext ctx);
	/**
	 * Exit a parse tree produced by the {@code term_add_sub}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void exitTerm_add_sub(KatoParser.Term_add_subContext ctx);
	/**
	 * Enter a parse tree produced by the {@code term_literal}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void enterTerm_literal(KatoParser.Term_literalContext ctx);
	/**
	 * Exit a parse tree produced by the {@code term_literal}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void exitTerm_literal(KatoParser.Term_literalContext ctx);
	/**
	 * Enter a parse tree produced by the {@code term_neg}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void enterTerm_neg(KatoParser.Term_negContext ctx);
	/**
	 * Exit a parse tree produced by the {@code term_neg}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void exitTerm_neg(KatoParser.Term_negContext ctx);
	/**
	 * Enter a parse tree produced by the {@code term_mul_div}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void enterTerm_mul_div(KatoParser.Term_mul_divContext ctx);
	/**
	 * Exit a parse tree produced by the {@code term_mul_div}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void exitTerm_mul_div(KatoParser.Term_mul_divContext ctx);
	/**
	 * Enter a parse tree produced by the {@code term_paren}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void enterTerm_paren(KatoParser.Term_parenContext ctx);
	/**
	 * Exit a parse tree produced by the {@code term_paren}
	 * labeled alternative in {@link KatoParser#term}.
	 * @param ctx the parse tree
	 */
	void exitTerm_paren(KatoParser.Term_parenContext ctx);
	/**
	 * Enter a parse tree produced by the {@code value_boolean}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	void enterValue_boolean(KatoParser.Value_booleanContext ctx);
	/**
	 * Exit a parse tree produced by the {@code value_boolean}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	void exitValue_boolean(KatoParser.Value_booleanContext ctx);
	/**
	 * Enter a parse tree produced by the {@code value_numeric}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	void enterValue_numeric(KatoParser.Value_numericContext ctx);
	/**
	 * Exit a parse tree produced by the {@code value_numeric}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	void exitValue_numeric(KatoParser.Value_numericContext ctx);
	/**
	 * Enter a parse tree produced by the {@code boolean_true}
	 * labeled alternative in {@link KatoParser#boolean}.
	 * @param ctx the parse tree
	 */
	void enterBoolean_true(KatoParser.Boolean_trueContext ctx);
	/**
	 * Exit a parse tree produced by the {@code boolean_true}
	 * labeled alternative in {@link KatoParser#boolean}.
	 * @param ctx the parse tree
	 */
	void exitBoolean_true(KatoParser.Boolean_trueContext ctx);
	/**
	 * Enter a parse tree produced by the {@code boolean_false}
	 * labeled alternative in {@link KatoParser#boolean}.
	 * @param ctx the parse tree
	 */
	void enterBoolean_false(KatoParser.Boolean_falseContext ctx);
	/**
	 * Exit a parse tree produced by the {@code boolean_false}
	 * labeled alternative in {@link KatoParser#boolean}.
	 * @param ctx the parse tree
	 */
	void exitBoolean_false(KatoParser.Boolean_falseContext ctx);
	/**
	 * Enter a parse tree produced by the {@code numeric_integer}
	 * labeled alternative in {@link KatoParser#numeric}.
	 * @param ctx the parse tree
	 */
	void enterNumeric_integer(KatoParser.Numeric_integerContext ctx);
	/**
	 * Exit a parse tree produced by the {@code numeric_integer}
	 * labeled alternative in {@link KatoParser#numeric}.
	 * @param ctx the parse tree
	 */
	void exitNumeric_integer(KatoParser.Numeric_integerContext ctx);
	/**
	 * Enter a parse tree produced by the {@code numeric_real}
	 * labeled alternative in {@link KatoParser#numeric}.
	 * @param ctx the parse tree
	 */
	void enterNumeric_real(KatoParser.Numeric_realContext ctx);
	/**
	 * Exit a parse tree produced by the {@code numeric_real}
	 * labeled alternative in {@link KatoParser#numeric}.
	 * @param ctx the parse tree
	 */
	void exitNumeric_real(KatoParser.Numeric_realContext ctx);
}