grammar Kato;

scope: '{' (expr ';')+ '}';

expr
    : 'out' '(' term ')'     # expr_out // debugging
    | term          # expr_term
    ;

term
    : literal                 # term_literal
    | lhs=term (MUL | DIV) rhs=term # term_mul_div
    | lhs=term (ADD | SUB) rhs=term # term_add_sub
    | PAREN_L term PAREN_R  # term_paren
    | '-' term  # term_neg
    ;

literal
    : boolean       # value_boolean
    | numeric       # value_numeric
    ;

boolean
    : TRUE          # boolean_true
    | FALSE         # boolean_false
    ;

numeric
    : INTEGER       # numeric_integer
    | REAL          # numeric_real
    ;

// END RULES

// START LEXER TOKENS

TRUE:       'true';
FALSE:      'false';
INTEGER:    [0-9]+;
REAL:       [0-9]+ '.' [0-9]+; // must have something on both sides. No 1. or .9 weirdness.

MUL:    '*';
DIV:    '/';
ADD:    '+';
SUB:    '-';
PAREN_L: '(';
PAREN_R: ')';

WS: [ \t\n] -> skip;
