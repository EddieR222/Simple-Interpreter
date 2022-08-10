# Simple-Interpreter
A simple interpreter that takes in Rust Like code and performs said code.


The following is the rules that this program follows
function        ::= fn-keyword fn-name { identifier } fn-operator expression
fn-name         ::= identifier
fn-operator     ::= '=>'
fn-keyword      ::= 'fn'

expression      ::= factor | expression operator expression
factor          ::= number | identifier | assignment | '(' expression ')' | function-call
assignment      ::= identifier '=' expression
function-call   ::= fn-name { expression }

operator        ::= '+' | '-' | '*' | '/' | '%'

identifier      ::= letter | '_' { identifier-char }
identifier-char ::= '_' | letter | digit

number          ::= { digit } [ '.' digit { digit } ]

letter          ::= 'a' | 'b' | ... | 'y' | 'z' | 'A' | 'B' | ... | 'Y' | 'Z'
digit           ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'


As of right now we only accept input via the input method but this can easily be made to read lines from a file.


Examples of things this interpreter can handle are the following
1. x = 7 // would store the variable x and the value 7
2. y = x + 1 // would add 1 to previous x value (7) and save y as 8
3. y = 9 - (z = 3) // would save z as 3 and then update y to 6
4. y = (9 - 8) + (4 / ( 6 + ( 7 * 400))) // would be able to solve complex and nested math expressions like this and solves it in the correct PEMDAS order
5. fn avg x y => (x + y) / 2 // would save this function so we can call it later
6. avg 4 2 // would return the correct function call with passed in paramter value, would get 3
7. fn echo x => x // would save more functions,
8. avg echo 2 echo 2 // handles nested function calls, so we would get 2 as the avg of 2 and 2 is 2
9. f = -10 // handles negative numbers
10. f = 9.4758 // also handles floating point numbers
