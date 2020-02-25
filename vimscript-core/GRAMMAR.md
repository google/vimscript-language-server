```
Stmt ::= LetStmt | CallStmt | NullStmt | IfStmt | FunctionStmt | ForStmt

LetStmt ::= 'let' VarName '=' Expr NewLine

CallStmt ::= 'call' MethodName '(' (Expr (',' Expr)*)? ')' NewLine

NullStmt ::= NewLine

IfStmt ::= 
  'if' Expr NewLine
  Stmt* 
  'endif' NewLine

FunctionStmt ::= 
  'function' '!'? FunctionName '(' (FunctionArg (',' FunctionArg)*)? ')' Abort? NewLine
  Stmt* 
  'endfunction' NewLine

ForStmt ::=
  'for' (VarName | '[' (VarName (',' VarName)*) ']' ) 'in' Expr NewLine
  Stmt*
  'endfor' NewLine

Expr ::=
  Number |
  StringLiteral |
  VarName |
  FunctionExpr |
  Expr '!=#' Expr |
  Expr '.' Expr |

FunctionExpr ::= VarName '(' (Expr (',' Expr)*)? ')'

NewLine ::= '\n' | EOF
```
