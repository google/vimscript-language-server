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

use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;
use std::iter::IntoIterator;

pub struct TokenIterator {
    index: usize,
    tokens: Vec<Token>,
}

impl TokenIterator {
    // It is required for tokens to have at least one element,
    // and it is expected that last token is EOF.
    pub fn new(tokens: Vec<Token>) -> TokenIterator {
        if tokens.len() == 0 {
            panic!("cannot create TokenIterator from empty tokens")
        }
        TokenIterator {
            index: 0,
            tokens: tokens,
        }
    }

    pub fn next(&mut self) -> &Token {
        let index = self.index;
        self.advance();
        return &self.tokens[index];
    }

    pub fn peek(&mut self) -> &Token {
        return &self.tokens[self.index];
    }

    pub fn advance(&mut self) {
        if self.index + 1 < self.tokens.len() {
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_returns_eof_for_empty_iterator() {
        let mut lexer = Lexer::new("");
        let mut tokens = lexer.lex();
        tokens.push(lexer.eof_token());
        let mut iter = TokenIterator::new(tokens);
        assert_eq!(iter.next().token_type, TokenType::Eof);
    }

    #[test]
    fn next_returns_next_token_and_advances() {
        let mut lexer = Lexer::new("for in");
        let mut tokens = lexer.lex();
        tokens.push(lexer.eof_token());
        let mut iter = TokenIterator::new(tokens);
        assert_eq!(iter.next().token_type, TokenType::For);
        assert_eq!(iter.next().token_type, TokenType::In);
        assert_eq!(iter.next().token_type, TokenType::Eof);
    }

    #[test]
    fn peek_returns_next_token_without_advancing() {
        let mut lexer = Lexer::new("for in");
        let mut tokens = lexer.lex();
        tokens.push(lexer.eof_token());
        let mut iter = TokenIterator::new(tokens);
        assert_eq!(iter.peek().token_type, TokenType::For);
        assert_eq!(iter.peek().token_type, TokenType::For);
    }
}
