use parser::syntax_kind::SyntaxKind;
use SyntaxKind::*;
use rowan::SmolStr;

pub fn lex(_source: &str) -> Vec<(SyntaxKind, SmolStr)> {
    vec![
        (LET_KW, "let".into()),
        (WHITESPACE, " ".into()),
        (IDENT, "l:a".into()),
        (EQ, "=".into()),
        (WHITESPACE, " ".into()),
        (NUMBER, "5".into()),
    ]
}
