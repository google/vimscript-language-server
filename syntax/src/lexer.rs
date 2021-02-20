use rowan::TextSize;
use parser::syntax_kind::SyntaxKind;
use SyntaxKind::*;
use vimscript_core::peekable_chars_with_position::PeekableCharsWithPosition;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: TextSize,
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = Lexer {
        source: source,
        chars: PeekableCharsWithPosition::new(source),
        tokens: Vec::new(),
        start: 0,
    };
    lexer.lex()
}

struct Lexer<'a> {
    source: &'a str,
    chars: PeekableCharsWithPosition<'a>,
    tokens: Vec<Token>,
    start: usize,
}

impl<'a> Lexer<'a> {
    fn lex(&mut self) -> Vec<Token> {
        while let Some(kind) = self.read_token() {
            let len = TextSize::try_from(self.chars.pos() - self.start).unwrap();
            self.tokens.push(Token{kind: kind, len: len});
            self.start = self.chars.pos();
        }
        return std::mem::replace(&mut self.tokens, Vec::new());
    }

    fn read_token(&mut self) -> Option<SyntaxKind> {
        match self.chars.next() {
            None => None,
            Some('\n') => Some(NEW_LINE),
            Some(' ') => Some(WHITESPACE),
            Some('=') => Some(EQ),
            Some(c) => {
                if '0' <= c && c <= '9' {
                    Some(self.read_number())
                } else {
                    Some(self.read_identifier())
                }
            }
        }
    }

    fn read_identifier(&mut self) -> SyntaxKind {
        loop {
            match self.chars.peek() {
                None => break,
                Some(c) => {
                    if !(('a' <= c && c <= 'z')
                        || ('A' <= c && c <= 'Z')
                        || c == '#'
                        || c == ':'
                        || c == '_'
                        || ('0' <= c && c <= '9'))
                    {
                        break;
                    }
                }
            }
            self.chars.next();
        }
        let s = &self.source[self.start..self.chars.pos()];
        match s {
            "let" => LET_KW,
            _ => IDENT,
        }
    }

    fn read_number(&mut self) -> SyntaxKind {
        // TODO: handle floating point numbers.
        loop {
            match self.chars.peek() {
                None => break,
                Some(c) => {
                    if !('0' <= c && c <= '9') {
                        break;
                    }
                }
            }
            self.chars.next();
        }
        return NUMBER
    }
}
