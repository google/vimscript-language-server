use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::SmolStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    EQ,
    PLUS,
    IDENT,

    LET_KW,

    WHITESPACE,
    // We use this because in vimscript new lines are important (end of statement).
    NEW_LINE,
    // Do I need this?
    EOF,
    ERROR,

    LET_STMT,

    IDENT_EXPR,

    ROOT,
}
use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum VimscriptLang {}
impl rowan::Language for VimscriptLang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
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

pub fn parse(source: &str) -> Parse {
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
        self.builder.start_node(ROOT.into());

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
        self.builder.start_node(LET_STMT.into());
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
        self.builder.start_node(IDENT_EXPR.into());
        self.bump();
        self.builder.finish_node();
    }

    // TODO: simplify error handling

    // TODO: bump_any, bump based on kind, etc.
    fn bump(&mut self) {
        let (kind, text) = self.tokens.pop().unwrap();
        self.builder.token(kind.into(), text);
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
}

#[cfg(test)]
mod tests;
