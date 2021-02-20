// This module is very strongly based on rust-analyzer.

pub mod lexer;

use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::Language;
use rowan::SmolStr;

use parser::syntax_kind::SyntaxKind;
use parser::TokenSource;
use parser::TreeSink;
use SyntaxKind::*;
use crate::lexer::lex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VimscriptLang {}
impl rowan::Language for VimscriptLang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }
    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<VimscriptLang>;

pub struct Parse {
    green_node: GreenNode,
    // TODO: add position
    errors: Vec<String>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}


pub fn parse(source: &str) -> Parse {
    let tokens = lex(source);
    let mut source = TextTokenSource {
        tokens: &tokens,
        current: 0,
    };
    let mut sink = TextTreeSink {
        builder: GreenNodeBuilder::new(),
        errors: Vec::new(),
        tokens: &tokens,
        current: 0,
    };
    parser::parse(&mut source, &mut sink);
    Parse {
        green_node: sink.builder.finish(),
        errors: sink.errors,
    }
}

struct TextTokenSource<'a> {
    // TODO: instead of SmolStr, pass the original text and use position (TextSize instead of
    // SmolStr).
    tokens: &'a [(SyntaxKind, SmolStr)],
    // Index into tokens
    current: usize,
}

impl<'a> TokenSource for TextTokenSource<'a> {
    fn current(&self) -> SyntaxKind {
        if self.current >= self.tokens.len() {
            return EOF;
        }
        self.tokens[self.current].0
    }
    fn bump(&mut self) {
        self.current += 1
    }
}

struct TextTreeSink<'a> {
    builder: GreenNodeBuilder<'static>,
    // TODO: add position
    errors: Vec<String>,
    // TODO: instead of SmolStr, pass the original text and use position (TextSize instead of
    // SmolStr).
    tokens: &'a [(SyntaxKind, SmolStr)],
    // Index into tokens
    current: usize,
}

impl<'a> TreeSink for TextTreeSink<'a> {
    fn token(&mut self, kind: SyntaxKind) {
        assert_eq!(kind, self.tokens[self.current].0);
        let kind = VimscriptLang::kind_to_raw(kind);
        self.builder
            .token(kind, self.tokens[self.current].1.clone());
        self.current += 1;
    }
    fn start_node(&mut self, kind: SyntaxKind) {
        let kind = VimscriptLang::kind_to_raw(kind);
        self.builder.start_node(kind);
    }
    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
    fn error(&mut self, error: String) {
        self.errors.push(error);
    }
}

#[cfg(test)]
mod tests;
