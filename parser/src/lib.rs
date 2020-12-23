// This crate contains the parser / grammar for Vim script.
//
// Most of the tests are inside syntax crate.

pub mod syntax_kind;

use crate::syntax_kind::SyntaxKind;

pub trait TokenSource {
    fn current(&self) -> SyntaxKind;
    fn bump(&mut self);
}

pub trait TreeSink {
    fn token(&mut self, kind: SyntaxKind);
    fn start_node(&mut self, kind: SyntaxKind);
    fn finish_node(&mut self, kind: SyntaxKind);
    fn error(&mut self, error: String);
}

pub fn parse(_source: &mut impl TokenSource, _sink: &mut impl TreeSink) {
}
