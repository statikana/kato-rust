// Generated from /home/ryan/documents/github/kato-rust/Kato.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue"})
public class KatoParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		T__0=1, T__1=2, T__2=3, T__3=4, T__4=5, T__5=6, T__6=7, T__7=8, T__8=9, 
		T__9=10, T__10=11, T__11=12, T__12=13, T__13=14, T__14=15, T__15=16, T__16=17, 
		T__17=18, T__18=19, T__19=20, T__20=21, ID=22, NUMBER=23, STRING=24, WS=25, 
		COMMENT=26;
	public static final int
		RULE_program = 0, RULE_scope = 1, RULE_statement = 2, RULE_varDefinition = 3, 
		RULE_funcDefinition = 4, RULE_expr = 5, RULE_variable = 6, RULE_control = 7, 
		RULE_if_then_else = 8, RULE_return_ = 9, RULE_emit_ = 10, RULE_literal = 11;
	private static String[] makeRuleNames() {
		return new String[] {
			"program", "scope", "statement", "varDefinition", "funcDefinition", "expr", 
			"variable", "control", "if_then_else", "return_", "emit_", "literal"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "'{'", "'}'", "';'", "'let '", "'='", "'func '", "'('", "','", 
			"')'", "'*'", "'/'", "'+'", "'-'", "'if'", "'else'", "'return'", "'return '", 
			"'emit '", "'.'", "'true'", "'false'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, "ID", "NUMBER", 
			"STRING", "WS", "COMMENT"
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
	public String getGrammarFileName() { return "Kato.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public KatoParser(TokenStream input) {
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
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & 100626642L) != 0)) {
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
		public TerminalNode COMMENT() { return getToken(KatoParser.COMMENT, 0); }
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
			case T__11:
			case T__12:
			case T__13:
			case T__15:
			case T__16:
			case T__17:
			case T__18:
			case T__19:
			case T__20:
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
				case T__11:
				case T__12:
				case T__18:
				case T__19:
				case T__20:
				case ID:
				case NUMBER:
				case STRING:
					{
					setState(37);
					expr(0);
					}
					break;
				case T__13:
				case T__15:
				case T__16:
				case T__17:
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
		public List<VariableContext> variable() {
			return getRuleContexts(VariableContext.class);
		}
		public VariableContext variable(int i) {
			return getRuleContext(VariableContext.class,i);
		}
		public ScopeContext scope() {
			return getRuleContext(ScopeContext.class,0);
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
			if (_la==ID) {
				{
				setState(54);
				variable();
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
						variable();
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
			setState(78);
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
			case T__11:
			case T__12:
			case T__18:
			case T__19:
			case T__20:
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
			case T__6:
				{
				_localctx = new ExprParenContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(74);
				match(T__6);
				setState(75);
				expr(0);
				setState(76);
				match(T__8);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(104);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,11,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(102);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,10,_ctx) ) {
					case 1:
						{
						_localctx = new ExprMulDivContext(new ExprContext(_parentctx, _parentState));
						((ExprMulDivContext)_localctx).lhs = _prevctx;
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(80);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(81);
						((ExprMulDivContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==T__9 || _la==T__10) ) {
							((ExprMulDivContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(82);
						((ExprMulDivContext)_localctx).rhs = expr(4);
						}
						break;
					case 2:
						{
						_localctx = new ExprAddSubContext(new ExprContext(_parentctx, _parentState));
						((ExprAddSubContext)_localctx).lhs = _prevctx;
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(83);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(84);
						((ExprAddSubContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==T__11 || _la==T__12) ) {
							((ExprAddSubContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(85);
						((ExprAddSubContext)_localctx).rhs = expr(3);
						}
						break;
					case 3:
						{
						_localctx = new ExprCallContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(86);
						if (!(precpred(_ctx, 7))) throw new FailedPredicateException(this, "precpred(_ctx, 7)");
						setState(87);
						match(T__6);
						setState(99);
						_errHandler.sync(this);
						_la = _input.LA(1);
						if ((((_la) & ~0x3f) == 0 && ((1L << _la) & 33042562L) != 0)) {
							{
							setState(88);
							expr(0);
							setState(93);
							_errHandler.sync(this);
							_alt = getInterpreter().adaptivePredict(_input,7,_ctx);
							while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
								if ( _alt==1 ) {
									{
									{
									setState(89);
									match(T__7);
									setState(90);
									expr(0);
									}
									} 
								}
								setState(95);
								_errHandler.sync(this);
								_alt = getInterpreter().adaptivePredict(_input,7,_ctx);
							}
							setState(97);
							_errHandler.sync(this);
							_la = _input.LA(1);
							if (_la==T__7) {
								{
								setState(96);
								match(T__7);
								}
							}

							}
						}

						setState(101);
						match(T__8);
						}
						break;
					}
					} 
				}
				setState(106);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,11,_ctx);
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
		public TerminalNode ID() { return getToken(KatoParser.ID, 0); }
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
			setState(107);
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
		public If_then_elseContext if_then_else() {
			return getRuleContext(If_then_elseContext.class,0);
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
			setState(112);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__13:
				enterOuterAlt(_localctx, 1);
				{
				setState(109);
				if_then_else();
				}
				break;
			case T__15:
			case T__16:
				enterOuterAlt(_localctx, 2);
				{
				setState(110);
				return_();
				}
				break;
			case T__17:
				enterOuterAlt(_localctx, 3);
				{
				setState(111);
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
	public static class If_then_elseContext extends ParserRuleContext {
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
		public If_then_elseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_if_then_else; }
	}

	public final If_then_elseContext if_then_else() throws RecognitionException {
		If_then_elseContext _localctx = new If_then_elseContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_if_then_else);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(114);
			match(T__13);
			setState(115);
			match(T__6);
			setState(116);
			((If_then_elseContext)_localctx).condition = expr(0);
			setState(117);
			match(T__8);
			setState(118);
			((If_then_elseContext)_localctx).then = scope();
			setState(121);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==T__14) {
				{
				setState(119);
				match(T__14);
				setState(120);
				((If_then_elseContext)_localctx).else_ = scope();
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
			setState(126);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__15:
				{
				setState(123);
				match(T__15);
				}
				break;
			case T__16:
				{
				{
				setState(124);
				match(T__16);
				setState(125);
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
			setState(128);
			match(T__17);
			setState(129);
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
		public List<TerminalNode> NUMBER() { return getTokens(KatoParser.NUMBER); }
		public TerminalNode NUMBER(int i) {
			return getToken(KatoParser.NUMBER, i);
		}
		public LiteralIntegerContext(LiteralContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class LiteralStringContext extends LiteralContext {
		public TerminalNode STRING() { return getToken(KatoParser.STRING, 0); }
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
		public List<TerminalNode> NUMBER() { return getTokens(KatoParser.NUMBER); }
		public TerminalNode NUMBER(int i) {
			return getToken(KatoParser.NUMBER, i);
		}
		public LiteralNumberContext(LiteralContext ctx) { copyFrom(ctx); }
	}

	public final LiteralContext literal() throws RecognitionException {
		LiteralContext _localctx = new LiteralContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_literal);
		int _la;
		try {
			int _alt;
			setState(167);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,22,_ctx) ) {
			case 1:
				_localctx = new LiteralNumberContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(132);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==T__11 || _la==T__12) {
					{
					setState(131);
					((LiteralNumberContext)_localctx).sign = _input.LT(1);
					_la = _input.LA(1);
					if ( !(_la==T__11 || _la==T__12) ) {
						((LiteralNumberContext)_localctx).sign = (Token)_errHandler.recoverInline(this);
					}
					else {
						if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
						_errHandler.reportMatch(this);
						consume();
					}
					}
				}

				setState(158);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,20,_ctx) ) {
				case 1:
					{
					{
					setState(135); 
					_errHandler.sync(this);
					_la = _input.LA(1);
					do {
						{
						{
						setState(134);
						((LiteralNumberContext)_localctx).left = match(NUMBER);
						}
						}
						setState(137); 
						_errHandler.sync(this);
						_la = _input.LA(1);
					} while ( _la==NUMBER );
					setState(139);
					match(T__18);
					setState(143);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,17,_ctx);
					while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
						if ( _alt==1 ) {
							{
							{
							setState(140);
							((LiteralNumberContext)_localctx).right = match(NUMBER);
							}
							} 
						}
						setState(145);
						_errHandler.sync(this);
						_alt = getInterpreter().adaptivePredict(_input,17,_ctx);
					}
					}
					}
					break;
				case 2:
					{
					setState(149);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==NUMBER) {
						{
						{
						setState(146);
						((LiteralNumberContext)_localctx).left = match(NUMBER);
						}
						}
						setState(151);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(152);
					match(T__18);
					setState(154); 
					_errHandler.sync(this);
					_alt = 1;
					do {
						switch (_alt) {
						case 1:
							{
							{
							setState(153);
							((LiteralNumberContext)_localctx).right = match(NUMBER);
							}
							}
							break;
						default:
							throw new NoViableAltException(this);
						}
						setState(156); 
						_errHandler.sync(this);
						_alt = getInterpreter().adaptivePredict(_input,19,_ctx);
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
				setState(161); 
				_errHandler.sync(this);
				_alt = 1;
				do {
					switch (_alt) {
					case 1:
						{
						{
						setState(160);
						match(NUMBER);
						}
						}
						break;
					default:
						throw new NoViableAltException(this);
					}
					setState(163); 
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,21,_ctx);
				} while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER );
				}
				break;
			case 3:
				_localctx = new LiteralBooleanContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(165);
				_la = _input.LA(1);
				if ( !(_la==T__19 || _la==T__20) ) {
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
				setState(166);
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
			return precpred(_ctx, 7);
		}
		return true;
	}

	public static final String _serializedATN =
		"\u0004\u0001\u001a\u00aa\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001"+
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
		"\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0003\u0005O\b\u0005\u0001"+
		"\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0001"+
		"\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0001\u0005\u0005\u0005\\\b"+
		"\u0005\n\u0005\f\u0005_\t\u0005\u0001\u0005\u0003\u0005b\b\u0005\u0003"+
		"\u0005d\b\u0005\u0001\u0005\u0005\u0005g\b\u0005\n\u0005\f\u0005j\t\u0005"+
		"\u0001\u0006\u0001\u0006\u0001\u0007\u0001\u0007\u0001\u0007\u0003\u0007"+
		"q\b\u0007\u0001\b\u0001\b\u0001\b\u0001\b\u0001\b\u0001\b\u0001\b\u0003"+
		"\bz\b\b\u0001\t\u0001\t\u0001\t\u0003\t\u007f\b\t\u0001\n\u0001\n\u0001"+
		"\n\u0001\u000b\u0003\u000b\u0085\b\u000b\u0001\u000b\u0004\u000b\u0088"+
		"\b\u000b\u000b\u000b\f\u000b\u0089\u0001\u000b\u0001\u000b\u0005\u000b"+
		"\u008e\b\u000b\n\u000b\f\u000b\u0091\t\u000b\u0001\u000b\u0005\u000b\u0094"+
		"\b\u000b\n\u000b\f\u000b\u0097\t\u000b\u0001\u000b\u0001\u000b\u0004\u000b"+
		"\u009b\b\u000b\u000b\u000b\f\u000b\u009c\u0003\u000b\u009f\b\u000b\u0001"+
		"\u000b\u0004\u000b\u00a2\b\u000b\u000b\u000b\f\u000b\u00a3\u0001\u000b"+
		"\u0001\u000b\u0003\u000b\u00a8\b\u000b\u0001\u000b\u0000\u0001\n\f\u0000"+
		"\u0002\u0004\u0006\b\n\f\u000e\u0010\u0012\u0014\u0016\u0000\u0003\u0001"+
		"\u0000\n\u000b\u0001\u0000\f\r\u0001\u0000\u0014\u0015\u00bc\u0000\u0018"+
		"\u0001\u0000\u0000\u0000\u0002\u001a\u0001\u0000\u0000\u0000\u0004,\u0001"+
		"\u0000\u0000\u0000\u0006.\u0001\u0000\u0000\u0000\b3\u0001\u0000\u0000"+
		"\u0000\nN\u0001\u0000\u0000\u0000\fk\u0001\u0000\u0000\u0000\u000ep\u0001"+
		"\u0000\u0000\u0000\u0010r\u0001\u0000\u0000\u0000\u0012~\u0001\u0000\u0000"+
		"\u0000\u0014\u0080\u0001\u0000\u0000\u0000\u0016\u00a7\u0001\u0000\u0000"+
		"\u0000\u0018\u0019\u0003\u0002\u0001\u0000\u0019\u0001\u0001\u0000\u0000"+
		"\u0000\u001a\u001e\u0005\u0001\u0000\u0000\u001b\u001d\u0003\u0004\u0002"+
		"\u0000\u001c\u001b\u0001\u0000\u0000\u0000\u001d \u0001\u0000\u0000\u0000"+
		"\u001e\u001c\u0001\u0000\u0000\u0000\u001e\u001f\u0001\u0000\u0000\u0000"+
		"\u001f!\u0001\u0000\u0000\u0000 \u001e\u0001\u0000\u0000\u0000!\"\u0005"+
		"\u0002\u0000\u0000\"\u0003\u0001\u0000\u0000\u0000#(\u0003\u0006\u0003"+
		"\u0000$(\u0003\b\u0004\u0000%(\u0003\n\u0005\u0000&(\u0003\u000e\u0007"+
		"\u0000\'#\u0001\u0000\u0000\u0000\'$\u0001\u0000\u0000\u0000\'%\u0001"+
		"\u0000\u0000\u0000\'&\u0001\u0000\u0000\u0000()\u0001\u0000\u0000\u0000"+
		")*\u0005\u0003\u0000\u0000*-\u0001\u0000\u0000\u0000+-\u0005\u001a\u0000"+
		"\u0000,\'\u0001\u0000\u0000\u0000,+\u0001\u0000\u0000\u0000-\u0005\u0001"+
		"\u0000\u0000\u0000./\u0005\u0004\u0000\u0000/0\u0003\f\u0006\u000001\u0005"+
		"\u0005\u0000\u000012\u0003\n\u0005\u00002\u0007\u0001\u0000\u0000\u0000"+
		"34\u0005\u0006\u0000\u000045\u0003\f\u0006\u00005A\u0005\u0007\u0000\u0000"+
		"6;\u0003\f\u0006\u000078\u0005\b\u0000\u00008:\u0003\f\u0006\u000097\u0001"+
		"\u0000\u0000\u0000:=\u0001\u0000\u0000\u0000;9\u0001\u0000\u0000\u0000"+
		";<\u0001\u0000\u0000\u0000<?\u0001\u0000\u0000\u0000=;\u0001\u0000\u0000"+
		"\u0000>@\u0005\b\u0000\u0000?>\u0001\u0000\u0000\u0000?@\u0001\u0000\u0000"+
		"\u0000@B\u0001\u0000\u0000\u0000A6\u0001\u0000\u0000\u0000AB\u0001\u0000"+
		"\u0000\u0000BC\u0001\u0000\u0000\u0000CD\u0005\t\u0000\u0000DE\u0003\u0002"+
		"\u0001\u0000E\t\u0001\u0000\u0000\u0000FG\u0006\u0005\uffff\uffff\u0000"+
		"GO\u0003\f\u0006\u0000HO\u0003\u0016\u000b\u0000IO\u0003\u0002\u0001\u0000"+
		"JK\u0005\u0007\u0000\u0000KL\u0003\n\u0005\u0000LM\u0005\t\u0000\u0000"+
		"MO\u0001\u0000\u0000\u0000NF\u0001\u0000\u0000\u0000NH\u0001\u0000\u0000"+
		"\u0000NI\u0001\u0000\u0000\u0000NJ\u0001\u0000\u0000\u0000Oh\u0001\u0000"+
		"\u0000\u0000PQ\n\u0003\u0000\u0000QR\u0007\u0000\u0000\u0000Rg\u0003\n"+
		"\u0005\u0004ST\n\u0002\u0000\u0000TU\u0007\u0001\u0000\u0000Ug\u0003\n"+
		"\u0005\u0003VW\n\u0007\u0000\u0000Wc\u0005\u0007\u0000\u0000X]\u0003\n"+
		"\u0005\u0000YZ\u0005\b\u0000\u0000Z\\\u0003\n\u0005\u0000[Y\u0001\u0000"+
		"\u0000\u0000\\_\u0001\u0000\u0000\u0000][\u0001\u0000\u0000\u0000]^\u0001"+
		"\u0000\u0000\u0000^a\u0001\u0000\u0000\u0000_]\u0001\u0000\u0000\u0000"+
		"`b\u0005\b\u0000\u0000a`\u0001\u0000\u0000\u0000ab\u0001\u0000\u0000\u0000"+
		"bd\u0001\u0000\u0000\u0000cX\u0001\u0000\u0000\u0000cd\u0001\u0000\u0000"+
		"\u0000de\u0001\u0000\u0000\u0000eg\u0005\t\u0000\u0000fP\u0001\u0000\u0000"+
		"\u0000fS\u0001\u0000\u0000\u0000fV\u0001\u0000\u0000\u0000gj\u0001\u0000"+
		"\u0000\u0000hf\u0001\u0000\u0000\u0000hi\u0001\u0000\u0000\u0000i\u000b"+
		"\u0001\u0000\u0000\u0000jh\u0001\u0000\u0000\u0000kl\u0005\u0016\u0000"+
		"\u0000l\r\u0001\u0000\u0000\u0000mq\u0003\u0010\b\u0000nq\u0003\u0012"+
		"\t\u0000oq\u0003\u0014\n\u0000pm\u0001\u0000\u0000\u0000pn\u0001\u0000"+
		"\u0000\u0000po\u0001\u0000\u0000\u0000q\u000f\u0001\u0000\u0000\u0000"+
		"rs\u0005\u000e\u0000\u0000st\u0005\u0007\u0000\u0000tu\u0003\n\u0005\u0000"+
		"uv\u0005\t\u0000\u0000vy\u0003\u0002\u0001\u0000wx\u0005\u000f\u0000\u0000"+
		"xz\u0003\u0002\u0001\u0000yw\u0001\u0000\u0000\u0000yz\u0001\u0000\u0000"+
		"\u0000z\u0011\u0001\u0000\u0000\u0000{\u007f\u0005\u0010\u0000\u0000|"+
		"}\u0005\u0011\u0000\u0000}\u007f\u0003\n\u0005\u0000~{\u0001\u0000\u0000"+
		"\u0000~|\u0001\u0000\u0000\u0000\u007f\u0013\u0001\u0000\u0000\u0000\u0080"+
		"\u0081\u0005\u0012\u0000\u0000\u0081\u0082\u0003\n\u0005\u0000\u0082\u0015"+
		"\u0001\u0000\u0000\u0000\u0083\u0085\u0007\u0001\u0000\u0000\u0084\u0083"+
		"\u0001\u0000\u0000\u0000\u0084\u0085\u0001\u0000\u0000\u0000\u0085\u009e"+
		"\u0001\u0000\u0000\u0000\u0086\u0088\u0005\u0017\u0000\u0000\u0087\u0086"+
		"\u0001\u0000\u0000\u0000\u0088\u0089\u0001\u0000\u0000\u0000\u0089\u0087"+
		"\u0001\u0000\u0000\u0000\u0089\u008a\u0001\u0000\u0000\u0000\u008a\u008b"+
		"\u0001\u0000\u0000\u0000\u008b\u008f\u0005\u0013\u0000\u0000\u008c\u008e"+
		"\u0005\u0017\u0000\u0000\u008d\u008c\u0001\u0000\u0000\u0000\u008e\u0091"+
		"\u0001\u0000\u0000\u0000\u008f\u008d\u0001\u0000\u0000\u0000\u008f\u0090"+
		"\u0001\u0000\u0000\u0000\u0090\u009f\u0001\u0000\u0000\u0000\u0091\u008f"+
		"\u0001\u0000\u0000\u0000\u0092\u0094\u0005\u0017\u0000\u0000\u0093\u0092"+
		"\u0001\u0000\u0000\u0000\u0094\u0097\u0001\u0000\u0000\u0000\u0095\u0093"+
		"\u0001\u0000\u0000\u0000\u0095\u0096\u0001\u0000\u0000\u0000\u0096\u0098"+
		"\u0001\u0000\u0000\u0000\u0097\u0095\u0001\u0000\u0000\u0000\u0098\u009a"+
		"\u0005\u0013\u0000\u0000\u0099\u009b\u0005\u0017\u0000\u0000\u009a\u0099"+
		"\u0001\u0000\u0000\u0000\u009b\u009c\u0001\u0000\u0000\u0000\u009c\u009a"+
		"\u0001\u0000\u0000\u0000\u009c\u009d\u0001\u0000\u0000\u0000\u009d\u009f"+
		"\u0001\u0000\u0000\u0000\u009e\u0087\u0001\u0000\u0000\u0000\u009e\u0095"+
		"\u0001\u0000\u0000\u0000\u009f\u00a8\u0001\u0000\u0000\u0000\u00a0\u00a2"+
		"\u0005\u0017\u0000\u0000\u00a1\u00a0\u0001\u0000\u0000\u0000\u00a2\u00a3"+
		"\u0001\u0000\u0000\u0000\u00a3\u00a1\u0001\u0000\u0000\u0000\u00a3\u00a4"+
		"\u0001\u0000\u0000\u0000\u00a4\u00a8\u0001\u0000\u0000\u0000\u00a5\u00a8"+
		"\u0007\u0002\u0000\u0000\u00a6\u00a8\u0005\u0018\u0000\u0000\u00a7\u0084"+
		"\u0001\u0000\u0000\u0000\u00a7\u00a1\u0001\u0000\u0000\u0000\u00a7\u00a5"+
		"\u0001\u0000\u0000\u0000\u00a7\u00a6\u0001\u0000\u0000\u0000\u00a8\u0017"+
		"\u0001\u0000\u0000\u0000\u0017\u001e\',;?AN]acfhpy~\u0084\u0089\u008f"+
		"\u0095\u009c\u009e\u00a3\u00a7";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}