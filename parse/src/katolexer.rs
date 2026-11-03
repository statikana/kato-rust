// Generated from Kato.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;
use antlr_rust::atn::ATN;
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::char_stream::CharStream;
use antlr_rust::dfa::DFA;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext, cast};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, EmptyContext, EmptyCustomRuleContext};
use antlr_rust::token::*;
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};

use antlr_rust::{Tid, TidAble, TidExt, lazy_static};

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const T__6: isize = 7;
pub const TRUE: isize = 8;
pub const FALSE: isize = 9;
pub const INTEGER: isize = 10;
pub const REAL: isize = 11;
pub const MUL: isize = 12;
pub const DIV: isize = 13;
pub const ADD: isize = 14;
pub const SUB: isize = 15;
pub const PAREN_L: isize = 16;
pub const PAREN_R: isize = 17;
pub const WS: isize = 18;
pub const channelNames: [&'static str; 0 + 2] = ["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];

pub const modeNames: [&'static str; 1] = ["DEFAULT_MODE"];

pub const ruleNames: [&'static str; 18] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "TRUE", "FALSE", "INTEGER", "REAL",
    "MUL", "DIV", "ADD", "SUB", "PAREN_L", "PAREN_R", "WS",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 18] = [
    None,
    Some("'{'"),
    Some("';'"),
    Some("'}'"),
    Some("'out'"),
    Some("'['"),
    Some("','"),
    Some("']'"),
    Some("'true'"),
    Some("'false'"),
    None,
    None,
    Some("'*'"),
    Some("'/'"),
    Some("'+'"),
    Some("'-'"),
    Some("'('"),
    Some("')'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 19] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("TRUE"),
    Some("FALSE"),
    Some("INTEGER"),
    Some("REAL"),
    Some("MUL"),
    Some("DIV"),
    Some("ADD"),
    Some("SUB"),
    Some("PAREN_L"),
    Some("PAREN_R"),
    Some("WS"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

pub type LexerContext<'input> =
    BaseRuleContext<'input, EmptyCustomRuleContext<'input, LocalTokenFactory<'input>>>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a>>::From;

pub struct KatoLexer<'input, Input: CharStream<From<'input>>> {
    base: BaseLexer<'input, KatoLexerActions, Input, LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for KatoLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input: CharStream<From<'input>>> Deref for KatoLexer<'input, Input> {
    type Target = BaseLexer<'input, KatoLexerActions, Input, LocalTokenFactory<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> DerefMut for KatoLexer<'input, Input> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> KatoLexer<'input, Input> {
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "KatoLexer.g4"
    }

    pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
                KatoLexerActions {},
                tf,
            ),
        }
    }
}

impl<'input, Input: CharStream<From<'input>>> KatoLexer<'input, Input>
where
    &'input LocalTokenFactory<'input>: Default,
{
    pub fn new(input: Input) -> Self {
        KatoLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
    }
}

pub struct KatoLexerActions {}

impl KatoLexerActions {}

impl<'input, Input: CharStream<From<'input>>>
    Actions<'input, BaseLexer<'input, KatoLexerActions, Input, LocalTokenFactory<'input>>>
    for KatoLexerActions
{
}

impl<'input, Input: CharStream<From<'input>>> KatoLexer<'input, Input> {}

impl<'input, Input: CharStream<From<'input>>>
    LexerRecog<'input, BaseLexer<'input, KatoLexerActions, Input, LocalTokenFactory<'input>>>
    for KatoLexerActions
{
}
impl<'input> TokenAware<'input> for KatoLexerActions {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, Input: CharStream<From<'input>>> TokenSource<'input> for KatoLexer<'input, Input> {
    type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str = "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x14\x62\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\
		\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\
		\x04\x13\x09\x13\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\x03\x05\
		\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x08\x03\x08\
		\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\
		\x03\x0a\x03\x0a\x03\x0b\x06\x0b\x44\x0a\x0b\x0d\x0b\x0e\x0b\x45\x03\x0c\
		\x06\x0c\x49\x0a\x0c\x0d\x0c\x0e\x0c\x4a\x03\x0c\x03\x0c\x06\x0c\x4f\x0a\
		\x0c\x0d\x0c\x0e\x0c\x50\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\
		\x03\x10\x03\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x13\
		\x03\x13\x02\x02\x14\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\x0f\
		\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\x1d\x10\x1f\x11\x21\
		\x12\x23\x13\x25\x14\x03\x02\x04\x03\x02\x32\x3b\x04\x02\x0b\x0c\x22\x22\
		\x02\x64\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\
		\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\
		\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\x02\x13\x03\x02\
		\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\x03\x02\x02\x02\x02\x19\x03\x02\
		\x02\x02\x02\x1b\x03\x02\x02\x02\x02\x1d\x03\x02\x02\x02\x02\x1f\x03\x02\
		\x02\x02\x02\x21\x03\x02\x02\x02\x02\x23\x03\x02\x02\x02\x02\x25\x03\x02\
		\x02\x02\x03\x27\x03\x02\x02\x02\x05\x29\x03\x02\x02\x02\x07\x2b\x03\x02\
		\x02\x02\x09\x2d\x03\x02\x02\x02\x0b\x31\x03\x02\x02\x02\x0d\x33\x03\x02\
		\x02\x02\x0f\x35\x03\x02\x02\x02\x11\x37\x03\x02\x02\x02\x13\x3c\x03\x02\
		\x02\x02\x15\x43\x03\x02\x02\x02\x17\x48\x03\x02\x02\x02\x19\x52\x03\x02\
		\x02\x02\x1b\x54\x03\x02\x02\x02\x1d\x56\x03\x02\x02\x02\x1f\x58\x03\x02\
		\x02\x02\x21\x5a\x03\x02\x02\x02\x23\x5c\x03\x02\x02\x02\x25\x5e\x03\x02\
		\x02\x02\x27\x28\x07\x7d\x02\x02\x28\x04\x03\x02\x02\x02\x29\x2a\x07\x3d\
		\x02\x02\x2a\x06\x03\x02\x02\x02\x2b\x2c\x07\x7f\x02\x02\x2c\x08\x03\x02\
		\x02\x02\x2d\x2e\x07\x71\x02\x02\x2e\x2f\x07\x77\x02\x02\x2f\x30\x07\x76\
		\x02\x02\x30\x0a\x03\x02\x02\x02\x31\x32\x07\x5d\x02\x02\x32\x0c\x03\x02\
		\x02\x02\x33\x34\x07\x2e\x02\x02\x34\x0e\x03\x02\x02\x02\x35\x36\x07\x5f\
		\x02\x02\x36\x10\x03\x02\x02\x02\x37\x38\x07\x76\x02\x02\x38\x39\x07\x74\
		\x02\x02\x39\x3a\x07\x77\x02\x02\x3a\x3b\x07\x67\x02\x02\x3b\x12\x03\x02\
		\x02\x02\x3c\x3d\x07\x68\x02\x02\x3d\x3e\x07\x63\x02\x02\x3e\x3f\x07\x6e\
		\x02\x02\x3f\x40\x07\x75\x02\x02\x40\x41\x07\x67\x02\x02\x41\x14\x03\x02\
		\x02\x02\x42\x44\x09\x02\x02\x02\x43\x42\x03\x02\x02\x02\x44\x45\x03\x02\
		\x02\x02\x45\x43\x03\x02\x02\x02\x45\x46\x03\x02\x02\x02\x46\x16\x03\x02\
		\x02\x02\x47\x49\x09\x02\x02\x02\x48\x47\x03\x02\x02\x02\x49\x4a\x03\x02\
		\x02\x02\x4a\x48\x03\x02\x02\x02\x4a\x4b\x03\x02\x02\x02\x4b\x4c\x03\x02\
		\x02\x02\x4c\x4e\x07\x30\x02\x02\x4d\x4f\x09\x02\x02\x02\x4e\x4d\x03\x02\
		\x02\x02\x4f\x50\x03\x02\x02\x02\x50\x4e\x03\x02\x02\x02\x50\x51\x03\x02\
		\x02\x02\x51\x18\x03\x02\x02\x02\x52\x53\x07\x2c\x02\x02\x53\x1a\x03\x02\
		\x02\x02\x54\x55\x07\x31\x02\x02\x55\x1c\x03\x02\x02\x02\x56\x57\x07\x2d\
		\x02\x02\x57\x1e\x03\x02\x02\x02\x58\x59\x07\x2f\x02\x02\x59\x20\x03\x02\
		\x02\x02\x5a\x5b\x07\x2a\x02\x02\x5b\x22\x03\x02\x02\x02\x5c\x5d\x07\x2b\
		\x02\x02\x5d\x24\x03\x02\x02\x02\x5e\x5f\x09\x03\x02\x02\x5f\x60\x03\x02\
		\x02\x02\x60\x61\x08\x13\x02\x02\x61\x26\x03\x02\x02\x02\x06\x02\x45\x4a\
		\x50\x03\x08\x02\x02";
