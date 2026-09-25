grammar Kato;

program: scope;

scope: '{' statement* '}';

statement
    : ((varDefinition | funcDefinition | expr | control) ';')
    | COMMENT;

// variable and function definitions
varDefinition: 'let ' variable '=' expr;
funcDefinition: 'func ' variable '(' (variable (',' variable)* ','?)? ')' scope;

// expressions
expr
    : expr '(' (expr (',' expr)* ','?)? ')' #ExprCall
    | variable  #ExprVar
    | literal   #ExprLiteral
    | scope     #ExprScope
    // | '[' (expr (',' expr)*)? ']' #ExprArray

    | lhs=expr op=('*' | '/') rhs=expr  #ExprMulDiv
    | lhs=expr op=('+' | '-') rhs=expr  #ExprAddSub
    | '(' expr ')'                      #ExprParen
    ;

variable: ID;

control: if_then_else | return_ | emit_;
if_then_else: 'if' '(' condition=expr ')' then=scope ('else' else_=scope)?;
return_: ('return' | ('return ' expr));
emit_: 'emit ' expr;

literal
    : sign=('+' | '-')? ((left=NUMBER+ '.' right=NUMBER*) | left=NUMBER* '.' right=NUMBER+) #LiteralNumber
    | NUMBER+ #LiteralInteger
    | ('true' | 'false') #LiteralBoolean
    | STRING #LiteralString
    ;


// Tokens
ID: CHAR (CHAR | NUMBER)*;
NUMBER: [0-9];

fragment CHAR: [a-zA-Z];
STRING: '"' .*? '"';
WS: [ \t\n] -> skip;

COMMENT: '/..' .*? '../' -> skip;

