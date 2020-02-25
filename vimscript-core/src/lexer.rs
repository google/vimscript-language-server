// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::peekable_chars_with_position::PeekableCharsWithPosition;
use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Let,
    Assign,
    Ident,
    Number,
    // :help string-literal
    // :help expr-quote
    StringLiteral,
    Function,
    EndFunction,
    If,
    Else,
    ElseIf,
    EndIf,
    Try,
    Catch,
    Finally,
    EndTry,
    // ()
    LeftParenthesis,
    RightParenthesis,
    // []
    LeftBracket,
    RightBracket,
    // {}
    LeftCurlyBrace,
    RightCurlyBrace,
    Colon,
    QuestionMark,
    Bang,
    Comma,
    Set,
    For,
    EndFor,
    While,
    EndWhile,
    In,
    Dot,
    Variadic,
    Abort,
    Call,
    Break,
    Execute,
    Return,
    EqualCaseSensitive,
    InEqualCaseSensitive,
    EqualCaseInSensitive,
    InEqualCaseInSensitive,
    Equal,
    InEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    And,
    Or,
    Pipe,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    DotAssign,
    NewLine,
    Invalid,
    Eof,
}

impl TokenType {
    pub fn to_str(&self) -> &str {
        match self {
            TokenType::Let => "`let`",
            TokenType::Assign => "=",
            TokenType::Ident => "identifier",
            TokenType::Number => "number",
            TokenType::StringLiteral => "string literal",
            TokenType::Function => "`function`",
            TokenType::EndFunction => "`endfunction`",
            TokenType::If => "`if`",
            TokenType::Else => "`else`",
            TokenType::ElseIf => "`elseif`",
            TokenType::EndIf => "`endif`",
            TokenType::Try => "`try`",
            TokenType::Catch => "`catch`",
            TokenType::Finally => "`finally`",
            TokenType::EndTry => "`endtry`",
            TokenType::LeftParenthesis => "`(`",
            TokenType::RightParenthesis => "`)`",
            TokenType::LeftBracket => "`[`",
            TokenType::RightBracket => "`]`",
            TokenType::LeftCurlyBrace => "`{`",
            TokenType::RightCurlyBrace => "`}`",
            TokenType::Bang => "`!`",
            TokenType::Colon => "`:`",
            TokenType::QuestionMark => "`?`",
            TokenType::Comma => "`,`",
            TokenType::Set => "`set`",
            TokenType::For => "`for`",
            TokenType::EndFor => "`endfor`",
            TokenType::While => "`while`",
            TokenType::EndWhile => "`endwhile`",
            TokenType::In => "`in`",
            TokenType::Dot => "`.`",
            TokenType::Variadic => "`...`",
            TokenType::Abort => "`abort`",
            TokenType::Call => "`call`",
            TokenType::Break => "`break`",
            TokenType::Execute => "`execute`",
            TokenType::Return => "`return`",
            TokenType::EqualCaseSensitive => "`==#`",
            TokenType::InEqualCaseSensitive => "`!=#`",
            TokenType::EqualCaseInSensitive => "`==?`",
            TokenType::InEqualCaseInSensitive => "`!=?`",
            TokenType::Equal => "`==`",
            TokenType::InEqual => "`!=`",
            TokenType::Less => "`<`",
            TokenType::LessOrEqual => "`<=`",
            TokenType::Greater => "`>`",
            TokenType::GreaterOrEqual => "`>=`",
            TokenType::And => "`&&`",
            TokenType::Or => "`||`",
            TokenType::Pipe => "`|`",
            TokenType::Plus => "`+`",
            TokenType::Minus => "`-`",
            TokenType::Multiply => "`*`",
            TokenType::Divide => "`/`",
            TokenType::Modulo => "`%`",
            TokenType::PlusAssign => "`+=`",
            TokenType::MinusAssign => "`-=`",
            TokenType::MultiplyAssign => "`*=`",
            TokenType::DivideAssign => "`/=`",
            TokenType::ModuloAssign => "`%=`",
            TokenType::DotAssign => "`.=`",
            TokenType::NewLine => "new line",
            TokenType::Invalid => "invalid",
            TokenType::Eof => "end of file",
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            TokenType::Let => "`let`",
            TokenType::Assign => "`=`",
            TokenType::Ident => "identifier",
            TokenType::Number => "number",
            TokenType::StringLiteral => "string literal",
            TokenType::Function => "`function`",
            TokenType::EndFunction => "`endfunction`",
            TokenType::If => "`if`",
            TokenType::Else => "`else`",
            TokenType::ElseIf => "`elseif`",
            TokenType::EndIf => "`endif`",
            TokenType::Try => "`try`",
            TokenType::Catch => "`catch`",
            TokenType::Finally => "`finally`",
            TokenType::EndTry => "`endtry`",
            TokenType::LeftParenthesis => "`(`",
            TokenType::RightParenthesis => "`)`",
            TokenType::LeftBracket => "`[`",
            TokenType::RightBracket => "`]`",
            TokenType::LeftCurlyBrace => "`{`",
            TokenType::RightCurlyBrace => "`}`",
            TokenType::Bang => "`!`",
            TokenType::Colon => "`:`",
            TokenType::QuestionMark => "`?`",
            TokenType::Comma => "`,`",
            TokenType::Set => "`set`",
            TokenType::For => "`for`",
            TokenType::EndFor => "`endfor`",
            TokenType::While => "`while`",
            TokenType::EndWhile => "`endwhile`",
            TokenType::In => "`in`",
            TokenType::Dot => "`.`",
            TokenType::Variadic => "`...`",
            TokenType::Abort => "`abort`",
            TokenType::Call => "`call`",
            TokenType::Break => "`break`",
            TokenType::Execute => "`execute`",
            TokenType::Return => "`return`",
            TokenType::EqualCaseSensitive => "`==#`",
            TokenType::InEqualCaseSensitive => "`!=#`",
            TokenType::EqualCaseInSensitive => "`==?`",
            TokenType::InEqualCaseInSensitive => "`!=?`",
            TokenType::Equal => "`==`",
            TokenType::InEqual => "`!=`",
            TokenType::Less => "`<`",
            TokenType::LessOrEqual => "`<=`",
            TokenType::Greater => "`>`",
            TokenType::GreaterOrEqual => "`>=`",
            TokenType::And => "`&&`",
            TokenType::Or => "`||`",
            TokenType::Pipe => "`|`",
            TokenType::Plus => "`+`",
            TokenType::Minus => "`-`",
            TokenType::Multiply => "`*`",
            TokenType::Divide => "`/`",
            TokenType::Modulo => "`%`",
            TokenType::PlusAssign => "`+=`",
            TokenType::MinusAssign => "`-=`",
            TokenType::MultiplyAssign => "`*=`",
            TokenType::DivideAssign => "`/=`",
            TokenType::ModuloAssign => "`%=`",
            TokenType::DotAssign => "`.=`",
            TokenType::NewLine => "new line",
            TokenType::Invalid => "invalid",
            TokenType::Eof => "end of file",
        }
    }
}

// Location in a source code (most of the time point to the start of the token).
#[derive(PartialEq, Debug, Clone)]
pub struct SourceLocation {
    range: std::ops::Range<usize>,
}

#[derive(PartialEq, Debug)]
pub struct TokenPosition {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

// Position with zero based offsets, points to position just before the character.
#[derive(PartialEq, PartialOrd, Debug)]
pub struct SourcePosition {
    // starting from 0
    pub line: i32,
    // starting from 0
    pub character: i32,
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.character)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub location: SourceLocation,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: PeekableCharsWithPosition<'a>,
    tokens: Vec<Token>,
    // The position of the start of the current token.
    start: usize,
    first_token_in_line: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        return Lexer {
            source: source,
            chars: PeekableCharsWithPosition::new(source),
            start: 0,
            tokens: Vec::new(),
            first_token_in_line: true,
        };
    }
    // TODO: remove this method once Lexer always returns Eof as last token.
    pub fn eof_token(&self) -> Token {
        return Token {
            token_type: TokenType::Eof,
            location: SourceLocation {
                range: self.source.len()..self.source.len(),
            },
        };
    }

    pub fn token_text(&self, location: &SourceLocation) -> &'a str {
        return &self.source[location.range.clone()];
    }

    // This is expensive, expected to be called only for errors.
    fn source_position(&self, location: usize) -> SourcePosition {
        let mut line = 0;
        let mut character = 0;
        for (pos, c) in self.source.char_indices() {
            if pos >= location {
                return SourcePosition {
                    line: line,
                    character: character,
                };
            }
            character += 1;
            if c == '\n' {
                line += 1;
                character = 0;
            }
        }
        return SourcePosition {
            line: line,
            character: character,
        };
    }

    // This is expensive, expected to be called only for errors.
    pub fn token_position(&self, location: &SourceLocation) -> TokenPosition {
        return TokenPosition {
            start: self.source_position(location.range.start),
            end: self.source_position(location.range.end),
        };
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.read_token() {
            self.start = self.chars.pos();
        }
        return std::mem::replace(&mut self.tokens, Vec::new());
    }

    fn read_token(&mut self) -> bool {
        match self.chars.next() {
            None => return false,
            Some('\n') => self.read_newline(),
            Some('(') => self.add_token(TokenType::LeftParenthesis),
            Some(')') => self.add_token(TokenType::RightParenthesis),
            Some('[') => self.add_token(TokenType::LeftBracket),
            Some(']') => self.add_token(TokenType::RightBracket),
            Some('{') => self.add_token(TokenType::LeftCurlyBrace),
            Some('}') => self.add_token(TokenType::RightCurlyBrace),
            Some(',') => self.add_token(TokenType::Comma),
            Some(':') => self.add_token(TokenType::Colon),
            Some('?') => self.add_token(TokenType::QuestionMark),
            Some('+') => self.read_math_operator(TokenType::Plus, TokenType::PlusAssign),
            Some('-') => self.read_math_operator(TokenType::Minus, TokenType::MinusAssign),
            Some('*') => self.read_math_operator(TokenType::Multiply, TokenType::MultiplyAssign),
            Some('/') => self.read_math_operator(TokenType::Divide, TokenType::DivideAssign),
            Some('%') => self.read_math_operator(TokenType::Modulo, TokenType::ModuloAssign),
            Some('.') => self.read_dot(),
            Some('\'') => self.read_string_literal(),
            Some('=') => self.read_equal(),
            Some('!') => self.read_in_equal(),
            Some('<') => self.read_less(),
            Some('>') => self.read_greater(),
            Some('&') => self.read_and(),
            Some('|') => self.read_pipe(),
            Some('"') => self.read_quote(),
            Some(' ') => {}
            Some(c) => {
                if '0' <= c && c <= '9' {
                    self.read_number();
                } else {
                    self.read_identifier();
                }
            }
        }
        return true;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type: token_type,
            location: SourceLocation {
                range: self.start..self.chars.pos(),
            },
        });
        self.first_token_in_line = token_type == TokenType::NewLine
    }

    fn read_math_operator(&mut self, op: TokenType, assign: TokenType) {
        if Some('=') == self.chars.peek() {
            self.chars.next();
            self.add_token(assign);
        } else {
            self.add_token(op);
        }
    }

    fn read_newline(&mut self) {
        let token = Token {
            token_type: TokenType::NewLine,
            location: SourceLocation {
                range: self.start..self.chars.pos(),
            },
        };
        loop {
            match self.chars.peek() {
                Some(' ') => {
                    self.chars.next();
                }
                Some('\t') => {
                    self.chars.next();
                }
                Some('\\') => {
                    self.chars.next();
                    return;
                }
                _ => {
                    self.tokens.push(token);
                    self.first_token_in_line = true;
                    return;
                }
            }
        }
    }

    fn read_less(&mut self) {
        match self.chars.peek() {
            Some('=') => {
                self.chars.next();
                self.add_token(TokenType::LessOrEqual);
            }
            _ => self.add_token(TokenType::Less),
        }
    }

    fn read_greater(&mut self) {
        match self.chars.peek() {
            Some('=') => {
                self.chars.next();
                self.add_token(TokenType::GreaterOrEqual);
            }
            _ => self.add_token(TokenType::Greater),
        }
    }

    fn read_and(&mut self) {
        match self.chars.peek() {
            Some('&') => {
                self.chars.next();
                self.add_token(TokenType::And);
            }
            _ => {
                self.read_identifier();
            }
        }
    }

    fn read_equal(&mut self) {
        match self.chars.peek() {
            Some('=') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('#') => {
                        self.chars.next();
                        self.add_token(TokenType::EqualCaseSensitive);
                    }
                    Some('?') => {
                        self.chars.next();
                        self.add_token(TokenType::EqualCaseInSensitive);
                    }
                    _ => {
                        self.add_token(TokenType::Equal);
                    }
                }
            }
            _ => self.add_token(TokenType::Assign),
        }
    }

    fn read_in_equal(&mut self) {
        match self.chars.peek() {
            Some('=') => {
                self.chars.next();
                match self.chars.peek() {
                    Some('#') => {
                        self.chars.next();
                        self.add_token(TokenType::InEqualCaseSensitive);
                    }
                    Some('?') => {
                        self.chars.next();
                        self.add_token(TokenType::InEqualCaseInSensitive);
                    }
                    _ => {
                        self.add_token(TokenType::InEqual);
                    }
                }
            }
            _ => self.add_token(TokenType::Bang),
        }
    }

    fn read_dot(&mut self) {
        match self.chars.peek() {
            None => self.add_token(TokenType::Dot),
            Some('.') => {
                self.chars.next();
                if let Some('.') = self.chars.peek() {
                    self.chars.next();
                    self.add_token(TokenType::Variadic);
                    return;
                } else {
                    self.add_token(TokenType::Invalid);
                }
            }
            Some('=') => {
                self.chars.next();
                self.add_token(TokenType::DotAssign);
            }
            Some(_) => self.add_token(TokenType::Dot),
        }
    }

    fn read_string_literal(&mut self) {
        loop {
            match self.chars.next() {
                None => {
                    self.add_token(TokenType::Invalid);
                    return;
                }
                Some('\'') => {
                    if self.chars.peek() == Some('\'') {
                        self.chars.next();
                        continue;
                    }
                    break;
                }
                Some('\n') => {
                    // Next line has to start with a backslash (with allowed spaces before).
                    // TODO: how can we report the error nicely here?
                    loop {
                        match self.chars.peek() {
                            Some(' ') => {
                                self.chars.next();
                            }
                            Some('\\') => {
                                self.chars.next();
                                break;
                            }
                            _ => {
                                self.add_token(TokenType::Invalid);
                                return;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        self.add_token(TokenType::StringLiteral)
    }

    fn read_quote(&mut self) {
        // TODO: handle proper escaping.
        let mut escaped = false;
        loop {
            match self.chars.peek() {
                None => return,
                Some('\\') => {
                    self.chars.next();
                    escaped = !escaped;
                }
                Some('"') => {
                    self.chars.next();
                    if !self.first_token_in_line && !escaped {
                        self.add_token(TokenType::StringLiteral);
                        return;
                    }
                }
                Some('\n') => {
                    return;
                }
                _ => {
                    self.chars.next();
                    escaped = false;
                }
            }
        }
    }

    fn read_pipe(&mut self) {
        if self.chars.peek() == Some('|') {
            self.chars.next();
            self.add_token(TokenType::Or);
            return;
        }
        self.add_token(TokenType::Pipe);
    }

    fn read_number(&mut self) {
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
        self.add_token(TokenType::Number);
    }

    fn read_identifier(&mut self) {
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
        self.add_token(match s {
            "let" => TokenType::Let,
            "function" => TokenType::Function,
            "endfunction" => TokenType::EndFunction,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "elseif" => TokenType::ElseIf,
            "endif" => TokenType::EndIf,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "finally" => TokenType::Finally,
            "endtry" => TokenType::EndTry,
            "for" => TokenType::For,
            "set" => TokenType::Set,
            "endfor" => TokenType::EndFor,
            "while" => TokenType::While,
            "endwhile" => TokenType::EndWhile,
            "in" => TokenType::In,
            "call" => TokenType::Call,
            "break" => TokenType::Break,
            "execute" => TokenType::Execute,
            "return" => TokenType::Return,
            "abort" => TokenType::Abort,
            _ => TokenType::Ident,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    fn parse_source(source: &str) -> Vec<(TokenType, &str)> {
        let mut lexer = Lexer::new(source);
        return lexer
            .lex()
            .into_iter()
            .map(|t| (t.token_type, lexer.token_text(&t.location)))
            .collect();
    }

    #[test]
    fn parses_sample_vimscript() {
        assert_eq!(
            parse_source(
                "let s:enabled = 15
                 function! plug#name(...) abort
                   if a:test_123
                   endif
                 endfunction"
            ),
            &[
                (TokenType::Let, "let"),
                (TokenType::Ident, "s:enabled"),
                (TokenType::Assign, "="),
                (TokenType::Number, "15"),
                (TokenType::NewLine, "\n"),
                (TokenType::Function, "function"),
                (TokenType::Bang, "!"),
                (TokenType::Ident, "plug#name"),
                (TokenType::LeftParenthesis, "("),
                (TokenType::Variadic, "..."),
                (TokenType::RightParenthesis, ")"),
                (TokenType::Abort, "abort"),
                (TokenType::NewLine, "\n"),
                (TokenType::If, "if"),
                (TokenType::Ident, "a:test_123"),
                (TokenType::NewLine, "\n"),
                (TokenType::EndIf, "endif"),
                (TokenType::NewLine, "\n"),
                (TokenType::EndFunction, "endfunction"),
            ],
        );
    }

    #[test]
    fn parses_concatenation_of_string_literals() {
        assert_eq!(
            parse_source("'a' . 'b'"),
            &[
                (TokenType::StringLiteral, "'a'"),
                (TokenType::Dot, "."),
                (TokenType::StringLiteral, "'b'"),
            ],
        );
    }

    #[test]
    fn parses_comma_colon_question_mark() {
        assert_eq!(
            parse_source(", : ?"),
            &[
                (TokenType::Comma, ","),
                (TokenType::Colon, ":"),
                (TokenType::QuestionMark, "?"),
            ],
        );
    }

    #[test]
    fn parses_pipe_and_or() {
        assert_eq!(
            parse_source("| ||"),
            &[(TokenType::Pipe, "|"), (TokenType::Or, "||"),]
        );
    }

    #[test]
    fn parses_reserved_words() {
        assert_eq!(
            parse_source("let return execute call break set for endfor while endwhile if endif function endfunction abort in"),
            &[
                (TokenType::Let, "let"),
                (TokenType::Return, "return"),
                (TokenType::Execute, "execute"),
                (TokenType::Call, "call"),
                (TokenType::Break, "break"),
                (TokenType::Set, "set"),
                (TokenType::For, "for"),
                (TokenType::EndFor, "endfor"),
                (TokenType::While, "while"),
                (TokenType::EndWhile, "endwhile"),
                (TokenType::If, "if"),
                (TokenType::EndIf, "endif"),
                (TokenType::Function, "function"),
                (TokenType::EndFunction, "endfunction"),
                (TokenType::Abort, "abort"),
                (TokenType::In, "in"),
            ],
        );
    }

    #[test]
    fn parses_for_statement() {
        assert_eq!(
            parse_source("for [a, b] in items endfor",),
            &[
                (TokenType::For, "for"),
                (TokenType::LeftBracket, "["),
                (TokenType::Ident, "a"),
                (TokenType::Comma, ","),
                (TokenType::Ident, "b"),
                (TokenType::RightBracket, "]"),
                (TokenType::In, "in"),
                (TokenType::Ident, "items"),
                (TokenType::EndFor, "endfor"),
            ]
        );
    }

    #[test]
    fn parses_quote_expression() {
        assert_eq!(
            parse_source("endif \"some\""),
            &[
                (TokenType::EndIf, "endif"),
                (TokenType::StringLiteral, "\"some\"")
            ]
        );
    }

    #[test]
    fn skips_comments() {
        assert_eq!(
            parse_source(",\" some comment\n="),
            &[
                (TokenType::Comma, ","),
                (TokenType::NewLine, "\n"),
                (TokenType::Assign, "=")
            ],
        );
    }

    #[test]
    fn returns_no_tokens_when_input_is_empty() {
        assert_eq!(parse_source(""), &[])
    }

    #[test]
    fn parses_string_literals() {
        assert_eq!(
            parse_source("'That''s enough.'"),
            &[(TokenType::StringLiteral, "'That''s enough.'"),]
        )
    }

    #[test]
    fn parses_string_literals_with_new_lines() {
        assert_eq!(
            parse_source("'That\n \\is valid literal'"),
            &[(TokenType::StringLiteral, "'That\n \\is valid literal'"),]
        )
    }

    #[test]
    fn returns_invalid_string_for_multi_line_literal_without_backslash() {
        assert_eq!(
            parse_source("'That\n '"),
            &[(TokenType::Invalid, "'That\n "), (TokenType::Invalid, "'"),]
        )
    }

    #[test]
    fn parses_comparison_operators() {
        assert_eq!(
            parse_source("==# !=# ==? !=? == != < <= > >= &&"),
            &[
                (TokenType::EqualCaseSensitive, "==#"),
                (TokenType::InEqualCaseSensitive, "!=#"),
                (TokenType::EqualCaseInSensitive, "==?"),
                (TokenType::InEqualCaseInSensitive, "!=?"),
                (TokenType::Equal, "=="),
                (TokenType::InEqual, "!="),
                (TokenType::Less, "<"),
                (TokenType::LessOrEqual, "<="),
                (TokenType::Greater, ">"),
                (TokenType::GreaterOrEqual, ">="),
                (TokenType::And, "&&"),
            ],
        )
    }

    #[test]
    fn parses_math_operators() {
        assert_eq!(
            parse_source("+ += - -= * *= / /= % %= . .="),
            &[
                (TokenType::Plus, "+"),
                (TokenType::PlusAssign, "+="),
                (TokenType::Minus, "-"),
                (TokenType::MinusAssign, "-="),
                (TokenType::Multiply, "*"),
                (TokenType::MultiplyAssign, "*="),
                (TokenType::Divide, "/"),
                (TokenType::DivideAssign, "/="),
                (TokenType::Modulo, "%"),
                (TokenType::ModuloAssign, "%="),
                (TokenType::Dot, "."),
                (TokenType::DotAssign, ".="),
            ],
        )
    }

    #[test]
    fn parses_two_string_literals() {
        assert_eq!(
            parse_source(r#"endif "foo" "bar""#),
            &[
                (TokenType::EndIf, "endif"),
                (TokenType::StringLiteral, "\"foo\""),
                (TokenType::StringLiteral, "\"bar\""),
            ],
        )
    }

    #[test]
    fn parses_identifier_with_capital_letter() {
        assert_eq!(
            parse_source(r#"s:Length"#),
            &[(TokenType::Ident, "s:Length"),],
        )
    }

    #[test]
    fn parses_identifier_with_ampersand() {
        assert_eq!(parse_source(r#"&paste"#), &[(TokenType::Ident, "&paste"),],)
    }

    #[test]
    fn parses_string_literal_with_escaped_quote() {
        assert_eq!(
            parse_source(r#"endif "\"foo""#),
            &[
                (TokenType::EndIf, "endif"),
                (TokenType::StringLiteral, r#""\"foo""#),
            ],
        )
    }

    #[test]
    fn parses_comment_with_quotes_in_it() {
        assert_eq!(parse_source(r#"" This is comment with "quotes""#), &[])
    }

    #[test]
    fn includes_new_line_after_comment() {
        assert_eq!(
            parse_source("\"comment\nendif"),
            &[(TokenType::NewLine, "\n"), (TokenType::EndIf, "endif"),]
        )
    }

    #[test]
    fn parses_line_breaks() {
        assert_eq!(
            parse_source("a + \n \t\\ b"),
            &[
                (TokenType::Ident, "a"),
                (TokenType::Plus, "+"),
                (TokenType::Ident, "b"),
            ]
        )
    }

    #[test]
    fn parses_try_catch_keywords() {
        assert_eq!(
            parse_source("try catch finally endtry"),
            &[
                (TokenType::Try, "try"),
                (TokenType::Catch, "catch"),
                (TokenType::Finally, "finally"),
                (TokenType::EndTry, "endtry"),
            ]
        )
    }

    #[test]
    fn returns_correct_token_position() {
        let mut lexer = Lexer::new("unknown");
        let tokens = lexer.lex();
        let position = lexer.token_position(&tokens[0].location);
        assert_eq!(
            position,
            TokenPosition {
                start: SourcePosition {
                    line: 0,
                    character: 0
                },
                end: SourcePosition {
                    line: 0,
                    character: 7
                },
            }
        )
    }
}
