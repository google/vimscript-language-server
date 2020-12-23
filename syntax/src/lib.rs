// This module is very strongly based on rust-analyzer.

use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::SmolStr;
use rowan::Language;

use parser::syntax_kind::SyntaxKind;
use SyntaxKind::*;

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

pub fn lex() -> Vec<(SyntaxKind, SmolStr)> {
    vec![
        (LET_KW, "let".into()),
        (WHITESPACE, " ".into()),
        (EQ, "=".into()),
        (WHITESPACE, " ".into()),
        (IDENT, "l:a".into()),
    ]
}

pub fn parse(_source: &str) -> Parse {
    let mut tokens = lex();
    tokens.reverse();
    Parser {
        tokens,
        builder: GreenNodeBuilder::new(),
        errors: Vec::new(),
    }
    .parse()
}

struct Parser {
    tokens: Vec<(SyntaxKind, SmolStr)>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
}

impl Parser {
    fn parse(mut self) -> Parse {
        self.start_node(ROOT.into());

        self.parse_stmt();

        self.builder.finish_node();
        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn parse_stmt(&mut self) {
        self.skip_ws();
        if self.current() == Some(LET_KW) {
            self.parse_let_stmt();
            return;
        }
        // TODO: do an error if this is invalid statement
    }

    fn parse_let_stmt(&mut self) {
        assert_eq!(self.current(), Some(LET_KW));
        self.start_node(LET_STMT);
        self.bump();
        self.skip_ws();
        // TODO: what to do if this is not a valid element?
        assert_eq!(self.current(), Some(EQ));
        self.bump();
        self.skip_ws();
        self.parse_expr();
        // TODO: read white spaces and comments up to end of line (or eof)
        self.builder.finish_node();
    }

    fn parse_expr(&mut self) {
        // TODO: this is parsing only ident expression
        assert_eq!(self.current(), Some(IDENT));
        self.start_node(IDENT_EXPR);
        self.bump();
        self.builder.finish_node();
    }

    // TODO: simplify error handling

    // TODO: bump_any, bump based on kind, etc.
    fn bump(&mut self) {
        let (kind, text) = self.tokens.pop().unwrap();
        self.token(kind.into(), text);
    }

    /// Peek at the first unprocessed token
    fn current(&self) -> Option<SyntaxKind> {
        self.tokens.last().map(|(kind, _)| *kind)
    }

    fn skip_ws(&mut self) {
        while self.current() == Some(WHITESPACE) {
            self.bump()
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        let kind = VimscriptLang::kind_to_raw(kind);
        self.builder.start_node(kind);
    }

    pub fn token(&mut self, kind: SyntaxKind, text: SmolStr) {
        let kind = VimscriptLang::kind_to_raw(kind);
        self.builder.token(kind, text)
    }
}

#[cfg(test)]
mod tests;
