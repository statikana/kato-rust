#![allow(nonstandard_style)]
// Generated from Kato.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::katoparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link KatoParser}.
 */
pub trait KatoVisitor<'input>: ParseTreeVisitor<'input,KatoParserContextType>{
	/**
	 * Visit a parse tree produced by {@link KatoParser#program}.
	 * @param ctx the parse tree
	 */
	fn visit_program(&mut self, ctx: &ProgramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#scope}.
	 * @param ctx the parse tree
	 */
	fn visit_scope(&mut self, ctx: &ScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#varDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_varDefinition(&mut self, ctx: &VarDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#funcDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_funcDefinition(&mut self, ctx: &FuncDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprVar}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprVar(&mut self, ctx: &ExprVarContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprAddSub}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprAddSub(&mut self, ctx: &ExprAddSubContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprCall}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprParen}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprParen(&mut self, ctx: &ExprParenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprMulDiv}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprMulDiv(&mut self, ctx: &ExprMulDivContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprLiteral}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprLiteral(&mut self, ctx: &ExprLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ExprScope}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ExprScope(&mut self, ctx: &ExprScopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#control}.
	 * @param ctx the parse tree
	 */
	fn visit_control(&mut self, ctx: &ControlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#if_then_else}.
	 * @param ctx the parse tree
	 */
	fn visit_if_then_else(&mut self, ctx: &If_then_elseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#return_}.
	 * @param ctx the parse tree
	 */
	fn visit_return_(&mut self, ctx: &Return_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link KatoParser#emit_}.
	 * @param ctx the parse tree
	 */
	fn visit_emit_(&mut self, ctx: &Emit_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LiteralNumber}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_LiteralNumber(&mut self, ctx: &LiteralNumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LiteralInteger}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_LiteralInteger(&mut self, ctx: &LiteralIntegerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LiteralBoolean}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_LiteralBoolean(&mut self, ctx: &LiteralBooleanContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LiteralString}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_LiteralString(&mut self, ctx: &LiteralStringContext<'input>) { self.visit_children(ctx) }

}

pub trait KatoVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= KatoParserContextType>{
	/**
	 * Visit a parse tree produced by {@link KatoParser#program}.
	 * @param ctx the parse tree
	 */
		fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#scope}.
	 * @param ctx the parse tree
	 */
		fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#varDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_varDefinition(&mut self, ctx: &VarDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#funcDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_funcDefinition(&mut self, ctx: &FuncDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprVar}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprVar(&mut self, ctx: &ExprVarContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprAddSub}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprAddSub(&mut self, ctx: &ExprAddSubContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprCall}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprParen}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprParen(&mut self, ctx: &ExprParenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprMulDiv}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprMulDiv(&mut self, ctx: &ExprMulDivContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprLiteral}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprLiteral(&mut self, ctx: &ExprLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ExprScope}
	 * labeled alternative in {@link KatoParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ExprScope(&mut self, ctx: &ExprScopeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#control}.
	 * @param ctx the parse tree
	 */
		fn visit_control(&mut self, ctx: &ControlContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#if_then_else}.
	 * @param ctx the parse tree
	 */
		fn visit_if_then_else(&mut self, ctx: &If_then_elseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#return_}.
	 * @param ctx the parse tree
	 */
		fn visit_return_(&mut self, ctx: &Return_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link KatoParser#emit_}.
	 * @param ctx the parse tree
	 */
		fn visit_emit_(&mut self, ctx: &Emit_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LiteralNumber}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_LiteralNumber(&mut self, ctx: &LiteralNumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LiteralInteger}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_LiteralInteger(&mut self, ctx: &LiteralIntegerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LiteralBoolean}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_LiteralBoolean(&mut self, ctx: &LiteralBooleanContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LiteralString}
	 * labeled alternative in {@link KatoParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_LiteralString(&mut self, ctx: &LiteralStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> KatoVisitor<'input> for T
where
	T: KatoVisitorCompat<'input>
{
	fn visit_program(&mut self, ctx: &ProgramContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scope(&mut self, ctx: &ScopeContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_scope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_varDefinition(&mut self, ctx: &VarDefinitionContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_varDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_funcDefinition(&mut self, ctx: &FuncDefinitionContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_funcDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprVar(&mut self, ctx: &ExprVarContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_ExprVar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprAddSub(&mut self, ctx: &ExprAddSubContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_ExprAddSub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprCall(&mut self, ctx: &ExprCallContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_ExprCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprParen(&mut self, ctx: &ExprParenContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_ExprParen(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprMulDiv(&mut self, ctx: &ExprMulDivContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_ExprMulDiv(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprLiteral(&mut self, ctx: &ExprLiteralContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_ExprLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ExprScope(&mut self, ctx: &ExprScopeContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_ExprScope(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_control(&mut self, ctx: &ControlContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_control(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_if_then_else(&mut self, ctx: &If_then_elseContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_if_then_else(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_return_(&mut self, ctx: &Return_Context<'input>){
		let result = <Self as KatoVisitorCompat>::visit_return_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emit_(&mut self, ctx: &Emit_Context<'input>){
		let result = <Self as KatoVisitorCompat>::visit_emit_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LiteralNumber(&mut self, ctx: &LiteralNumberContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_LiteralNumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LiteralInteger(&mut self, ctx: &LiteralIntegerContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_LiteralInteger(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LiteralBoolean(&mut self, ctx: &LiteralBooleanContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_LiteralBoolean(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LiteralString(&mut self, ctx: &LiteralStringContext<'input>){
		let result = <Self as KatoVisitorCompat>::visit_LiteralString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}