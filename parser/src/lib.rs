// This crate contains the parser / grammar for Vim script.
//
// Most of the tests are inside syntax crate.

pub mod syntax_kind;

use crate::syntax_kind::SyntaxKind;
use SyntaxKind::*;

pub trait TokenSource {
    fn current(&self) -> SyntaxKind;
    fn bump(&mut self);
}

pub trait TreeSink {
    fn token(&mut self, kind: SyntaxKind);
    fn start_node(&mut self, kind: SyntaxKind);
    fn finish_node(&mut self);
    fn error(&mut self, error: String);
}

pub fn parse(_source: &mut impl TokenSource, sink: &mut impl TreeSink) {
    sink.start_node(ROOT);
        sink.start_node(LET_STMT);
            sink.token(LET_KW);
            sink.token(WHITESPACE);
            sink.token(EQ);
            sink.token(WHITESPACE);
            sink.start_node(IDENT_EXPR);
                sink.token(IDENT);
            sink.finish_node();
        sink.finish_node();
    sink.finish_node();
}
