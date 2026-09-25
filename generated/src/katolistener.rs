#![allow(nonstandard_style)]
// Generated from Kato.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::katoparser::*;

pub trait KatoListener<'input> : ParseTreeListener<'input,KatoParserContextType>{
/**
 * Enter a parse tree produced by {@link KatoParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#scope}.
 * @param ctx the parse tree
 */
fn enter_scope(&mut self, _ctx: &ScopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#scope}.
 * @param ctx the parse tree
 */
fn exit_scope(&mut self, _ctx: &ScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#varDefinition}.
 * @param ctx the parse tree
 */
fn enter_varDefinition(&mut self, _ctx: &VarDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#varDefinition}.
 * @param ctx the parse tree
 */
fn exit_varDefinition(&mut self, _ctx: &VarDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#funcDefinition}.
 * @param ctx the parse tree
 */
fn enter_funcDefinition(&mut self, _ctx: &FuncDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#funcDefinition}.
 * @param ctx the parse tree
 */
fn exit_funcDefinition(&mut self, _ctx: &FuncDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprVar}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprVar(&mut self, _ctx: &ExprVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprVar}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprVar(&mut self, _ctx: &ExprVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprAddSub}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprAddSub(&mut self, _ctx: &ExprAddSubContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprAddSub}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprAddSub(&mut self, _ctx: &ExprAddSubContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprCall}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprCall(&mut self, _ctx: &ExprCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprCall}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprCall(&mut self, _ctx: &ExprCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprParen}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprParen(&mut self, _ctx: &ExprParenContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprParen}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprParen(&mut self, _ctx: &ExprParenContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprMulDiv}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprMulDiv(&mut self, _ctx: &ExprMulDivContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprMulDiv}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprMulDiv(&mut self, _ctx: &ExprMulDivContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprLiteral}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprLiteral(&mut self, _ctx: &ExprLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprLiteral}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprLiteral(&mut self, _ctx: &ExprLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprScope}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprScope(&mut self, _ctx: &ExprScopeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprScope}
 * labeled alternative in {@link KatoParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprScope(&mut self, _ctx: &ExprScopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#control}.
 * @param ctx the parse tree
 */
fn enter_control(&mut self, _ctx: &ControlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#control}.
 * @param ctx the parse tree
 */
fn exit_control(&mut self, _ctx: &ControlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#if_then_else}.
 * @param ctx the parse tree
 */
fn enter_if_then_else(&mut self, _ctx: &If_then_elseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#if_then_else}.
 * @param ctx the parse tree
 */
fn exit_if_then_else(&mut self, _ctx: &If_then_elseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#return_}.
 * @param ctx the parse tree
 */
fn enter_return_(&mut self, _ctx: &Return_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#return_}.
 * @param ctx the parse tree
 */
fn exit_return_(&mut self, _ctx: &Return_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link KatoParser#emit_}.
 * @param ctx the parse tree
 */
fn enter_emit_(&mut self, _ctx: &Emit_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link KatoParser#emit_}.
 * @param ctx the parse tree
 */
fn exit_emit_(&mut self, _ctx: &Emit_Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code LiteralNumber}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn enter_LiteralNumber(&mut self, _ctx: &LiteralNumberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LiteralNumber}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn exit_LiteralNumber(&mut self, _ctx: &LiteralNumberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LiteralInteger}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn enter_LiteralInteger(&mut self, _ctx: &LiteralIntegerContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LiteralInteger}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn exit_LiteralInteger(&mut self, _ctx: &LiteralIntegerContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LiteralBoolean}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn enter_LiteralBoolean(&mut self, _ctx: &LiteralBooleanContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LiteralBoolean}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn exit_LiteralBoolean(&mut self, _ctx: &LiteralBooleanContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LiteralString}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn enter_LiteralString(&mut self, _ctx: &LiteralStringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LiteralString}
 * labeled alternative in {@link KatoParser#literal}.
 * @param ctx the parse tree
 */
fn exit_LiteralString(&mut self, _ctx: &LiteralStringContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : KatoListener<'input> }


