// Generated from Kato.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::katolistener::*;
use super::katovisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const ID:isize=22; 
		pub const NUMBER:isize=23; 
		pub const STRING:isize=24; 
		pub const WS:isize=25; 
		pub const COMMENT:isize=26;
	pub const RULE_program:usize = 0; 
	pub const RULE_scope:usize = 1; 
	pub const RULE_statement:usize = 2; 
	pub const RULE_varDefinition:usize = 3; 
	pub const RULE_funcDefinition:usize = 4; 
	pub const RULE_expr:usize = 5; 
	pub const RULE_variable:usize = 6; 
	pub const RULE_control:usize = 7; 
	pub const RULE_if_then_else:usize = 8; 
	pub const RULE_return_:usize = 9; 
	pub const RULE_emit_:usize = 10; 
	pub const RULE_literal:usize = 11;
	pub const ruleNames: [&'static str; 12] =  [
		"program", "scope", "statement", "varDefinition", "funcDefinition", "expr", 
		"variable", "control", "if_then_else", "return_", "emit_", "literal"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;22] = [
		None, Some("'{'"), Some("'}'"), Some("';'"), Some("'let '"), Some("'='"), 
		Some("'func '"), Some("'('"), Some("','"), Some("')'"), Some("'*'"), Some("'/'"), 
		Some("'+'"), Some("'-'"), Some("'if'"), Some("'else'"), Some("'return'"), 
		Some("'return '"), Some("'emit '"), Some("'.'"), Some("'true'"), Some("'false'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;27]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, Some("ID"), 
		Some("NUMBER"), Some("STRING"), Some("WS"), Some("COMMENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,KatoParserExt<'input>, I, KatoParserContextType , dyn KatoListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type KatoTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, KatoParserContextType , dyn KatoListener<'input> + 'a>;

/// Parser for Kato grammar
pub struct KatoParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				KatoParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> KatoParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> KatoParser<'input, I, DefaultErrorStrategy<'input,KatoParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for KatoParser
pub trait KatoParserContext<'input>:
	for<'x> Listenable<dyn KatoListener<'input> + 'x > + 
	for<'x> Visitable<dyn KatoVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=KatoParserContextType>
{}

antlr_rust::coerce_from!{ 'input : KatoParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn KatoParserContext<'input> + 'input
where
    T: KatoVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn KatoVisitor<'input> + 'x))
    }
}

impl<'input> KatoParserContext<'input> for TerminalNode<'input,KatoParserContextType> {}
impl<'input> KatoParserContext<'input> for ErrorNode<'input,KatoParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn KatoParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn KatoListener<'input> + 'input }

pub struct KatoParserContextType;
antlr_rust::tid!{KatoParserContextType}

impl<'input> ParserNodeType<'input> for KatoParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn KatoParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct KatoParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> KatoParserExt<'input>{
}
antlr_rust::tid! { KatoParserExt<'a> }

impl<'input> TokenAware<'input> for KatoParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for KatoParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for KatoParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Kato.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn KatoParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					5 => KatoParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> KatoParser<'input, I, DefaultErrorStrategy<'input,KatoParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 3)
				}
				1=>{
					recog.precpred(None, 2)
				}
				2=>{
					recog.precpred(None, 7)
				}
			_ => true
		}
	}
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ProgramContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_program(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule scope*/
			recog.base.set_state(24);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scope ----------------
pub type ScopeContextAll<'input> = ScopeContext<'input>;


pub type ScopeContext<'input> = BaseParserRuleContext<'input,ScopeContextExt<'input>>;

#[derive(Clone)]
pub struct ScopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for ScopeContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ScopeContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_scope(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_scope(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_scope(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scope }
}
antlr_rust::tid!{ScopeContextExt<'a>}

impl<'input> ScopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScopeContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<ScopeContextExt<'input>>{

fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ScopeContextAttrs<'input> for ScopeContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scope(&mut self,)
	-> Result<Rc<ScopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_scope);
        let mut _localctx: Rc<ScopeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(26);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			recog.base.set_state(30);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__3) | (1usize << T__5) | (1usize << T__6) | (1usize << T__11) | (1usize << T__12) | (1usize << T__13) | (1usize << T__15) | (1usize << T__16) | (1usize << T__17) | (1usize << T__18) | (1usize << T__19) | (1usize << T__20) | (1usize << ID) | (1usize << NUMBER) | (1usize << STRING) | (1usize << COMMENT))) != 0 {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(27);
				recog.statement()?;

				}
				}
				recog.base.set_state(32);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(33);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn varDefinition(&self) -> Option<Rc<VarDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn funcDefinition(&self) -> Option<Rc<FuncDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn control(&self) -> Option<Rc<ControlContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMENT
/// Returns `None` if there is no child corresponding to token COMMENT
fn COMMENT(&self) -> Option<Rc<TerminalNode<'input,KatoParserContextType>>> where Self:Sized{
	self.get_token(COMMENT, 0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(44);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 | T__3 | T__5 | T__6 | T__11 | T__12 | T__13 | T__15 | T__16 | T__17 |
			 T__18 | T__19 | T__20 | ID | NUMBER | STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					{
					recog.base.set_state(39);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__3 
						=> {
							{
							/*InvokeRule varDefinition*/
							recog.base.set_state(35);
							recog.varDefinition()?;

							}
						}

					 T__5 
						=> {
							{
							/*InvokeRule funcDefinition*/
							recog.base.set_state(36);
							recog.funcDefinition()?;

							}
						}

					 T__0 | T__6 | T__11 | T__12 | T__18 | T__19 | T__20 | ID | NUMBER |
					 STRING 
						=> {
							{
							/*InvokeRule expr*/
							recog.base.set_state(37);
							recog.expr_rec(0)?;

							}
						}

					 T__13 | T__15 | T__16 | T__17 
						=> {
							{
							/*InvokeRule control*/
							recog.base.set_state(38);
							recog.control()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(41);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					}
					}
				}

			 COMMENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(43);
					recog.base.match_token(COMMENT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- varDefinition ----------------
pub type VarDefinitionContextAll<'input> = VarDefinitionContext<'input>;


pub type VarDefinitionContext<'input> = BaseParserRuleContext<'input,VarDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct VarDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for VarDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for VarDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_varDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_varDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for VarDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_varDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_varDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_varDefinition }
}
antlr_rust::tid!{VarDefinitionContextExt<'a>}

impl<'input> VarDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VarDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VarDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VarDefinitionContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<VarDefinitionContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VarDefinitionContextAttrs<'input> for VarDefinitionContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn varDefinition(&mut self,)
	-> Result<Rc<VarDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VarDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_varDefinition);
        let mut _localctx: Rc<VarDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(46);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule variable*/
			recog.base.set_state(47);
			recog.variable()?;

			recog.base.set_state(48);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(49);
			recog.expr_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- funcDefinition ----------------
pub type FuncDefinitionContextAll<'input> = FuncDefinitionContext<'input>;


pub type FuncDefinitionContext<'input> = BaseParserRuleContext<'input,FuncDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct FuncDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for FuncDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for FuncDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_funcDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_funcDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for FuncDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_funcDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for FuncDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcDefinition }
}
antlr_rust::tid!{FuncDefinitionContextExt<'a>}

impl<'input> FuncDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FuncDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FuncDefinitionContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<FuncDefinitionContextExt<'input>>{

fn variable_all(&self) ->  Vec<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variable(&self, i: usize) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FuncDefinitionContextAttrs<'input> for FuncDefinitionContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn funcDefinition(&mut self,)
	-> Result<Rc<FuncDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_funcDefinition);
        let mut _localctx: Rc<FuncDefinitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(51);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			/*InvokeRule variable*/
			recog.base.set_state(52);
			recog.variable()?;

			recog.base.set_state(53);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			recog.base.set_state(65);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ID {
				{
				/*InvokeRule variable*/
				recog.base.set_state(54);
				recog.variable()?;

				recog.base.set_state(59);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(55);
						recog.base.match_token(T__7,&mut recog.err_handler)?;

						/*InvokeRule variable*/
						recog.base.set_state(56);
						recog.variable()?;

						}
						} 
					}
					recog.base.set_state(61);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
				}
				recog.base.set_state(63);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__7 {
					{
					recog.base.set_state(62);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(67);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule scope*/
			recog.base.set_state(68);
			recog.scope()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	ExprVarContext(ExprVarContext<'input>),
	ExprAddSubContext(ExprAddSubContext<'input>),
	ExprCallContext(ExprCallContext<'input>),
	ExprParenContext(ExprParenContext<'input>),
	ExprMulDivContext(ExprMulDivContext<'input>),
	ExprLiteralContext(ExprLiteralContext<'input>),
	ExprScopeContext(ExprScopeContext<'input>),
Error(ExprContext<'input>)
}
antlr_rust::tid!{ExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> KatoParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			ExprVarContext(inner) => inner,
			ExprAddSubContext(inner) => inner,
			ExprCallContext(inner) => inner,
			ExprParenContext(inner) => inner,
			ExprMulDivContext(inner) => inner,
			ExprLiteralContext(inner) => inner,
			ExprScopeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn KatoVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn KatoListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn KatoListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprContext<'input>{
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type ExprVarContext<'input> = BaseParserRuleContext<'input,ExprVarContextExt<'input>>;

pub trait ExprVarContextAttrs<'input>: KatoParserContext<'input>{
	fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprVarContextAttrs<'input> for ExprVarContext<'input>{}

pub struct ExprVarContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprVarContextExt<'a>}

impl<'input> KatoParserContext<'input> for ExprVarContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprVarContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprVar(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_ExprVar(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprVarContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_ExprVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprVarContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprVarContext<'input> {}

impl<'input> ExprVarContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprVarContext(
				BaseParserRuleContext::copy_from(ctx,ExprVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprAddSubContext<'input> = BaseParserRuleContext<'input,ExprAddSubContextExt<'input>>;

pub trait ExprAddSubContextAttrs<'input>: KatoParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ExprAddSubContextAttrs<'input> for ExprAddSubContext<'input>{}

pub struct ExprAddSubContextExt<'input>{
	base:ExprContextExt<'input>,
	pub lhs: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub rhs: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprAddSubContextExt<'a>}

impl<'input> KatoParserContext<'input> for ExprAddSubContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprAddSubContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprAddSub(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_ExprAddSub(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprAddSubContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_ExprAddSub(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprAddSubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprAddSubContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprAddSubContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprAddSubContext<'input> {}

impl<'input> ExprAddSubContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprAddSubContext(
				BaseParserRuleContext::copy_from(ctx,ExprAddSubContextExt{
					op:None, 
        			lhs:None, rhs:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprCallContext<'input> = BaseParserRuleContext<'input,ExprCallContextExt<'input>>;

pub trait ExprCallContextAttrs<'input>: KatoParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ExprCallContextAttrs<'input> for ExprCallContext<'input>{}

pub struct ExprCallContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprCallContextExt<'a>}

impl<'input> KatoParserContext<'input> for ExprCallContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprCallContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprCall(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_ExprCall(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_ExprCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprCallContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprCallContext<'input> {}

impl<'input> ExprCallContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprCallContext(
				BaseParserRuleContext::copy_from(ctx,ExprCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprParenContext<'input> = BaseParserRuleContext<'input,ExprParenContextExt<'input>>;

pub trait ExprParenContextAttrs<'input>: KatoParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprParenContextAttrs<'input> for ExprParenContext<'input>{}

pub struct ExprParenContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprParenContextExt<'a>}

impl<'input> KatoParserContext<'input> for ExprParenContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprParenContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprParen(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_ExprParen(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprParenContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_ExprParen(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprParenContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprParenContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprParenContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprParenContext<'input> {}

impl<'input> ExprParenContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprParenContext(
				BaseParserRuleContext::copy_from(ctx,ExprParenContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprMulDivContext<'input> = BaseParserRuleContext<'input,ExprMulDivContextExt<'input>>;

pub trait ExprMulDivContextAttrs<'input>: KatoParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ExprMulDivContextAttrs<'input> for ExprMulDivContext<'input>{}

pub struct ExprMulDivContextExt<'input>{
	base:ExprContextExt<'input>,
	pub lhs: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub rhs: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprMulDivContextExt<'a>}

impl<'input> KatoParserContext<'input> for ExprMulDivContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprMulDivContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprMulDiv(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_ExprMulDiv(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprMulDivContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_ExprMulDiv(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprMulDivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprMulDivContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprMulDivContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprMulDivContext<'input> {}

impl<'input> ExprMulDivContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprMulDivContext(
				BaseParserRuleContext::copy_from(ctx,ExprMulDivContextExt{
					op:None, 
        			lhs:None, rhs:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprLiteralContext<'input> = BaseParserRuleContext<'input,ExprLiteralContextExt<'input>>;

pub trait ExprLiteralContextAttrs<'input>: KatoParserContext<'input>{
	fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprLiteralContextAttrs<'input> for ExprLiteralContext<'input>{}

pub struct ExprLiteralContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprLiteralContextExt<'a>}

impl<'input> KatoParserContext<'input> for ExprLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprLiteral(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_ExprLiteral(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_ExprLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprLiteralContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprLiteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprLiteralContext<'input> {}

impl<'input> ExprLiteralContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprLiteralContext(
				BaseParserRuleContext::copy_from(ctx,ExprLiteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprScopeContext<'input> = BaseParserRuleContext<'input,ExprScopeContextExt<'input>>;

pub trait ExprScopeContextAttrs<'input>: KatoParserContext<'input>{
	fn scope(&self) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExprScopeContextAttrs<'input> for ExprScopeContext<'input>{}

pub struct ExprScopeContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprScopeContextExt<'a>}

impl<'input> KatoParserContext<'input> for ExprScopeContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ExprScopeContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprScope(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_ExprScope(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ExprScopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_ExprScope(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprScopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ExprScopeContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ExprScopeContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ExprScopeContext<'input> {}

impl<'input> ExprScopeContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ExprScopeContext(
				BaseParserRuleContext::copy_from(ctx,ExprScopeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: isize)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 10, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 10;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(78);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ID 
				=> {
					{
					let mut tmp = ExprVarContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					/*InvokeRule variable*/
					recog.base.set_state(71);
					recog.variable()?;

					}
				}

			 T__11 | T__12 | T__18 | T__19 | T__20 | NUMBER | STRING 
				=> {
					{
					let mut tmp = ExprLiteralContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule literal*/
					recog.base.set_state(72);
					recog.literal()?;

					}
				}

			 T__0 
				=> {
					{
					let mut tmp = ExprScopeContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule scope*/
					recog.base.set_state(73);
					recog.scope()?;

					}
				}

			 T__6 
				=> {
					{
					let mut tmp = ExprParenContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(74);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(75);
					recog.expr_rec(0)?;

					recog.base.set_state(76);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(104);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(102);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprMulDivContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprMulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.lhs = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(80);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(81);
							if let ExprContextAll::ExprMulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__9 || _la==T__10) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ExprMulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(82);
							let tmp = recog.expr_rec(4)?;
							if let ExprContextAll::ExprMulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.rhs = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprAddSubContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ExprAddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.lhs = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(83);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(84);
							if let ExprContextAll::ExprAddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==T__11 || _la==T__12) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ExprAddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(85);
							let tmp = recog.expr_rec(3)?;
							if let ExprContextAll::ExprAddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.rhs = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExprCallContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(86);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(87);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							recog.base.set_state(99);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if ((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__6) | (1usize << T__11) | (1usize << T__12) | (1usize << T__18) | (1usize << T__19) | (1usize << T__20) | (1usize << ID) | (1usize << NUMBER) | (1usize << STRING))) != 0 {
								{
								/*InvokeRule expr*/
								recog.base.set_state(88);
								recog.expr_rec(0)?;

								recog.base.set_state(93);
								recog.err_handler.sync(&mut recog.base)?;
								_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
								while { _alt!=2 && _alt!=INVALID_ALT } {
									if _alt==1 {
										{
										{
										recog.base.set_state(89);
										recog.base.match_token(T__7,&mut recog.err_handler)?;

										/*InvokeRule expr*/
										recog.base.set_state(90);
										recog.expr_rec(0)?;

										}
										} 
									}
									recog.base.set_state(95);
									recog.err_handler.sync(&mut recog.base)?;
									_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
								}
								recog.base.set_state(97);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if _la==T__7 {
									{
									recog.base.set_state(96);
									recog.base.match_token(T__7,&mut recog.err_handler)?;

									}
								}

								}
							}

							recog.base.set_state(101);
							recog.base.match_token(T__8,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(106);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variable(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr_rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,KatoParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(107);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- control ----------------
pub type ControlContextAll<'input> = ControlContext<'input>;


pub type ControlContext<'input> = BaseParserRuleContext<'input,ControlContextExt<'input>>;

#[derive(Clone)]
pub struct ControlContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for ControlContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for ControlContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_control(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_control(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for ControlContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_control(self);
	}
}

impl<'input> CustomRuleContext<'input> for ControlContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_control }
	//fn type_rule_index() -> usize where Self: Sized { RULE_control }
}
antlr_rust::tid!{ControlContextExt<'a>}

impl<'input> ControlContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ControlContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ControlContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ControlContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<ControlContextExt<'input>>{

fn if_then_else(&self) -> Option<Rc<If_then_elseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn return_(&self) -> Option<Rc<Return_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn emit_(&self) -> Option<Rc<Emit_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ControlContextAttrs<'input> for ControlContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn control(&mut self,)
	-> Result<Rc<ControlContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ControlContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_control);
        let mut _localctx: Rc<ControlContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(112);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule if_then_else*/
					recog.base.set_state(109);
					recog.if_then_else()?;

					}
				}

			 T__15 | T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule return_*/
					recog.base.set_state(110);
					recog.return_()?;

					}
				}

			 T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule emit_*/
					recog.base.set_state(111);
					recog.emit_()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- if_then_else ----------------
pub type If_then_elseContextAll<'input> = If_then_elseContext<'input>;


pub type If_then_elseContext<'input> = BaseParserRuleContext<'input,If_then_elseContextExt<'input>>;

#[derive(Clone)]
pub struct If_then_elseContextExt<'input>{
	pub condition: Option<Rc<ExprContextAll<'input>>>,
	pub then: Option<Rc<ScopeContextAll<'input>>>,
	pub else_: Option<Rc<ScopeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for If_then_elseContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for If_then_elseContext<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_if_then_else(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_if_then_else(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for If_then_elseContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_if_then_else(self);
	}
}

impl<'input> CustomRuleContext<'input> for If_then_elseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_then_else }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_then_else }
}
antlr_rust::tid!{If_then_elseContextExt<'a>}

impl<'input> If_then_elseContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<If_then_elseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_then_elseContextExt{
				condition: None, then: None, else_: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait If_then_elseContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<If_then_elseContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scope_all(&self) ->  Vec<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn scope(&self, i: usize) -> Option<Rc<ScopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> If_then_elseContextAttrs<'input> for If_then_elseContext<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn if_then_else(&mut self,)
	-> Result<Rc<If_then_elseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_then_elseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_if_then_else);
        let mut _localctx: Rc<If_then_elseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(114);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

			recog.base.set_state(115);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(116);
			let tmp = recog.expr_rec(0)?;
			 cast_mut::<_,If_then_elseContext >(&mut _localctx).condition = Some(tmp.clone());
			  

			recog.base.set_state(117);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule scope*/
			recog.base.set_state(118);
			let tmp = recog.scope()?;
			 cast_mut::<_,If_then_elseContext >(&mut _localctx).then = Some(tmp.clone());
			  

			recog.base.set_state(121);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__14 {
				{
				recog.base.set_state(119);
				recog.base.match_token(T__14,&mut recog.err_handler)?;

				/*InvokeRule scope*/
				recog.base.set_state(120);
				let tmp = recog.scope()?;
				 cast_mut::<_,If_then_elseContext >(&mut _localctx).else_ = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- return_ ----------------
pub type Return_ContextAll<'input> = Return_Context<'input>;


pub type Return_Context<'input> = BaseParserRuleContext<'input,Return_ContextExt<'input>>;

#[derive(Clone)]
pub struct Return_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for Return_Context<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for Return_Context<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_return_(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_return_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for Return_Context<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_return_(self);
	}
}

impl<'input> CustomRuleContext<'input> for Return_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_return_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_return_ }
}
antlr_rust::tid!{Return_ContextExt<'a>}

impl<'input> Return_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Return_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Return_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Return_ContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<Return_ContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Return_ContextAttrs<'input> for Return_Context<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn return_(&mut self,)
	-> Result<Rc<Return_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Return_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_return_);
        let mut _localctx: Rc<Return_ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(126);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__15 
				=> {
					{
					recog.base.set_state(123);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					}
				}

			 T__16 
				=> {
					{
					{
					recog.base.set_state(124);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(125);
					recog.expr_rec(0)?;

					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- emit_ ----------------
pub type Emit_ContextAll<'input> = Emit_Context<'input>;


pub type Emit_Context<'input> = BaseParserRuleContext<'input,Emit_ContextExt<'input>>;

#[derive(Clone)]
pub struct Emit_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for Emit_Context<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for Emit_Context<'input>{
		fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_emit_(self);
		}
		fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
			listener.exit_emit_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for Emit_Context<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_emit_(self);
	}
}

impl<'input> CustomRuleContext<'input> for Emit_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_emit_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_emit_ }
}
antlr_rust::tid!{Emit_ContextExt<'a>}

impl<'input> Emit_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Emit_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Emit_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Emit_ContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<Emit_ContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Emit_ContextAttrs<'input> for Emit_Context<'input>{}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn emit_(&mut self,)
	-> Result<Rc<Emit_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Emit_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_emit_);
        let mut _localctx: Rc<Emit_ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(128);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(129);
			recog.expr_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal ----------------
#[derive(Debug)]
pub enum LiteralContextAll<'input>{
	LiteralIntegerContext(LiteralIntegerContext<'input>),
	LiteralStringContext(LiteralStringContext<'input>),
	LiteralBooleanContext(LiteralBooleanContext<'input>),
	LiteralNumberContext(LiteralNumberContext<'input>),
Error(LiteralContext<'input>)
}
antlr_rust::tid!{LiteralContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LiteralContextAll<'input>{}

impl<'input> KatoParserContext<'input> for LiteralContextAll<'input>{}

impl<'input> Deref for LiteralContextAll<'input>{
	type Target = dyn LiteralContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LiteralContextAll::*;
		match self{
			LiteralIntegerContext(inner) => inner,
			LiteralStringContext(inner) => inner,
			LiteralBooleanContext(inner) => inner,
			LiteralNumberContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for LiteralContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn KatoVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for LiteralContextAll<'input>{
    fn enter(&self, listener: &mut (dyn KatoListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn KatoListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> KatoParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for LiteralContext<'input>{
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for LiteralContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn KatoParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
		LiteralContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LiteralContextAttrs<'input>: KatoParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{


}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

pub type LiteralIntegerContext<'input> = BaseParserRuleContext<'input,LiteralIntegerContextExt<'input>>;

pub trait LiteralIntegerContextAttrs<'input>: KatoParserContext<'input>{
	/// Retrieves all `TerminalNode`s corresponding to token NUMBER in current rule
	fn NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,KatoParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token NUMBER, starting from 0.
	/// Returns `None` if number of children corresponding to token NUMBER is less or equal than `i`.
	fn NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,KatoParserContextType>>> where Self:Sized{
		self.get_token(NUMBER, i)
	}
}

impl<'input> LiteralIntegerContextAttrs<'input> for LiteralIntegerContext<'input>{}

pub struct LiteralIntegerContextExt<'input>{
	base:LiteralContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LiteralIntegerContextExt<'a>}

impl<'input> KatoParserContext<'input> for LiteralIntegerContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for LiteralIntegerContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LiteralInteger(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_LiteralInteger(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for LiteralIntegerContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_LiteralInteger(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralIntegerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}

impl<'input> Borrow<LiteralContextExt<'input>> for LiteralIntegerContext<'input>{
	fn borrow(&self) -> &LiteralContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LiteralContextExt<'input>> for LiteralIntegerContext<'input>{
	fn borrow_mut(&mut self) -> &mut LiteralContextExt<'input> { &mut self.base }
}

impl<'input> LiteralContextAttrs<'input> for LiteralIntegerContext<'input> {}

impl<'input> LiteralIntegerContextExt<'input>{
	fn new(ctx: &dyn LiteralContextAttrs<'input>) -> Rc<LiteralContextAll<'input>>  {
		Rc::new(
			LiteralContextAll::LiteralIntegerContext(
				BaseParserRuleContext::copy_from(ctx,LiteralIntegerContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LiteralStringContext<'input> = BaseParserRuleContext<'input,LiteralStringContextExt<'input>>;

pub trait LiteralStringContextAttrs<'input>: KatoParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING
	/// Returns `None` if there is no child corresponding to token STRING
	fn STRING(&self) -> Option<Rc<TerminalNode<'input,KatoParserContextType>>> where Self:Sized{
		self.get_token(STRING, 0)
	}
}

impl<'input> LiteralStringContextAttrs<'input> for LiteralStringContext<'input>{}

pub struct LiteralStringContextExt<'input>{
	base:LiteralContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LiteralStringContextExt<'a>}

impl<'input> KatoParserContext<'input> for LiteralStringContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for LiteralStringContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LiteralString(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_LiteralString(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for LiteralStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_LiteralString(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}

impl<'input> Borrow<LiteralContextExt<'input>> for LiteralStringContext<'input>{
	fn borrow(&self) -> &LiteralContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LiteralContextExt<'input>> for LiteralStringContext<'input>{
	fn borrow_mut(&mut self) -> &mut LiteralContextExt<'input> { &mut self.base }
}

impl<'input> LiteralContextAttrs<'input> for LiteralStringContext<'input> {}

impl<'input> LiteralStringContextExt<'input>{
	fn new(ctx: &dyn LiteralContextAttrs<'input>) -> Rc<LiteralContextAll<'input>>  {
		Rc::new(
			LiteralContextAll::LiteralStringContext(
				BaseParserRuleContext::copy_from(ctx,LiteralStringContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LiteralBooleanContext<'input> = BaseParserRuleContext<'input,LiteralBooleanContextExt<'input>>;

pub trait LiteralBooleanContextAttrs<'input>: KatoParserContext<'input>{
}

impl<'input> LiteralBooleanContextAttrs<'input> for LiteralBooleanContext<'input>{}

pub struct LiteralBooleanContextExt<'input>{
	base:LiteralContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LiteralBooleanContextExt<'a>}

impl<'input> KatoParserContext<'input> for LiteralBooleanContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for LiteralBooleanContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LiteralBoolean(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_LiteralBoolean(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for LiteralBooleanContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_LiteralBoolean(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralBooleanContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}

impl<'input> Borrow<LiteralContextExt<'input>> for LiteralBooleanContext<'input>{
	fn borrow(&self) -> &LiteralContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LiteralContextExt<'input>> for LiteralBooleanContext<'input>{
	fn borrow_mut(&mut self) -> &mut LiteralContextExt<'input> { &mut self.base }
}

impl<'input> LiteralContextAttrs<'input> for LiteralBooleanContext<'input> {}

impl<'input> LiteralBooleanContextExt<'input>{
	fn new(ctx: &dyn LiteralContextAttrs<'input>) -> Rc<LiteralContextAll<'input>>  {
		Rc::new(
			LiteralContextAll::LiteralBooleanContext(
				BaseParserRuleContext::copy_from(ctx,LiteralBooleanContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LiteralNumberContext<'input> = BaseParserRuleContext<'input,LiteralNumberContextExt<'input>>;

pub trait LiteralNumberContextAttrs<'input>: KatoParserContext<'input>{
	/// Retrieves all `TerminalNode`s corresponding to token NUMBER in current rule
	fn NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,KatoParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token NUMBER, starting from 0.
	/// Returns `None` if number of children corresponding to token NUMBER is less or equal than `i`.
	fn NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,KatoParserContextType>>> where Self:Sized{
		self.get_token(NUMBER, i)
	}
}

impl<'input> LiteralNumberContextAttrs<'input> for LiteralNumberContext<'input>{}

pub struct LiteralNumberContextExt<'input>{
	base:LiteralContextExt<'input>,
	pub sign: Option<TokenType<'input>>,
	pub left: Option<TokenType<'input>>,
	pub right: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LiteralNumberContextExt<'a>}

impl<'input> KatoParserContext<'input> for LiteralNumberContext<'input>{}

impl<'input,'a> Listenable<dyn KatoListener<'input> + 'a> for LiteralNumberContext<'input>{
	fn enter(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LiteralNumber(self);
	}
	fn exit(&self,listener: &mut (dyn KatoListener<'input> + 'a)) {
		listener.exit_LiteralNumber(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn KatoVisitor<'input> + 'a> for LiteralNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn KatoVisitor<'input> + 'a)) {
		visitor.visit_LiteralNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = KatoParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}

impl<'input> Borrow<LiteralContextExt<'input>> for LiteralNumberContext<'input>{
	fn borrow(&self) -> &LiteralContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LiteralContextExt<'input>> for LiteralNumberContext<'input>{
	fn borrow_mut(&mut self) -> &mut LiteralContextExt<'input> { &mut self.base }
}

impl<'input> LiteralContextAttrs<'input> for LiteralNumberContext<'input> {}

impl<'input> LiteralNumberContextExt<'input>{
	fn new(ctx: &dyn LiteralContextAttrs<'input>) -> Rc<LiteralContextAll<'input>>  {
		Rc::new(
			LiteralContextAll::LiteralNumberContext(
				BaseParserRuleContext::copy_from(ctx,LiteralNumberContextExt{
					sign:None, left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> KatoParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(167);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					let tmp = LiteralNumberContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(132);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__11 || _la==T__12 {
						{
						recog.base.set_state(131);
						if let LiteralContextAll::LiteralNumberContext(ctx) = cast_mut::<_,LiteralContextAll >(&mut _localctx){
						ctx.sign = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
						_la = recog.base.input.la(1);
						if { !(_la==T__11 || _la==T__12) } {
							let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
							if let LiteralContextAll::LiteralNumberContext(ctx) = cast_mut::<_,LiteralContextAll >(&mut _localctx){
							ctx.sign = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(158);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
						1 =>{
							{
							{
							recog.base.set_state(135); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							loop {
								{
								{
								recog.base.set_state(134);
								let tmp = recog.base.match_token(NUMBER,&mut recog.err_handler)?;
								if let LiteralContextAll::LiteralNumberContext(ctx) = cast_mut::<_,LiteralContextAll >(&mut _localctx){
								ctx.left = Some(tmp.clone()); } else {unreachable!("cant cast");}  

								}
								}
								recog.base.set_state(137); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if !(_la==NUMBER) {break}
							}
							recog.base.set_state(139);
							recog.base.match_token(T__18,&mut recog.err_handler)?;

							recog.base.set_state(143);
							recog.err_handler.sync(&mut recog.base)?;
							_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
							while { _alt!=2 && _alt!=INVALID_ALT } {
								if _alt==1 {
									{
									{
									recog.base.set_state(140);
									let tmp = recog.base.match_token(NUMBER,&mut recog.err_handler)?;
									if let LiteralContextAll::LiteralNumberContext(ctx) = cast_mut::<_,LiteralContextAll >(&mut _localctx){
									ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

									}
									} 
								}
								recog.base.set_state(145);
								recog.err_handler.sync(&mut recog.base)?;
								_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
							}
							}
							}
						}
					,
						2 =>{
							{
							recog.base.set_state(149);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while _la==NUMBER {
								{
								{
								recog.base.set_state(146);
								let tmp = recog.base.match_token(NUMBER,&mut recog.err_handler)?;
								if let LiteralContextAll::LiteralNumberContext(ctx) = cast_mut::<_,LiteralContextAll >(&mut _localctx){
								ctx.left = Some(tmp.clone()); } else {unreachable!("cant cast");}  

								}
								}
								recog.base.set_state(151);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							recog.base.set_state(152);
							recog.base.match_token(T__18,&mut recog.err_handler)?;

							recog.base.set_state(154); 
							recog.err_handler.sync(&mut recog.base)?;
							_alt = 1;
							loop {
								match _alt {
								    x if x == 1=>
									{
									{
									recog.base.set_state(153);
									let tmp = recog.base.match_token(NUMBER,&mut recog.err_handler)?;
									if let LiteralContextAll::LiteralNumberContext(ctx) = cast_mut::<_,LiteralContextAll >(&mut _localctx){
									ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

									}
									}

								_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
								}
								recog.base.set_state(156); 
								recog.err_handler.sync(&mut recog.base)?;
								_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
								if _alt==2 || _alt==INVALID_ALT { break }
							}
							}
						}

						_ => {}
					}
					}
				}
			,
				2 =>{
					let tmp = LiteralIntegerContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(161); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							recog.base.set_state(160);
							recog.base.match_token(NUMBER,&mut recog.err_handler)?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(163); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}
			,
				3 =>{
					let tmp = LiteralBooleanContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(165);
					_la = recog.base.input.la(1);
					if { !(_la==T__19 || _la==T__20) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				4 =>{
					let tmp = LiteralStringContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(166);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x1c\u{ac}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x03\x02\x03\x02\
	\x03\x03\x03\x03\x07\x03\x1f\x0a\x03\x0c\x03\x0e\x03\x22\x0b\x03\x03\x03\
	\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\x2a\x0a\x04\x03\x04\x03\
	\x04\x03\x04\x05\x04\x2f\x0a\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x07\x06\x3c\x0a\x06\x0c\
	\x06\x0e\x06\x3f\x0b\x06\x03\x06\x05\x06\x42\x0a\x06\x05\x06\x44\x0a\x06\
	\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\
	\x03\x07\x03\x07\x05\x07\x51\x0a\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\
	\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x07\x07\x5e\x0a\x07\
	\x0c\x07\x0e\x07\x61\x0b\x07\x03\x07\x05\x07\x64\x0a\x07\x05\x07\x66\x0a\
	\x07\x03\x07\x07\x07\x69\x0a\x07\x0c\x07\x0e\x07\x6c\x0b\x07\x03\x08\x03\
	\x08\x03\x09\x03\x09\x03\x09\x05\x09\x73\x0a\x09\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\x7c\x0a\x0a\x03\x0b\x03\x0b\x03\
	\x0b\x05\x0b\u{81}\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x05\x0d\u{87}\
	\x0a\x0d\x03\x0d\x06\x0d\u{8a}\x0a\x0d\x0d\x0d\x0e\x0d\u{8b}\x03\x0d\x03\
	\x0d\x07\x0d\u{90}\x0a\x0d\x0c\x0d\x0e\x0d\u{93}\x0b\x0d\x03\x0d\x07\x0d\
	\u{96}\x0a\x0d\x0c\x0d\x0e\x0d\u{99}\x0b\x0d\x03\x0d\x03\x0d\x06\x0d\u{9d}\
	\x0a\x0d\x0d\x0d\x0e\x0d\u{9e}\x05\x0d\u{a1}\x0a\x0d\x03\x0d\x06\x0d\u{a4}\
	\x0a\x0d\x0d\x0d\x0e\x0d\u{a5}\x03\x0d\x03\x0d\x05\x0d\u{aa}\x0a\x0d\x03\
	\x0d\x02\x03\x0c\x0e\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x02\
	\x05\x03\x02\x0c\x0d\x03\x02\x0e\x0f\x03\x02\x16\x17\x02\u{be}\x02\x1a\x03\
	\x02\x02\x02\x04\x1c\x03\x02\x02\x02\x06\x2e\x03\x02\x02\x02\x08\x30\x03\
	\x02\x02\x02\x0a\x35\x03\x02\x02\x02\x0c\x50\x03\x02\x02\x02\x0e\x6d\x03\
	\x02\x02\x02\x10\x72\x03\x02\x02\x02\x12\x74\x03\x02\x02\x02\x14\u{80}\x03\
	\x02\x02\x02\x16\u{82}\x03\x02\x02\x02\x18\u{a9}\x03\x02\x02\x02\x1a\x1b\
	\x05\x04\x03\x02\x1b\x03\x03\x02\x02\x02\x1c\x20\x07\x03\x02\x02\x1d\x1f\
	\x05\x06\x04\x02\x1e\x1d\x03\x02\x02\x02\x1f\x22\x03\x02\x02\x02\x20\x1e\
	\x03\x02\x02\x02\x20\x21\x03\x02\x02\x02\x21\x23\x03\x02\x02\x02\x22\x20\
	\x03\x02\x02\x02\x23\x24\x07\x04\x02\x02\x24\x05\x03\x02\x02\x02\x25\x2a\
	\x05\x08\x05\x02\x26\x2a\x05\x0a\x06\x02\x27\x2a\x05\x0c\x07\x02\x28\x2a\
	\x05\x10\x09\x02\x29\x25\x03\x02\x02\x02\x29\x26\x03\x02\x02\x02\x29\x27\
	\x03\x02\x02\x02\x29\x28\x03\x02\x02\x02\x2a\x2b\x03\x02\x02\x02\x2b\x2c\
	\x07\x05\x02\x02\x2c\x2f\x03\x02\x02\x02\x2d\x2f\x07\x1c\x02\x02\x2e\x29\
	\x03\x02\x02\x02\x2e\x2d\x03\x02\x02\x02\x2f\x07\x03\x02\x02\x02\x30\x31\
	\x07\x06\x02\x02\x31\x32\x05\x0e\x08\x02\x32\x33\x07\x07\x02\x02\x33\x34\
	\x05\x0c\x07\x02\x34\x09\x03\x02\x02\x02\x35\x36\x07\x08\x02\x02\x36\x37\
	\x05\x0e\x08\x02\x37\x43\x07\x09\x02\x02\x38\x3d\x05\x0e\x08\x02\x39\x3a\
	\x07\x0a\x02\x02\x3a\x3c\x05\x0e\x08\x02\x3b\x39\x03\x02\x02\x02\x3c\x3f\
	\x03\x02\x02\x02\x3d\x3b\x03\x02\x02\x02\x3d\x3e\x03\x02\x02\x02\x3e\x41\
	\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\x40\x42\x07\x0a\x02\x02\x41\x40\
	\x03\x02\x02\x02\x41\x42\x03\x02\x02\x02\x42\x44\x03\x02\x02\x02\x43\x38\
	\x03\x02\x02\x02\x43\x44\x03\x02\x02\x02\x44\x45\x03\x02\x02\x02\x45\x46\
	\x07\x0b\x02\x02\x46\x47\x05\x04\x03\x02\x47\x0b\x03\x02\x02\x02\x48\x49\
	\x08\x07\x01\x02\x49\x51\x05\x0e\x08\x02\x4a\x51\x05\x18\x0d\x02\x4b\x51\
	\x05\x04\x03\x02\x4c\x4d\x07\x09\x02\x02\x4d\x4e\x05\x0c\x07\x02\x4e\x4f\
	\x07\x0b\x02\x02\x4f\x51\x03\x02\x02\x02\x50\x48\x03\x02\x02\x02\x50\x4a\
	\x03\x02\x02\x02\x50\x4b\x03\x02\x02\x02\x50\x4c\x03\x02\x02\x02\x51\x6a\
	\x03\x02\x02\x02\x52\x53\x0c\x05\x02\x02\x53\x54\x09\x02\x02\x02\x54\x69\
	\x05\x0c\x07\x06\x55\x56\x0c\x04\x02\x02\x56\x57\x09\x03\x02\x02\x57\x69\
	\x05\x0c\x07\x05\x58\x59\x0c\x09\x02\x02\x59\x65\x07\x09\x02\x02\x5a\x5f\
	\x05\x0c\x07\x02\x5b\x5c\x07\x0a\x02\x02\x5c\x5e\x05\x0c\x07\x02\x5d\x5b\
	\x03\x02\x02\x02\x5e\x61\x03\x02\x02\x02\x5f\x5d\x03\x02\x02\x02\x5f\x60\
	\x03\x02\x02\x02\x60\x63\x03\x02\x02\x02\x61\x5f\x03\x02\x02\x02\x62\x64\
	\x07\x0a\x02\x02\x63\x62\x03\x02\x02\x02\x63\x64\x03\x02\x02\x02\x64\x66\
	\x03\x02\x02\x02\x65\x5a\x03\x02\x02\x02\x65\x66\x03\x02\x02\x02\x66\x67\
	\x03\x02\x02\x02\x67\x69\x07\x0b\x02\x02\x68\x52\x03\x02\x02\x02\x68\x55\
	\x03\x02\x02\x02\x68\x58\x03\x02\x02\x02\x69\x6c\x03\x02\x02\x02\x6a\x68\
	\x03\x02\x02\x02\x6a\x6b\x03\x02\x02\x02\x6b\x0d\x03\x02\x02\x02\x6c\x6a\
	\x03\x02\x02\x02\x6d\x6e\x07\x18\x02\x02\x6e\x0f\x03\x02\x02\x02\x6f\x73\
	\x05\x12\x0a\x02\x70\x73\x05\x14\x0b\x02\x71\x73\x05\x16\x0c\x02\x72\x6f\
	\x03\x02\x02\x02\x72\x70\x03\x02\x02\x02\x72\x71\x03\x02\x02\x02\x73\x11\
	\x03\x02\x02\x02\x74\x75\x07\x10\x02\x02\x75\x76\x07\x09\x02\x02\x76\x77\
	\x05\x0c\x07\x02\x77\x78\x07\x0b\x02\x02\x78\x7b\x05\x04\x03\x02\x79\x7a\
	\x07\x11\x02\x02\x7a\x7c\x05\x04\x03\x02\x7b\x79\x03\x02\x02\x02\x7b\x7c\
	\x03\x02\x02\x02\x7c\x13\x03\x02\x02\x02\x7d\u{81}\x07\x12\x02\x02\x7e\x7f\
	\x07\x13\x02\x02\x7f\u{81}\x05\x0c\x07\x02\u{80}\x7d\x03\x02\x02\x02\u{80}\
	\x7e\x03\x02\x02\x02\u{81}\x15\x03\x02\x02\x02\u{82}\u{83}\x07\x14\x02\x02\
	\u{83}\u{84}\x05\x0c\x07\x02\u{84}\x17\x03\x02\x02\x02\u{85}\u{87}\x09\x03\
	\x02\x02\u{86}\u{85}\x03\x02\x02\x02\u{86}\u{87}\x03\x02\x02\x02\u{87}\u{a0}\
	\x03\x02\x02\x02\u{88}\u{8a}\x07\x19\x02\x02\u{89}\u{88}\x03\x02\x02\x02\
	\u{8a}\u{8b}\x03\x02\x02\x02\u{8b}\u{89}\x03\x02\x02\x02\u{8b}\u{8c}\x03\
	\x02\x02\x02\u{8c}\u{8d}\x03\x02\x02\x02\u{8d}\u{91}\x07\x15\x02\x02\u{8e}\
	\u{90}\x07\x19\x02\x02\u{8f}\u{8e}\x03\x02\x02\x02\u{90}\u{93}\x03\x02\x02\
	\x02\u{91}\u{8f}\x03\x02\x02\x02\u{91}\u{92}\x03\x02\x02\x02\u{92}\u{a1}\
	\x03\x02\x02\x02\u{93}\u{91}\x03\x02\x02\x02\u{94}\u{96}\x07\x19\x02\x02\
	\u{95}\u{94}\x03\x02\x02\x02\u{96}\u{99}\x03\x02\x02\x02\u{97}\u{95}\x03\
	\x02\x02\x02\u{97}\u{98}\x03\x02\x02\x02\u{98}\u{9a}\x03\x02\x02\x02\u{99}\
	\u{97}\x03\x02\x02\x02\u{9a}\u{9c}\x07\x15\x02\x02\u{9b}\u{9d}\x07\x19\x02\
	\x02\u{9c}\u{9b}\x03\x02\x02\x02\u{9d}\u{9e}\x03\x02\x02\x02\u{9e}\u{9c}\
	\x03\x02\x02\x02\u{9e}\u{9f}\x03\x02\x02\x02\u{9f}\u{a1}\x03\x02\x02\x02\
	\u{a0}\u{89}\x03\x02\x02\x02\u{a0}\u{97}\x03\x02\x02\x02\u{a1}\u{aa}\x03\
	\x02\x02\x02\u{a2}\u{a4}\x07\x19\x02\x02\u{a3}\u{a2}\x03\x02\x02\x02\u{a4}\
	\u{a5}\x03\x02\x02\x02\u{a5}\u{a3}\x03\x02\x02\x02\u{a5}\u{a6}\x03\x02\x02\
	\x02\u{a6}\u{aa}\x03\x02\x02\x02\u{a7}\u{aa}\x09\x04\x02\x02\u{a8}\u{aa}\
	\x07\x1a\x02\x02\u{a9}\u{86}\x03\x02\x02\x02\u{a9}\u{a3}\x03\x02\x02\x02\
	\u{a9}\u{a7}\x03\x02\x02\x02\u{a9}\u{a8}\x03\x02\x02\x02\u{aa}\x19\x03\x02\
	\x02\x02\x19\x20\x29\x2e\x3d\x41\x43\x50\x5f\x63\x65\x68\x6a\x72\x7b\u{80}\
	\u{86}\u{8b}\u{91}\u{97}\u{9e}\u{a0}\u{a5}\u{a9}";

