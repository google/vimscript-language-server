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

use crate::ast::Stmt;
use crate::ast::StmtKind;
use crate::ast::TryStatement;
use crate::lexer::TokenType;
use crate::parser::Parser;

pub fn parse(parser: &mut Parser) -> Option<TryStatement> {
    parser.expect_end_of_statement()?;
    let body = parse_statements_until(parser, |t| {
        return t == TokenType::EndTry || t == TokenType::Finally;
    });
    match parser.peek_token().token_type {
        TokenType::EndTry => {
            parser.advance();
            parser.expect_end_of_statement()?;
            return Some(TryStatement {
                body: body,
                finally: None,
            });
        }
        TokenType::Finally => {
            parser.advance();
            parser.expect_end_of_statement()?;
            let finally = parser.parse_statements_until(TokenType::EndTry)?;
            return Some(TryStatement {
                body: body,
                finally: Some(finally),
            });
        }
        _ => {
            parser.expect_token(TokenType::EndTry);
            return None;
        }
    }
}

fn parse_statements_until<F>(parser: &mut Parser, predicate: F) -> Vec<Stmt>
where
    F: Fn(TokenType) -> bool,
{
    let mut stmts = Vec::new();
    while parser.peek_token().token_type != TokenType::Eof
        && !predicate(parser.peek_token().token_type)
    {
        if let Some(stmt) = parser.parse_statement() {
            stmts.push(stmt);
        }
    }
    return stmts;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn parses_try_statement() {
        let mut parser = Parser::new(Lexer::new(
            "
             try
                 call my#method()
             endtry
             ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "try": {
                    "body": [{
                        "call": {
                            "method": "my#method",
                            "arguments": [],
                        }
                    }],
                },
            }])
        );
    }

    #[test]
    fn parses_try_statement_with_finally() {
        let mut parser = Parser::new(Lexer::new(
            "
             try
                 call my#foo()
             finally
                 call my#bar()
             endtry
             ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "try": {
                    "body": [{
                        "call": {
                            "method": "my#foo",
                            "arguments": [],
                        }
                    }],
                    "finally": [{
                        "call": {
                            "method": "my#bar",
                            "arguments": [],
                        }
                    }],
                },
            }])
        );
    }
}
