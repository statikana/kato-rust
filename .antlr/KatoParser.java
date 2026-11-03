// Generated from /home/ryan/Documents/github/statikana/kato-rust/Kato.g4 by ANTLR 4.13.1
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
		T__0=1, T__1=2, T__2=3, T__3=4, T__4=5, T__5=6, T__6=7, TRUE=8, FALSE=9, 
		INTEGER=10, REAL=11, MUL=12, DIV=13, ADD=14, SUB=15, PAREN_L=16, PAREN_R=17, 
		WS=18;
	public static final int
		RULE_scope = 0, RULE_expr = 1, RULE_term = 2, RULE_literal = 3, RULE_boolean = 4, 
		RULE_numeric = 5, RULE_matrix = 6;
	private static String[] makeRuleNames() {
		return new String[] {
			"scope", "expr", "term", "literal", "boolean", "numeric", "matrix"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "'{'", "';'", "'}'", "'out'", "'['", "','", "']'", "'true'", "'false'", 
			null, null, "'*'", "'/'", "'+'", "'-'", "'('", "')'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, "TRUE", "FALSE", "INTEGER", 
			"REAL", "MUL", "DIV", "ADD", "SUB", "PAREN_L", "PAREN_R", "WS"
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
	public static class ScopeContext extends ParserRuleContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public ScopeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_scope; }
	}

	public final ScopeContext scope() throws RecognitionException {
		ScopeContext _localctx = new ScopeContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_scope);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(14);
			match(T__0);
			setState(18); 
			_errHandler.sync(this);
			_la = _input.LA(1);
			do {
				{
				{
				setState(15);
				expr();
				setState(16);
				match(T__1);
				}
				}
				setState(20); 
				_errHandler.sync(this);
				_la = _input.LA(1);
			} while ( (((_la) & ~0x3f) == 0 && ((1L << _la) & 102192L) != 0) );
			setState(22);
			match(T__2);
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
	public static class Expr_outContext extends ExprContext {
		public TerminalNode PAREN_L() { return getToken(KatoParser.PAREN_L, 0); }
		public TermContext term() {
			return getRuleContext(TermContext.class,0);
		}
		public TerminalNode PAREN_R() { return getToken(KatoParser.PAREN_R, 0); }
		public Expr_outContext(ExprContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Expr_termContext extends ExprContext {
		public TermContext term() {
			return getRuleContext(TermContext.class,0);
		}
		public Expr_termContext(ExprContext ctx) { copyFrom(ctx); }
	}

	public final ExprContext expr() throws RecognitionException {
		ExprContext _localctx = new ExprContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_expr);
		try {
			setState(30);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__3:
				_localctx = new Expr_outContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(24);
				match(T__3);
				setState(25);
				match(PAREN_L);
				setState(26);
				term(0);
				setState(27);
				match(PAREN_R);
				}
				break;
			case T__4:
			case TRUE:
			case FALSE:
			case INTEGER:
			case REAL:
			case SUB:
			case PAREN_L:
				_localctx = new Expr_termContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(29);
				term(0);
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
	public static class TermContext extends ParserRuleContext {
		public TermContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_term; }
	 
		public TermContext() { }
		public void copyFrom(TermContext ctx) {
			super.copyFrom(ctx);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Term_add_subContext extends TermContext {
		public TermContext lhs;
		public TermContext rhs;
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public TerminalNode ADD() { return getToken(KatoParser.ADD, 0); }
		public TerminalNode SUB() { return getToken(KatoParser.SUB, 0); }
		public Term_add_subContext(TermContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Term_literalContext extends TermContext {
		public LiteralContext literal() {
			return getRuleContext(LiteralContext.class,0);
		}
		public Term_literalContext(TermContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Term_negContext extends TermContext {
		public TerminalNode SUB() { return getToken(KatoParser.SUB, 0); }
		public TermContext term() {
			return getRuleContext(TermContext.class,0);
		}
		public Term_negContext(TermContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Term_mul_divContext extends TermContext {
		public TermContext lhs;
		public TermContext rhs;
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public TerminalNode MUL() { return getToken(KatoParser.MUL, 0); }
		public TerminalNode DIV() { return getToken(KatoParser.DIV, 0); }
		public Term_mul_divContext(TermContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Term_parenContext extends TermContext {
		public TerminalNode PAREN_L() { return getToken(KatoParser.PAREN_L, 0); }
		public TermContext term() {
			return getRuleContext(TermContext.class,0);
		}
		public TerminalNode PAREN_R() { return getToken(KatoParser.PAREN_R, 0); }
		public Term_parenContext(TermContext ctx) { copyFrom(ctx); }
	}

	public final TermContext term() throws RecognitionException {
		return term(0);
	}

	private TermContext term(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		TermContext _localctx = new TermContext(_ctx, _parentState);
		TermContext _prevctx = _localctx;
		int _startState = 4;
		enterRecursionRule(_localctx, 4, RULE_term, _p);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(40);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__4:
			case TRUE:
			case FALSE:
			case INTEGER:
			case REAL:
				{
				_localctx = new Term_literalContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;

				setState(33);
				literal();
				}
				break;
			case SUB:
				{
				_localctx = new Term_negContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(34);
				match(SUB);
				setState(35);
				term(4);
				}
				break;
			case PAREN_L:
				{
				_localctx = new Term_parenContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(36);
				match(PAREN_L);
				setState(37);
				term(0);
				setState(38);
				match(PAREN_R);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(50);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,4,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(48);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,3,_ctx) ) {
					case 1:
						{
						_localctx = new Term_mul_divContext(new TermContext(_parentctx, _parentState));
						((Term_mul_divContext)_localctx).lhs = _prevctx;
						pushNewRecursionContext(_localctx, _startState, RULE_term);
						setState(42);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(43);
						_la = _input.LA(1);
						if ( !(_la==MUL || _la==DIV) ) {
						_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(44);
						((Term_mul_divContext)_localctx).rhs = term(4);
						}
						break;
					case 2:
						{
						_localctx = new Term_add_subContext(new TermContext(_parentctx, _parentState));
						((Term_add_subContext)_localctx).lhs = _prevctx;
						pushNewRecursionContext(_localctx, _startState, RULE_term);
						setState(45);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(46);
						_la = _input.LA(1);
						if ( !(_la==ADD || _la==SUB) ) {
						_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(47);
						((Term_add_subContext)_localctx).rhs = term(3);
						}
						break;
					}
					} 
				}
				setState(52);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,4,_ctx);
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
	public static class Value_booleanContext extends LiteralContext {
		public BooleanContext boolean_() {
			return getRuleContext(BooleanContext.class,0);
		}
		public Value_booleanContext(LiteralContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Value_numericContext extends LiteralContext {
		public NumericContext numeric() {
			return getRuleContext(NumericContext.class,0);
		}
		public Value_numericContext(LiteralContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Value_matrixContext extends LiteralContext {
		public MatrixContext matrix() {
			return getRuleContext(MatrixContext.class,0);
		}
		public Value_matrixContext(LiteralContext ctx) { copyFrom(ctx); }
	}

	public final LiteralContext literal() throws RecognitionException {
		LiteralContext _localctx = new LiteralContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_literal);
		try {
			setState(56);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case TRUE:
			case FALSE:
				_localctx = new Value_booleanContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(53);
				boolean_();
				}
				break;
			case INTEGER:
			case REAL:
				_localctx = new Value_numericContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(54);
				numeric();
				}
				break;
			case T__4:
				_localctx = new Value_matrixContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(55);
				matrix();
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
	public static class BooleanContext extends ParserRuleContext {
		public BooleanContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_boolean; }
	 
		public BooleanContext() { }
		public void copyFrom(BooleanContext ctx) {
			super.copyFrom(ctx);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Boolean_falseContext extends BooleanContext {
		public TerminalNode FALSE() { return getToken(KatoParser.FALSE, 0); }
		public Boolean_falseContext(BooleanContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Boolean_trueContext extends BooleanContext {
		public TerminalNode TRUE() { return getToken(KatoParser.TRUE, 0); }
		public Boolean_trueContext(BooleanContext ctx) { copyFrom(ctx); }
	}

	public final BooleanContext boolean_() throws RecognitionException {
		BooleanContext _localctx = new BooleanContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_boolean);
		try {
			setState(60);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case TRUE:
				_localctx = new Boolean_trueContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(58);
				match(TRUE);
				}
				break;
			case FALSE:
				_localctx = new Boolean_falseContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(59);
				match(FALSE);
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
	public static class NumericContext extends ParserRuleContext {
		public NumericContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_numeric; }
	 
		public NumericContext() { }
		public void copyFrom(NumericContext ctx) {
			super.copyFrom(ctx);
		}
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Numeric_realContext extends NumericContext {
		public TerminalNode REAL() { return getToken(KatoParser.REAL, 0); }
		public Numeric_realContext(NumericContext ctx) { copyFrom(ctx); }
	}
	@SuppressWarnings("CheckReturnValue")
	public static class Numeric_integerContext extends NumericContext {
		public TerminalNode INTEGER() { return getToken(KatoParser.INTEGER, 0); }
		public Numeric_integerContext(NumericContext ctx) { copyFrom(ctx); }
	}

	public final NumericContext numeric() throws RecognitionException {
		NumericContext _localctx = new NumericContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_numeric);
		try {
			setState(64);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INTEGER:
				_localctx = new Numeric_integerContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(62);
				match(INTEGER);
				}
				break;
			case REAL:
				_localctx = new Numeric_realContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(63);
				match(REAL);
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
	public static class MatrixContext extends ParserRuleContext {
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public MatrixContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_matrix; }
	}

	public final MatrixContext matrix() throws RecognitionException {
		MatrixContext _localctx = new MatrixContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_matrix);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(66);
			match(T__4);
			setState(72);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,8,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(67);
					term(0);
					setState(68);
					match(T__5);
					}
					} 
				}
				setState(74);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,8,_ctx);
			}
			setState(76);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if ((((_la) & ~0x3f) == 0 && ((1L << _la) & 102176L) != 0)) {
				{
				setState(75);
				term(0);
				}
			}

			setState(78);
			match(T__6);
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
		case 2:
			return term_sempred((TermContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean term_sempred(TermContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 3);
		case 1:
			return precpred(_ctx, 2);
		}
		return true;
	}

	public static final String _serializedATN =
		"\u0004\u0001\u0012Q\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001\u0002"+
		"\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004\u0002"+
		"\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0001\u0000\u0001\u0000\u0001"+
		"\u0000\u0001\u0000\u0004\u0000\u0013\b\u0000\u000b\u0000\f\u0000\u0014"+
		"\u0001\u0000\u0001\u0000\u0001\u0001\u0001\u0001\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0001\u0001\u0003\u0001\u001f\b\u0001\u0001\u0002\u0001\u0002"+
		"\u0001\u0002\u0001\u0002\u0001\u0002\u0001\u0002\u0001\u0002\u0001\u0002"+
		"\u0003\u0002)\b\u0002\u0001\u0002\u0001\u0002\u0001\u0002\u0001\u0002"+
		"\u0001\u0002\u0001\u0002\u0005\u00021\b\u0002\n\u0002\f\u00024\t\u0002"+
		"\u0001\u0003\u0001\u0003\u0001\u0003\u0003\u00039\b\u0003\u0001\u0004"+
		"\u0001\u0004\u0003\u0004=\b\u0004\u0001\u0005\u0001\u0005\u0003\u0005"+
		"A\b\u0005\u0001\u0006\u0001\u0006\u0001\u0006\u0001\u0006\u0005\u0006"+
		"G\b\u0006\n\u0006\f\u0006J\t\u0006\u0001\u0006\u0003\u0006M\b\u0006\u0001"+
		"\u0006\u0001\u0006\u0001\u0006\u0000\u0001\u0004\u0007\u0000\u0002\u0004"+
		"\u0006\b\n\f\u0000\u0002\u0001\u0000\f\r\u0001\u0000\u000e\u000fU\u0000"+
		"\u000e\u0001\u0000\u0000\u0000\u0002\u001e\u0001\u0000\u0000\u0000\u0004"+
		"(\u0001\u0000\u0000\u0000\u00068\u0001\u0000\u0000\u0000\b<\u0001\u0000"+
		"\u0000\u0000\n@\u0001\u0000\u0000\u0000\fB\u0001\u0000\u0000\u0000\u000e"+
		"\u0012\u0005\u0001\u0000\u0000\u000f\u0010\u0003\u0002\u0001\u0000\u0010"+
		"\u0011\u0005\u0002\u0000\u0000\u0011\u0013\u0001\u0000\u0000\u0000\u0012"+
		"\u000f\u0001\u0000\u0000\u0000\u0013\u0014\u0001\u0000\u0000\u0000\u0014"+
		"\u0012\u0001\u0000\u0000\u0000\u0014\u0015\u0001\u0000\u0000\u0000\u0015"+
		"\u0016\u0001\u0000\u0000\u0000\u0016\u0017\u0005\u0003\u0000\u0000\u0017"+
		"\u0001\u0001\u0000\u0000\u0000\u0018\u0019\u0005\u0004\u0000\u0000\u0019"+
		"\u001a\u0005\u0010\u0000\u0000\u001a\u001b\u0003\u0004\u0002\u0000\u001b"+
		"\u001c\u0005\u0011\u0000\u0000\u001c\u001f\u0001\u0000\u0000\u0000\u001d"+
		"\u001f\u0003\u0004\u0002\u0000\u001e\u0018\u0001\u0000\u0000\u0000\u001e"+
		"\u001d\u0001\u0000\u0000\u0000\u001f\u0003\u0001\u0000\u0000\u0000 !\u0006"+
		"\u0002\uffff\uffff\u0000!)\u0003\u0006\u0003\u0000\"#\u0005\u000f\u0000"+
		"\u0000#)\u0003\u0004\u0002\u0004$%\u0005\u0010\u0000\u0000%&\u0003\u0004"+
		"\u0002\u0000&\'\u0005\u0011\u0000\u0000\')\u0001\u0000\u0000\u0000( \u0001"+
		"\u0000\u0000\u0000(\"\u0001\u0000\u0000\u0000($\u0001\u0000\u0000\u0000"+
		")2\u0001\u0000\u0000\u0000*+\n\u0003\u0000\u0000+,\u0007\u0000\u0000\u0000"+
		",1\u0003\u0004\u0002\u0004-.\n\u0002\u0000\u0000./\u0007\u0001\u0000\u0000"+
		"/1\u0003\u0004\u0002\u00030*\u0001\u0000\u0000\u00000-\u0001\u0000\u0000"+
		"\u000014\u0001\u0000\u0000\u000020\u0001\u0000\u0000\u000023\u0001\u0000"+
		"\u0000\u00003\u0005\u0001\u0000\u0000\u000042\u0001\u0000\u0000\u0000"+
		"59\u0003\b\u0004\u000069\u0003\n\u0005\u000079\u0003\f\u0006\u000085\u0001"+
		"\u0000\u0000\u000086\u0001\u0000\u0000\u000087\u0001\u0000\u0000\u0000"+
		"9\u0007\u0001\u0000\u0000\u0000:=\u0005\b\u0000\u0000;=\u0005\t\u0000"+
		"\u0000<:\u0001\u0000\u0000\u0000<;\u0001\u0000\u0000\u0000=\t\u0001\u0000"+
		"\u0000\u0000>A\u0005\n\u0000\u0000?A\u0005\u000b\u0000\u0000@>\u0001\u0000"+
		"\u0000\u0000@?\u0001\u0000\u0000\u0000A\u000b\u0001\u0000\u0000\u0000"+
		"BH\u0005\u0005\u0000\u0000CD\u0003\u0004\u0002\u0000DE\u0005\u0006\u0000"+
		"\u0000EG\u0001\u0000\u0000\u0000FC\u0001\u0000\u0000\u0000GJ\u0001\u0000"+
		"\u0000\u0000HF\u0001\u0000\u0000\u0000HI\u0001\u0000\u0000\u0000IL\u0001"+
		"\u0000\u0000\u0000JH\u0001\u0000\u0000\u0000KM\u0003\u0004\u0002\u0000"+
		"LK\u0001\u0000\u0000\u0000LM\u0001\u0000\u0000\u0000MN\u0001\u0000\u0000"+
		"\u0000NO\u0005\u0007\u0000\u0000O\r\u0001\u0000\u0000\u0000\n\u0014\u001e"+
		"(028<@HL";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}