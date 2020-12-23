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

pub fn parse(source: &mut impl TokenSource, sink: &mut impl TreeSink) {
    sink.start_node(ROOT);
    match source.current() {
        LET_KW => parse_let_stmt(source, sink),
        // TODO: add error handling
        _ => {}
    }
    sink.finish_node();
}

// TODO: should parsing a statement also "eat" newline?
fn parse_let_stmt(source: &mut impl TokenSource, sink: &mut impl TreeSink) {
    sink.start_node(LET_STMT);

    assert_eq!(source.current(), LET_KW);
    bump_token_and_ws(source, sink);

    sink.start_node(LET_VAR);
    bump_token(source, sink);
    sink.finish_node();

    skip_ws(source, sink);

    assert_eq!(source.current(), EQ);
    bump_token_and_ws(source, sink);

    parse_expr(source, sink);

    sink.finish_node();
}

fn parse_expr(source: &mut impl TokenSource, sink: &mut impl TreeSink) {
    bump_token_and_ws(source, sink);
}

fn bump_token_and_ws(source: &mut impl TokenSource, sink: &mut impl TreeSink) {
    bump_token(source, sink);
    skip_ws(source, sink);
}

fn skip_ws(source: &mut impl TokenSource, sink: &mut impl TreeSink) {
    while source.current() == WHITESPACE {
        bump_token(source, sink);
    }
}

fn bump_token(source: &mut impl TokenSource, sink: &mut impl TreeSink) {
    sink.token(source.current());
    source.bump();
}
