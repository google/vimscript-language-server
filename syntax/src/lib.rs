// This module is very strongly based on rust-analyzer.

pub mod lexer;

use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::Language;
use rowan::SmolStr;
use rowan::TextSize;

use parser::syntax_kind::SyntaxKind;
use parser::TokenSource;
use parser::TreeSink;
use SyntaxKind::*;
use crate::lexer::lex;
use crate::lexer::Token;

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
    pub errors: Vec<String>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

pub fn parse(content: &str) -> Parse {
    let tokens = lex(content);
    let mut source = TextTokenSource {
        tokens: &tokens,
        current: 0,
    };
    let mut sink = TextTreeSink {
        content: content,
        builder: GreenNodeBuilder::new(),
        errors: Vec::new(),
        tokens: &tokens,
        current: 0,
        pos: 0,
    };
    parser::parse(&mut source, &mut sink);
    Parse {
        green_node: sink.builder.finish(),
        errors: sink.errors,
    }
}

struct TextTokenSource<'a> {
    tokens: &'a [Token],
    // Index into tokens
    current: usize,
}

impl<'a> TokenSource for TextTokenSource<'a> {
    fn current(&self) -> SyntaxKind {
        if self.current >= self.tokens.len() {
            return EOF;
        }
        self.tokens[self.current].kind
    }
    fn bump(&mut self) {
        self.current += 1
    }
}

struct TextTreeSink<'a> {
    builder: GreenNodeBuilder<'static>,
    content: &'a str,
    // TODO: add position
    errors: Vec<String>,
    tokens: &'a [Token],
    // Index into tokens
    current: usize,
    pos: usize,
}

impl<'a> TreeSink for TextTreeSink<'a> {
    fn token(&mut self, kind: SyntaxKind) {
        assert_eq!(kind, self.tokens[self.current].kind);
        let kind = VimscriptLang::kind_to_raw(kind);
        let len: usize = self.tokens[self.current].len.into();
        self.builder
            .token(kind, SmolStr::new(&self.content[self.pos..(self.pos+len)]));
        self.pos += len;
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
