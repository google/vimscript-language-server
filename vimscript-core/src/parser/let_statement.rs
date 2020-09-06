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

use crate::ast::LetStatement;
use crate::lexer::TokenType;
use crate::parser::ParseError;
use crate::parser::Parser;

fn is_assign_operator(token_type: TokenType) -> bool {
    match token_type {
        TokenType::Assign => true,
        TokenType::PlusAssign => true,
        TokenType::MinusAssign => true,
        TokenType::MultiplyAssign => true,
        TokenType::DivideAssign => true,
        TokenType::ModuloAssign => true,
        TokenType::DotAssign => true,
        _ => false,
    }
}

// Let = 'let' VarName = Expression (NewLine | EOF)
pub fn parse(parser: &mut Parser) -> Option<LetStatement> {
    // TODO: This is not really correct, as only some expressions like ident, array subscript and
    // array are supported here.
    let var = parser.parse_expression()?;

    let operator = parser.peek_token();
    if !is_assign_operator(operator.token_type) {
        parser.errors.push(ParseError {
            message: format!(
                "expected assign operator, found {}",
                parser.token_text(&operator)
            ),
            position: parser.l.token_position(&operator.location),
        });
        parser.consume_until_end_of_statement();
        // TODO: error
        return None;
    }
    parser.advance();

    let expr = parser.parse_expression()?;

    parser.expect_end_of_statement()?;

    return Some(LetStatement {
        var: Box::new(var),
        operator: operator.token_type,
        value: Box::new(expr),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::StmtKind;
    use crate::lexer::Lexer;
    use crate::lexer::SourcePosition;
    use crate::lexer::TokenPosition;
    use crate::parser::ParseError;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn parses_simple_let_statement() {
        let mut parser = Parser::new(Lexer::new("let l:var = 15"));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "let": {
                    "var": {"identifier": "l:var"},
                    "operator": "`=`",
                    "value": {
                        "number": 15.0,
                    },
                },
            }])
        );
    }

    #[test]
    fn parses_let_statement_with_different_operators() {
        let mut parser = Parser::new(Lexer::new("let l:var += 15"));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "let": {
                    "var": {"identifier": "l:var"},
                    "operator": "`+=`",
                    "value": {
                        "number": 15.0,
                    },
                },
            }])
        );
    }

    #[test]
    fn parses_let_statement_with_number_expression() {
        let mut parser = Parser::new(Lexer::new("let l:var = 15"));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);

        assert_eq!(program.statements.len(), 1);
        let let_stmt = match &program.statements[0].kind {
            StmtKind::Let(stmt) => stmt,
            stmt => panic!(format!("expected let statement, got {:?}", stmt)),
        };
        // assert_eq!(let_stmt.name(), "l:var");
        assert_eq!(let_stmt.value.to_string(), "15");
        // assert_eq!(
        //     parser
        //         .resolve_location(let_stmt.name_location().clone())
        //         .to_string(),
        //     "0:4-0:9"
        // );
    }

    #[test]
    fn returns_error_when_let_statement_is_missing_assign() {
        let mut parser = Parser::new(Lexer::new("let l:var ! 15"));
        parser.parse();
        assert_eq!(
            parser.errors,
            &[ParseError {
                message: "expected assign operator, found `!`".to_string(),
                position: TokenPosition {
                    start: SourcePosition {
                        line: 0,
                        character: 10,
                    },
                    end: SourcePosition {
                        line: 0,
                        character: 11,
                    },
                }
            }]
        );
    }

    #[test]
    fn returns_error_when_let_statement_ends_after_identifier() {
        let mut parser = Parser::new(Lexer::new("let l:var\nlet l:var = 15"));
        let program = parser.parse();
        assert_eq!(
            parser.errors,
            &[ParseError {
                message: "expected assign operator, found new line".to_string(),
                position: TokenPosition {
                    start: SourcePosition {
                        line: 0,
                        character: 9,
                    },
                    end: SourcePosition {
                        line: 1,
                        character: 0,
                    },
                }
            }]
        );
        assert_eq!(program.statements.len(), 1);
        let let_stmt = match &program.statements[0].kind {
            StmtKind::Let(stmt) => stmt,
            stmt => panic!(format!("expected let statement, got {:?}", stmt)),
        };
        // assert_eq!(let_stmt.name(), "l:var");
        assert_eq!(let_stmt.value.to_string(), "15");
    }

    #[test]
    fn returns_error_when_let_statement_is_missing_identifier() {
        let mut parser = Parser::new(Lexer::new("let\n"));
        parser.parse();
        let error_messages: Vec<String> =
            parser.errors.into_iter().map(|err| err.message).collect();
        // TODO: should be `expected variable`
        assert_eq!(error_messages, &["expected expression, found new line"],);
    }

    #[test]
    fn returns_error_when_let_statement_has_more_tokens_after_expression() {
        let mut parser = Parser::new(Lexer::new("let a = 'b' a\n"));
        parser.parse();
        let error_messages: Vec<String> =
            parser.errors.into_iter().map(|err| err.message).collect();
        assert_eq!(error_messages, &["expected new line, found `a`"],);
    }

    #[test]
    fn returns_error_when_expression_is_invalid() {
        let mut parser = Parser::new(Lexer::new("let a = 'b' .\n"));
        parser.parse();
        let error_messages: Vec<String> =
            parser.errors.into_iter().map(|err| err.message).collect();
        assert_eq!(error_messages, &["expected expression, found new line"],);
    }

    #[test]
    fn returns_error_when_let_statement_expression_() {
        let mut parser = Parser::new(Lexer::new("let a = 'b' a\n"));
        parser.parse();
        let error_messages: Vec<String> =
            parser.errors.into_iter().map(|err| err.message).collect();
        assert_eq!(error_messages, &["expected new line, found `a`"],);
    }
}
