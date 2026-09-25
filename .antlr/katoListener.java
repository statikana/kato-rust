// Generated from /home/ryan/documents/github/kato-rust/kato.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link katoParser}.
 */
public interface katoListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link katoParser#program}.
	 * @param ctx the parse tree
	 */
	void enterProgram(katoParser.ProgramContext ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#program}.
	 * @param ctx the parse tree
	 */
	void exitProgram(katoParser.ProgramContext ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#scope}.
	 * @param ctx the parse tree
	 */
	void enterScope(katoParser.ScopeContext ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#scope}.
	 * @param ctx the parse tree
	 */
	void exitScope(katoParser.ScopeContext ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#statement}.
	 * @param ctx the parse tree
	 */
	void enterStatement(katoParser.StatementContext ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#statement}.
	 * @param ctx the parse tree
	 */
	void exitStatement(katoParser.StatementContext ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#varDefinition}.
	 * @param ctx the parse tree
	 */
	void enterVarDefinition(katoParser.VarDefinitionContext ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#varDefinition}.
	 * @param ctx the parse tree
	 */
	void exitVarDefinition(katoParser.VarDefinitionContext ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#funcDefinition}.
	 * @param ctx the parse tree
	 */
	void enterFuncDefinition(katoParser.FuncDefinitionContext ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#funcDefinition}.
	 * @param ctx the parse tree
	 */
	void exitFuncDefinition(katoParser.FuncDefinitionContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprVar}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprVar(katoParser.ExprVarContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprVar}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprVar(katoParser.ExprVarContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprAddSub}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprAddSub(katoParser.ExprAddSubContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprAddSub}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprAddSub(katoParser.ExprAddSubContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprCall}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprCall(katoParser.ExprCallContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprCall}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprCall(katoParser.ExprCallContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprParen}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprParen(katoParser.ExprParenContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprParen}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprParen(katoParser.ExprParenContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprMulDiv}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprMulDiv(katoParser.ExprMulDivContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprMulDiv}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprMulDiv(katoParser.ExprMulDivContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprLiteral}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprLiteral(katoParser.ExprLiteralContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprLiteral}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprLiteral(katoParser.ExprLiteralContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprScope}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprScope(katoParser.ExprScopeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprScope}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprScope(katoParser.ExprScopeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ExprArray}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterExprArray(katoParser.ExprArrayContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ExprArray}
	 * labeled alternative in {@link katoParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitExprArray(katoParser.ExprArrayContext ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#variable}.
	 * @param ctx the parse tree
	 */
	void enterVariable(katoParser.VariableContext ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#variable}.
	 * @param ctx the parse tree
	 */
	void exitVariable(katoParser.VariableContext ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#control}.
	 * @param ctx the parse tree
	 */
	void enterControl(katoParser.ControlContext ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#control}.
	 * @param ctx the parse tree
	 */
	void exitControl(katoParser.ControlContext ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#if_}.
	 * @param ctx the parse tree
	 */
	void enterIf_(katoParser.If_Context ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#if_}.
	 * @param ctx the parse tree
	 */
	void exitIf_(katoParser.If_Context ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#return_}.
	 * @param ctx the parse tree
	 */
	void enterReturn_(katoParser.Return_Context ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#return_}.
	 * @param ctx the parse tree
	 */
	void exitReturn_(katoParser.Return_Context ctx);
	/**
	 * Enter a parse tree produced by {@link katoParser#emit_}.
	 * @param ctx the parse tree
	 */
	void enterEmit_(katoParser.Emit_Context ctx);
	/**
	 * Exit a parse tree produced by {@link katoParser#emit_}.
	 * @param ctx the parse tree
	 */
	void exitEmit_(katoParser.Emit_Context ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralNumber}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void enterLiteralNumber(katoParser.LiteralNumberContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralNumber}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void exitLiteralNumber(katoParser.LiteralNumberContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralInteger}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void enterLiteralInteger(katoParser.LiteralIntegerContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralInteger}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void exitLiteralInteger(katoParser.LiteralIntegerContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralBoolean}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void enterLiteralBoolean(katoParser.LiteralBooleanContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralBoolean}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void exitLiteralBoolean(katoParser.LiteralBooleanContext ctx);
	/**
	 * Enter a parse tree produced by the {@code LiteralString}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void enterLiteralString(katoParser.LiteralStringContext ctx);
	/**
	 * Exit a parse tree produced by the {@code LiteralString}
	 * labeled alternative in {@link katoParser#literal}.
	 * @param ctx the parse tree
	 */
	void exitLiteralString(katoParser.LiteralStringContext ctx);
}