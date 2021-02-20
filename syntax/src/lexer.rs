use rowan::SmolStr;
use rowan::TextSize;
use parser::syntax_kind::SyntaxKind;
use SyntaxKind::*;

pub struct Token {
    pub kind: SyntaxKind,
    pub len: TextSize,
}

pub fn lex(_source: &str) -> Vec<Token> {
    vec![
        Token{kind: LET_KW, len: 3.into()},
        Token{kind: WHITESPACE, len: 1.into()},
        Token{kind: IDENT, len: 3.into()},
        Token{kind: WHITESPACE, len: 1.into()},
        Token{kind: EQ, len: 1.into()},
        Token{kind: WHITESPACE, len: 1.into()},
        Token{kind: NUMBER, len: 1.into()},
    ]
}
