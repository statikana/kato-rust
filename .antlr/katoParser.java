// Generated from /home/ryan/documents/github/kato-rust/kato.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue"})
public class katoParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		T__0=1, T__1=2, T__2=3, T__3=4, T__4=5, T__5=6, T__6=7, T__7=8, T__8=9, 
		T__9=10, T__10=11, T__11=12, T__12=13, T__13=14, T__14=15, T__15=16, T__16=17, 
		T__17=18, T__18=19, T__19=20, T__20=21, T__21=22, T__22=23, ID=24, NUMBER=25, 
		STRING=26, WS=27, COMMENT=28;
	public static final int
		RULE_program = 0, RULE_scope = 1, RULE_statement = 2, RULE_varDefinition = 3, 
		RULE_funcDefinition = 4, RULE_expr = 5, RULE_variable = 6, RULE_control = 7, 
		RULE_if_ = 8, RULE_return_ = 9, RULE_emit_ = 10, RULE_literal = 11;
	private static String[] makeRuleNames() {
		return new String[] {
			"program", "scope", "statement", "varDefinition", "funcDefinition", "expr", 
			"variable", "control", "if_", "return_", "emit_", "literal"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "'{'", "'}'", "';'", "'let '", "'='", "'func '", "'('", "','", 
			"')'", "'['", "']'", "'*'", "'/'", "'+'", "'-'", "'if'", "'else'", "'return'", 
			"'return '", "'emit '", "'.'", "'true'", "'false'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			"ID", "NUMBER", "STRING", "WS", "COMMENT"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "kato.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public katoParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ProgramContext extends ParserRuleContext {
		public ScopeContext scope() {
			return getRuleContext(ScopeContext.class,0);
		}
		public ProgramContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_program; }
	}

	public final ProgramContext program() throws RecognitionException {
		ProgramContext _localctx = new ProgramContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_program);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(24);
			scope();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ScopeContext extends ParserRuleContext {
		public List<StatementContext> statement() {
			return getRuleContexts(StatementContext.class);
		}
		public StatementContext statement(int i) {
			return getRuleContext(StatementContext.class,i);
		}
		public ScopeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_scope; }
	}

	public final ScopeContext scope() throws RecognitionException {
		ScopeContext _localctx = new ScopeContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_scope);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(26);
			match(T__0);
			setState(30);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 402506962L) != 0)) {
				{
				{
				setState(27);
				statement();
				}
				}
				setState(32);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(33);
			match(T__1);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class StatementContext extends ParserRuleContext {
		public VarDefinitionContext varDefinition() {
			return getRuleContext(VarDefinitionContext.class,0);
		}
		public FuncDefinitionContext funcDefinition() {
			return getRuleContext(FuncDefinitionContext.class,0);
		}
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ControlContext control() {
			return getRuleContext(ControlContext.class,0);
		}
		public TerminalNode COMMENT() { return getToken(katoParser.COMMENT, 0); }
		public StatementContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_statement; }
	}

	public final StatementContext statement() throws RecognitionException {
		StatementContext _localctx = new StatementContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_statement);
		try {
			setState(44);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__0:
			case T__3:
			case T__5:
			case T__6:
			case T__9:
			case T__13:
			case T__14:
			case T__15:
			case T__17:
			case T__18:
			case T__19:
			case T__20:
			case T__21:
			case T__22:
			case ID:
			case NUMBER:
			case STRING:
				enterOuterAlt(_localctx, 1);
				{
				{
				setState(39);
				_errHandler.sync(this);
				switch (_input.LA(1)) {
				case T__3:
					{
					setState(35);
					varDefinition();
					}
					break;
				case T__5:
					{
					setState(36);
					funcDefinition();
					}
					break;
				case T__0:
				case T__6:
				case T__9:
				case T__13:
				case T__14:
				case T__20:
				case T__21:
				case T__22:
				case ID:
				case NUMBER:
				case STRING:
					{
					setState(37);
					expr(0);
					}
					break;
				case T__15:
				case T__17:
				case T__18:
				case T__19:
					{
					setState(38);
					control();
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				setState(41);
				match(T__2);
				}
				}
				break;
			case COMMENT:
				enterOuterAlt(_localctx, 2);
				{
				setState(43);
				match(COMMENT);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class VarDefinitionContext extends ParserRuleContext {
		public VariableContext variable() {
			return getRuleContext(VariableContext.class,0);
		}
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public VarDefinitionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_varDefinition; }
	}

	public final VarDefinitionContext varDefinition() throws RecognitionException {
		VarDefinitionContext _localctx = new VarDefinitionContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_varDefinition);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(46);
			match(T__3);
			setState(47);
			variable();
			setState(48);
			match(T__4);
			setState(49);
			expr(0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class FuncDefinitionContext extends ParserRuleContext {
		public VariableContext variable() {
			return getRuleContext(VariableContext.class,0);
		}
		public ScopeContext scope() {
			return getRuleContext(ScopeContext.class,0);
		}
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public FuncDefinitionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_funcDefinition; }
	}

	public final FuncDefinitionContext funcDefinition() throws RecognitionException {
		FuncDefinitionContext _localctx = new FuncDefinitionContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_funcDefinition);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(51);
			match(T__5);
			setState(52);
			variable();
			setState(53);
			match(T__6);
			setState(65);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if ((((_la) & ~0x3f) == 0 && ((1L << _la) & 132170882L) != 0)) {
				{
				setState(54);
				expr(0);
				setState(59);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,3,_ctx);
				while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
					if ( _alt==1 ) {
						{
						{
						setState(55);
						match(T__7);
						setState(56);
						expr(0);
						}
						} 
					}
					setState(61);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,3,_ctx);
				}
				setState(63);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__7) {
					{
					setState(62);
					match(T__7);
					}
				}

				}
			}

			setState(67);
			match(T__8);
			setState(68);
			scope();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExprContext extends ParserRuleContext {
		public ExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expr; }
	 
		public ExprContext() { }
		public void copyFrom(ExprContext ctx) {
			super.copyFrom(ctx);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprVarContext extends ExprContext {
		public VariableContext variable() {
			return getRuleContext(VariableContext.class,0);
		}
		public ExprVarContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprAddSubContext extends ExprContext {
		public ExprContext lhs;
		public Token op;
		public ExprContext rhs;
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public ExprAddSubContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprCallContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public ExprCallContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprParenContext extends ExprContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ExprParenContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprMulDivContext extends ExprContext {
		public ExprContext lhs;
		public Token op;
		public ExprContext rhs;
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public ExprMulDivContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprLiteralContext extends ExprContext {
		public LiteralContext literal() {
			return getRuleContext(LiteralContext.class,0);
		}
		public ExprLiteralContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprScopeContext extends ExprContext {
		public ScopeContext scope() {
			return getRuleContext(ScopeContext.class,0);
		}
		public ExprScopeContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class ExprArrayContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public ExprArrayContext(ExprContext ctx) { copyFrom(ctx); }
	}

	public final ExprContext expr() throws RecognitionException {
		return expr(0);
	}

	private ExprContext expr(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		ExprContext _localctx = new ExprContext(_ctx, _parentState);
		ExprContext _prevctx = _localctx;
		int _startState = 10;
		enterRecursionRule(_localctx, 10, RULE_expr, _p);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(90);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ID:
				{
				_localctx = new ExprVarContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;

				setState(71);
				variable();
				}
				break;
			case T__13:
			case T__14:
			case T__20:
			case T__21:
			case T__22:
			case NUMBER:
			case STRING:
				{
				_localctx = new ExprLiteralContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(72);
				literal();
				}
				break;
			case T__0:
				{
				_localctx = new ExprScopeContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(73);
				scope();
				}
				break;
			case T__9:
				{
				_localctx = new ExprArrayContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(74);
				match(T__9);
				setState(83);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if ((((_la) & ~0x3f) == 0 && ((1L << _la) & 132170882L) != 0)) {
					{
					setState(75);
					expr(0);
					setState(80);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==T__7) {
						{
						{
						setState(76);
						match(T__7);
						setState(77);
						expr(0);
						}
						}
						setState(82);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					}
				}

				setState(85);
				match(T__10);
				}
				break;
			case T__6:
				{
				_localctx = new ExprParenContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(86);
				match(T__6);
				setState(87);
				expr(0);
				setState(88);
				match(T__8);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(116);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,13,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(114);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,12,_ctx) ) {
					case 1:
						{
						_localctx = new ExprMulDivContext(new ExprContext(_parentctx, _parentState));
						((ExprMulDivContext)_localctx).lhs = _prevctx;
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(92);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(93);
						((ExprMulDivContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==T__11 || _la==T__12) ) {
							((ExprMulDivContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(94);
						((ExprMulDivContext)_localctx).rhs = expr(4);
						}
						break;
					case 2:
						{
						_localctx = new ExprAddSubContext(new ExprContext(_parentctx, _parentState));
						((ExprAddSubContext)_localctx).lhs = _prevctx;
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(95);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(96);
						((ExprAddSubContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==T__13 || _la==T__14) ) {
							((ExprAddSubContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(97);
						((ExprAddSubContext)_localctx).rhs = expr(3);
						}
						break;
					case 3:
						{
						_localctx = new ExprCallContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(98);
						if (!(precpred(_ctx, 8))) throw new FailedPredicateException(this, "precpred(_ctx, 8)");
						setState(99);
						match(T__6);
						setState(111);
						_errHandler.sync(this);
						_la = _input.LA(1);
						if ((((_la) & ~0x3f) == 0 && ((1L << _la) & 132170882L) != 0)) {
							{
							setState(100);
							expr(0);
							setState(105);
							_errHandler.sync(this);
							_alt = getInterpreter().adaptivePredict(_input,9,_ctx);
							while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
								if ( _alt==1 ) {
									{
									{
									setState(101);
									match(T__7);
									setState(102);
									expr(0);
									}
									} 
								}
								setState(107);
								_errHandler.sync(this);
								_alt = getInterpreter().adaptivePredict(_input,9,_ctx);
							}
							setState(109);
							_errHandler.sync(this);
							_la = _input.LA(1);
							if (_la==T__7) {
								{
								setState(108);
								match(T__7);
								}
							}

							}
						}

						setState(113);
						match(T__8);
						}
						break;
					}
					} 
				}
				setState(118);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,13,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class VariableContext extends ParserRuleContext {
		public TerminalNode ID() { return getToken(katoParser.ID, 0); }
		public VariableContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_variable; }
	}

	public final VariableContext variable() throws RecognitionException {
		VariableContext _localctx = new VariableContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_variable);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(119);
			match(ID);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ControlContext extends ParserRuleContext {
		public If_Context if_() {
			return getRuleContext(If_Context.class,0);
		}
		public Return_Context return_() {
			return getRuleContext(Return_Context.class,0);
		}
		public Emit_Context emit_() {
			return getRuleContext(Emit_Context.class,0);
		}
		public ControlContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_control; }
	}

	public final ControlContext control() throws RecognitionException {
		ControlContext _localctx = new ControlContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_control);
		try {
			setState(124);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__15:
				enterOuterAlt(_localctx, 1);
				{
				setState(121);
				if_();
				}
				break;
			case T__17:
			case T__18:
				enterOuterAlt(_localctx, 2);
				{
				setState(122);
				return_();
				}
				break;
			case T__19:
				enterOuterAlt(_localctx, 3);
				{
				setState(123);
				emit_();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class If_Context extends ParserRuleContext {
		public ExprContext condition;
		public ScopeContext then;
		public ScopeContext else_;
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public List<ScopeContext> scope() {
			return getRuleContexts(ScopeContext.class);
		}
		public ScopeContext scope(int i) {
			return getRuleContext(ScopeContext.class,i);
		}
		public If_Context(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_if_; }
	}

	public final If_Context if_() throws RecognitionException {
		If_Context _localctx = new If_Context(_ctx, getState());
		enterRule(_localctx, 16, RULE_if_);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(126);
			match(T__15);
			setState(127);
			match(T__6);
			setState(128);
			((If_Context)_localctx).condition = expr(0);
			setState(129);
			match(T__8);
			setState(130);
			((If_Context)_localctx).then = scope();
			setState(133);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__16) {
				{
				setState(131);
				match(T__16);
				setState(132);
				((If_Context)_localctx).else_ = scope();
				}
			}

			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Return_Context extends ParserRuleContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public Return_Context(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_return_; }
	}

	public final Return_Context return_() throws RecognitionException {
		Return_Context _localctx = new Return_Context(_ctx, getState());
		enterRule(_localctx, 18, RULE_return_);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(138);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__17:
				{
				setState(135);
				match(T__17);
				}
				break;
			case T__18:
				{
				{
				setState(136);
				match(T__18);
				setState(137);
				expr(0);
				}
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class Emit_Context extends ParserRuleContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public Emit_Context(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_emit_; }
	}

	public final Emit_Context emit_() throws RecognitionException {
		Emit_Context _localctx = new Emit_Context(_ctx, getState());
		enterRule(_localctx, 20, RULE_emit_);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(140);
			match(T__19);
			setState(141);
			expr(0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class LiteralContext extends ParserRuleContext {
		public LiteralContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_literal; }
	 
		public LiteralContext() { }
		public void copyFrom(LiteralContext ctx) {
			super.copyFrom(ctx);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LiteralIntegerContext extends LiteralContext {
		public List<TerminalNode> NUMBER() { return getTokens(katoParser.NUMBER); }
		public TerminalNode NUMBER(int i) {
			return getToken(katoParser.NUMBER, i);
		}
		public LiteralIntegerContext(LiteralContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LiteralStringContext extends LiteralContext {
		public TerminalNode STRING() { return getToken(katoParser.STRING, 0); }
		public LiteralStringContext(LiteralContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LiteralBooleanContext extends LiteralContext {
		public LiteralBooleanContext(LiteralContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LiteralNumberContext extends LiteralContext {
		public Token sign;
		public Token left;
		public Token right;
		public List<TerminalNode> NUMBER() { return getTokens(katoParser.NUMBER); }
		public TerminalNode NUMBER(int i) {
			return getToken(katoParser.NUMBER, i);
		}
		public LiteralNumberContext(LiteralContext ctx) { copyFrom(ctx); }
	}

	public final LiteralContext literal() throws RecognitionException {
		LiteralContext _localctx = new LiteralContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_literal);
		int _la;
		try {
			int _alt;
			setState(179);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,24,_ctx) ) {
			case 1:
				_localctx = new LiteralNumberContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(144);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__13 || _la==T__14) {
					{
					setState(143);
					((LiteralNumberContext)_localctx).sign = _input.LT(1);
					_la = _input.LA(1);
					if ( !(_la==T__13 || _la==T__14) ) {
						((LiteralNumberContext)_localctx).sign = (Token)_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					}
				}

				setState(170);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,22,_ctx) ) {
				case 1:
					{
					{
					setState(147); 
					_errHandler.sync(this);
					_la = _input.LA(1);
					do {
						{
						{
						setState(146);
						((LiteralNumberContext)_localctx).left = match(NUMBER);
						}
						}
						setState(149); 
						_errHandler.sync(this);
						_la = _input.LA(1);
					} while ( _la==NUMBER );
					setState(151);
					match(T__20);
					setState(155);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,19,_ctx);
					while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
						if ( _alt==1 ) {
							{
							{
							setState(152);
							((LiteralNumberContext)_localctx).right = match(NUMBER);
							}
							} 
						}
						setState(157);
						_errHandler.sync(this);
						_alt = getInterpreter().adaptivePredict(_input,19,_ctx);
					}
					}
					}
					break;
				case 2:
					{
					setState(161);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==NUMBER) {
						{
						{
						setState(158);
						((LiteralNumberContext)_localctx).left = match(NUMBER);
						}
						}
						setState(163);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(164);
					match(T__20);
					setState(166); 
					_errHandler.sync(this);
					_alt = 1;
					do {
						switch (_alt) {
						case 1:
							{
							{
							setState(165);
							((LiteralNumberContext)_localctx).right = match(NUMBER);
							}
							}
							break;
						default:
							throw new NoViableAltException(this);
						}
						setState(168); 
						_errHandler.sync(this);
						_alt = getInterpreter().adaptivePredict(_input,21,_ctx);
					} while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER );
					}
					break;
				}
				}
				break;
			case 2:
				_localctx = new LiteralIntegerContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(173); 
				_errHandler.sync(this);
				_alt = 1;
				do {
					switch (_alt) {
					case 1:
						{
						{
						setState(172);
						match(NUMBER);
						}
						}
						break;
					default:
						throw new NoViableAltException(this);
					}
					setState(175); 
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,23,_ctx);
				} while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER );
				}
				break;
			case 3:
				_localctx = new LiteralBooleanContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(177);
				_la = _input.LA(1);
				if ( !(_la==T__21 || _la==T__22) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				}
				break;
			case 4:
				_localctx = new LiteralStringContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(178);
				match(STRING);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 5:
			return expr_sempred((ExprContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean expr_sempred(ExprContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 3);
		case 1:
			return precpred(_ctx, 2);
		case 2:
			return precpred(_ctx, 8);
		}
		return true;
	}

	public static final String _serializedATN =
		"\u0004\u0001\u001c\u00b6\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001"+
		"\u0002\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004"+
		"\u0002\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002\u0007\u0007\u0007"+
		"\u0002\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002\u000b\u0007\u000b"+
		"\u0001\u0000\u0001\u0000\u0001\u0001\u0001\u0001\u0005\u0001\u001d\b\u0001"+
		"\n\u0001\f\u0001 \t\u0001\u0001\u0001\u0001\u0001\u0001\u0002\u0001\u0002"+
		"\u0001\u0002\u0001\u0002\u0003\u0002(\b\u0002\u0001\u0002\u0001\u0002"+
		"\u0001\u0002\u0003\u0002-\b\u0002\u0001\u0003\u0001\u0003\u0001\u0003"+
		"\u0001\u0003\u0001\u0003\u0001\u0004\u0001\u0004\u0001\u0004\u0001\u0004"+
		"\u0001\u0004\u0001\u0004\u0005\u0004:\b\u0004\n\u0004\f\u0004=\t\u0004"+
		"\u0001\u0004\u0003\u0004@\b\u0004\u0003\u0004B\b\u0004\u0001\u0004\u0001"+
		"\u0004\u0001\u0004\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0001"+
		"\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0005\u0005O\b\u0005\n\u0005"+
		"\f\u0005R\t\u0005\u0003\u0005T\b\u0005\u0001\u0005\u0001\u0005\u0001\u0005"+
		"\u0001\u0005\u0001\u0005\u0003\u0005[\b\u0005\u0001\u0005\u0001\u0005"+
		"\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005"+
		"\u0001\u0005\u0001\u0005\u0001\u0005\u0005\u0005h\b\u0005\n\u0005\f\u0005"+
		"k\t\u0005\u0001\u0005\u0003\u0005n\b\u0005\u0003\u0005p\b\u0005\u0001"+
		"\u0005\u0005\u0005s\b\u0005\n\u0005\f\u0005v\t\u0005\u0001\u0006\u0001"+
		"\u0006\u0001\u0007\u0001\u0007\u0001\u0007\u0003\u0007}\b\u0007\u0001"+
		"\b\u0001\b\u0001\b\u0001\b\u0001\b\u0001\b\u0001\b\u0003\b\u0086\b\b\u0001"+
		"\t\u0001\t\u0001\t\u0003\t\u008b\b\t\u0001\n\u0001\n\u0001\n\u0001\u000b"+
		"\u0003\u000b\u0091\b\u000b\u0001\u000b\u0004\u000b\u0094\b\u000b\u000b"+
		"\u000b\f\u000b\u0095\u0001\u000b\u0001\u000b\u0005\u000b\u009a\b\u000b"+
		"\n\u000b\f\u000b\u009d\t\u000b\u0001\u000b\u0005\u000b\u00a0\b\u000b\n"+
		"\u000b\f\u000b\u00a3\t\u000b\u0001\u000b\u0001\u000b\u0004\u000b\u00a7"+
		"\b\u000b\u000b\u000b\f\u000b\u00a8\u0003\u000b\u00ab\b\u000b\u0001\u000b"+
		"\u0004\u000b\u00ae\b\u000b\u000b\u000b\f\u000b\u00af\u0001\u000b\u0001"+
		"\u000b\u0003\u000b\u00b4\b\u000b\u0001\u000b\u0000\u0001\n\f\u0000\u0002"+
		"\u0004\u0006\b\n\f\u000e\u0010\u0012\u0014\u0016\u0000\u0003\u0001\u0000"+
		"\f\r\u0001\u0000\u000e\u000f\u0001\u0000\u0016\u0017\u00cb\u0000\u0018"+
		"\u0001\u0000\u0000\u0000\u0002\u001a\u0001\u0000\u0000\u0000\u0004,\u0001"+
		"\u0000\u0000\u0000\u0006.\u0001\u0000\u0000\u0000\b3\u0001\u0000\u0000"+
		"\u0000\nZ\u0001\u0000\u0000\u0000\fw\u0001\u0000\u0000\u0000\u000e|\u0001"+
		"\u0000\u0000\u0000\u0010~\u0001\u0000\u0000\u0000\u0012\u008a\u0001\u0000"+
		"\u0000\u0000\u0014\u008c\u0001\u0000\u0000\u0000\u0016\u00b3\u0001\u0000"+
		"\u0000\u0000\u0018\u0019\u0003\u0002\u0001\u0000\u0019\u0001\u0001\u0000"+
		"\u0000\u0000\u001a\u001e\u0005\u0001\u0000\u0000\u001b\u001d\u0003\u0004"+
		"\u0002\u0000\u001c\u001b\u0001\u0000\u0000\u0000\u001d \u0001\u0000\u0000"+
		"\u0000\u001e\u001c\u0001\u0000\u0000\u0000\u001e\u001f\u0001\u0000\u0000"+
		"\u0000\u001f!\u0001\u0000\u0000\u0000 \u001e\u0001\u0000\u0000\u0000!"+
		"\"\u0005\u0002\u0000\u0000\"\u0003\u0001\u0000\u0000\u0000#(\u0003\u0006"+
		"\u0003\u0000$(\u0003\b\u0004\u0000%(\u0003\n\u0005\u0000&(\u0003\u000e"+
		"\u0007\u0000\'#\u0001\u0000\u0000\u0000\'$\u0001\u0000\u0000\u0000\'%"+
		"\u0001\u0000\u0000\u0000\'&\u0001\u0000\u0000\u0000()\u0001\u0000\u0000"+
		"\u0000)*\u0005\u0003\u0000\u0000*-\u0001\u0000\u0000\u0000+-\u0005\u001c"+
		"\u0000\u0000,\'\u0001\u0000\u0000\u0000,+\u0001\u0000\u0000\u0000-\u0005"+
		"\u0001\u0000\u0000\u0000./\u0005\u0004\u0000\u0000/0\u0003\f\u0006\u0000"+
		"01\u0005\u0005\u0000\u000012\u0003\n\u0005\u00002\u0007\u0001\u0000\u0000"+
		"\u000034\u0005\u0006\u0000\u000045\u0003\f\u0006\u00005A\u0005\u0007\u0000"+
		"\u00006;\u0003\n\u0005\u000078\u0005\b\u0000\u00008:\u0003\n\u0005\u0000"+
		"97\u0001\u0000\u0000\u0000:=\u0001\u0000\u0000\u0000;9\u0001\u0000\u0000"+
		"\u0000;<\u0001\u0000\u0000\u0000<?\u0001\u0000\u0000\u0000=;\u0001\u0000"+
		"\u0000\u0000>@\u0005\b\u0000\u0000?>\u0001\u0000\u0000\u0000?@\u0001\u0000"+
		"\u0000\u0000@B\u0001\u0000\u0000\u0000A6\u0001\u0000\u0000\u0000AB\u0001"+
		"\u0000\u0000\u0000BC\u0001\u0000\u0000\u0000CD\u0005\t\u0000\u0000DE\u0003"+
		"\u0002\u0001\u0000E\t\u0001\u0000\u0000\u0000FG\u0006\u0005\uffff\uffff"+
		"\u0000G[\u0003\f\u0006\u0000H[\u0003\u0016\u000b\u0000I[\u0003\u0002\u0001"+
		"\u0000JS\u0005\n\u0000\u0000KP\u0003\n\u0005\u0000LM\u0005\b\u0000\u0000"+
		"MO\u0003\n\u0005\u0000NL\u0001\u0000\u0000\u0000OR\u0001\u0000\u0000\u0000"+
		"PN\u0001\u0000\u0000\u0000PQ\u0001\u0000\u0000\u0000QT\u0001\u0000\u0000"+
		"\u0000RP\u0001\u0000\u0000\u0000SK\u0001\u0000\u0000\u0000ST\u0001\u0000"+
		"\u0000\u0000TU\u0001\u0000\u0000\u0000U[\u0005\u000b\u0000\u0000VW\u0005"+
		"\u0007\u0000\u0000WX\u0003\n\u0005\u0000XY\u0005\t\u0000\u0000Y[\u0001"+
		"\u0000\u0000\u0000ZF\u0001\u0000\u0000\u0000ZH\u0001\u0000\u0000\u0000"+
		"ZI\u0001\u0000\u0000\u0000ZJ\u0001\u0000\u0000\u0000ZV\u0001\u0000\u0000"+
		"\u0000[t\u0001\u0000\u0000\u0000\\]\n\u0003\u0000\u0000]^\u0007\u0000"+
		"\u0000\u0000^s\u0003\n\u0005\u0004_`\n\u0002\u0000\u0000`a\u0007\u0001"+
		"\u0000\u0000as\u0003\n\u0005\u0003bc\n\b\u0000\u0000co\u0005\u0007\u0000"+
		"\u0000di\u0003\n\u0005\u0000ef\u0005\b\u0000\u0000fh\u0003\n\u0005\u0000"+
		"ge\u0001\u0000\u0000\u0000hk\u0001\u0000\u0000\u0000ig\u0001\u0000\u0000"+
		"\u0000ij\u0001\u0000\u0000\u0000jm\u0001\u0000\u0000\u0000ki\u0001\u0000"+
		"\u0000\u0000ln\u0005\b\u0000\u0000ml\u0001\u0000\u0000\u0000mn\u0001\u0000"+
		"\u0000\u0000np\u0001\u0000\u0000\u0000od\u0001\u0000\u0000\u0000op\u0001"+
		"\u0000\u0000\u0000pq\u0001\u0000\u0000\u0000qs\u0005\t\u0000\u0000r\\"+
		"\u0001\u0000\u0000\u0000r_\u0001\u0000\u0000\u0000rb\u0001\u0000\u0000"+
		"\u0000sv\u0001\u0000\u0000\u0000tr\u0001\u0000\u0000\u0000tu\u0001\u0000"+
		"\u0000\u0000u\u000b\u0001\u0000\u0000\u0000vt\u0001\u0000\u0000\u0000"+
		"wx\u0005\u0018\u0000\u0000x\r\u0001\u0000\u0000\u0000y}\u0003\u0010\b"+
		"\u0000z}\u0003\u0012\t\u0000{}\u0003\u0014\n\u0000|y\u0001\u0000\u0000"+
		"\u0000|z\u0001\u0000\u0000\u0000|{\u0001\u0000\u0000\u0000}\u000f\u0001"+
		"\u0000\u0000\u0000~\u007f\u0005\u0010\u0000\u0000\u007f\u0080\u0005\u0007"+
		"\u0000\u0000\u0080\u0081\u0003\n\u0005\u0000\u0081\u0082\u0005\t\u0000"+
		"\u0000\u0082\u0085\u0003\u0002\u0001\u0000\u0083\u0084\u0005\u0011\u0000"+
		"\u0000\u0084\u0086\u0003\u0002\u0001\u0000\u0085\u0083\u0001\u0000\u0000"+
		"\u0000\u0085\u0086\u0001\u0000\u0000\u0000\u0086\u0011\u0001\u0000\u0000"+
		"\u0000\u0087\u008b\u0005\u0012\u0000\u0000\u0088\u0089\u0005\u0013\u0000"+
		"\u0000\u0089\u008b\u0003\n\u0005\u0000\u008a\u0087\u0001\u0000\u0000\u0000"+
		"\u008a\u0088\u0001\u0000\u0000\u0000\u008b\u0013\u0001\u0000\u0000\u0000"+
		"\u008c\u008d\u0005\u0014\u0000\u0000\u008d\u008e\u0003\n\u0005\u0000\u008e"+
		"\u0015\u0001\u0000\u0000\u0000\u008f\u0091\u0007\u0001\u0000\u0000\u0090"+
		"\u008f\u0001\u0000\u0000\u0000\u0090\u0091\u0001\u0000\u0000\u0000\u0091"+
		"\u00aa\u0001\u0000\u0000\u0000\u0092\u0094\u0005\u0019\u0000\u0000\u0093"+
		"\u0092\u0001\u0000\u0000\u0000\u0094\u0095\u0001\u0000\u0000\u0000\u0095"+
		"\u0093\u0001\u0000\u0000\u0000\u0095\u0096\u0001\u0000\u0000\u0000\u0096"+
		"\u0097\u0001\u0000\u0000\u0000\u0097\u009b\u0005\u0015\u0000\u0000\u0098"+
		"\u009a\u0005\u0019\u0000\u0000\u0099\u0098\u0001\u0000\u0000\u0000\u009a"+
		"\u009d\u0001\u0000\u0000\u0000\u009b\u0099\u0001\u0000\u0000\u0000\u009b"+
		"\u009c\u0001\u0000\u0000\u0000\u009c\u00ab\u0001\u0000\u0000\u0000\u009d"+
		"\u009b\u0001\u0000\u0000\u0000\u009e\u00a0\u0005\u0019\u0000\u0000\u009f"+
		"\u009e\u0001\u0000\u0000\u0000\u00a0\u00a3\u0001\u0000\u0000\u0000\u00a1"+
		"\u009f\u0001\u0000\u0000\u0000\u00a1\u00a2\u0001\u0000\u0000\u0000\u00a2"+
		"\u00a4\u0001\u0000\u0000\u0000\u00a3\u00a1\u0001\u0000\u0000\u0000\u00a4"+
		"\u00a6\u0005\u0015\u0000\u0000\u00a5\u00a7\u0005\u0019\u0000\u0000\u00a6"+
		"\u00a5\u0001\u0000\u0000\u0000\u00a7\u00a8\u0001\u0000\u0000\u0000\u00a8"+
		"\u00a6\u0001\u0000\u0000\u0000\u00a8\u00a9\u0001\u0000\u0000\u0000\u00a9"+
		"\u00ab\u0001\u0000\u0000\u0000\u00aa\u0093\u0001\u0000\u0000\u0000\u00aa"+
		"\u00a1\u0001\u0000\u0000\u0000\u00ab\u00b4\u0001\u0000\u0000\u0000\u00ac"+
		"\u00ae\u0005\u0019\u0000\u0000\u00ad\u00ac\u0001\u0000\u0000\u0000\u00ae"+
		"\u00af\u0001\u0000\u0000\u0000\u00af\u00ad\u0001\u0000\u0000\u0000\u00af"+
		"\u00b0\u0001\u0000\u0000\u0000\u00b0\u00b4\u0001\u0000\u0000\u0000\u00b1"+
		"\u00b4\u0007\u0002\u0000\u0000\u00b2\u00b4\u0005\u001a\u0000\u0000\u00b3"+
		"\u0090\u0001\u0000\u0000\u0000\u00b3\u00ad\u0001\u0000\u0000\u0000\u00b3"+
		"\u00b1\u0001\u0000\u0000\u0000\u00b3\u00b2\u0001\u0000\u0000\u0000\u00b4"+
		"\u0017\u0001\u0000\u0000\u0000\u0019\u001e\',;?APSZimort|\u0085\u008a"+
		"\u0090\u0095\u009b\u00a1\u00a8\u00aa\u00af\u00b3";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}