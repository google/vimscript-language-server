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

use crate::ast::*;
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::span::BytePos;
use crate::span::Span;
use std::convert::TryInto;

pub fn parse(parser: &mut Parser) -> Option<Expr> {
    let mut left = parse_prefix_expression(parser)?;

    loop {
        let peek_type = parser.peek_token().token_type;
        if peek_type == TokenType::QuestionMark {
            parser.advance();
            let lhs = parse(parser)?;
            parser.expect_token(TokenType::Colon)?;
            let rhs = parse(parser)?;
            return Some(Expr {
                span: Span {
                    start: lhs.span.start,
                    end: rhs.span.end,
                },
                kind: ExprKind::Choose(ChooseExpression {
                    cond: Box::new(left),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }),
            });
        }
        if !is_operator(peek_type) {
            break;
        }
        parser.advance();
        let right = parse_prefix_expression(parser)?;
        left = Expr {
            span: Span {
                start: left.span.start,
                end: right.span.end,
            },
            kind: ExprKind::Infix(InfixExpression {
                left: Box::new(left),
                operator: peek_type,
                right: Box::new(right),
            }),
        }
    }
    return Some(left);
}

// Returns true if this token is an operator that can be between two expressions.
fn is_operator(token_type: TokenType) -> bool {
    match token_type {
        TokenType::Equal => true,
        TokenType::InEqual => true,
        TokenType::InEqualCaseSensitive => true,
        TokenType::InEqualCaseInSensitive => true,
        TokenType::EqualCaseSensitive => true,
        TokenType::EqualCaseInSensitive => true,
        TokenType::Less => true,
        TokenType::LessOrEqual => true,
        TokenType::Greater => true,
        TokenType::GreaterOrEqual => true,
        TokenType::RegexpMatchesIgnoreCase => true,
        TokenType::RegexpMatchesCaseSensitive => true,
        TokenType::RegexpMatchesCaseInSensitive => true,
        TokenType::Dot => true,
        TokenType::And => true,
        TokenType::Or => true,
        TokenType::Plus => true,
        TokenType::Minus => true,
        TokenType::Multiply => true,
        TokenType::Divide => true,
        TokenType::Modulo => true,
        _ => false,
    }
}

// Parses the whole expression starting with identifier:
// - single identifier
// - function call
fn parse_ident_expression(parser: &mut Parser) -> Option<Expr> {
    let name_location = parser.peek_token().location;
    let start = BytePos(name_location.range.start.try_into().unwrap());
    let name = parser.expect_identifier()?;
    let mut left = Expr {
        span: Span {
            start: start,
            end: parser.last_pos,
        },
        kind: ExprKind::Identifier(IdentifierExpression {
            name: name,
            name_location: name_location,
        }),
    };
    loop {
        match parser.peek_token().token_type {
            TokenType::LeftParenthesis => {
                parser.advance();
                let arguments =
                    parser.parse_list(|p| p.parse_expression(), TokenType::RightParenthesis)?;
                left = Expr {
                    span: Span {
                        start: start,
                        end: parser.last_pos,
                    },
                    kind: ExprKind::Function(FunctionExpression {
                        callee: Box::new(left),
                        arguments: arguments,
                    }),
                };
            }
            TokenType::LeftBracket => {
                parser.advance();
                let idx = parse_array_subscript(parser)?;
                parser.expect_token(TokenType::RightBracket)?;
                left = Expr {
                    span: Span {
                        start: start,
                        end: parser.last_pos,
                    },
                    kind: ExprKind::ArraySubscript(ArraySubscriptExpression {
                        base: Box::new(left),
                        idx: Box::new(idx),
                    }),
                };
            }
            _ => return Some(left),
        }
    }
}

fn parse_array_subscript(parser: &mut Parser) -> Option<ArraySubscript> {
    let mut left = None;
    if parser.peek_token().token_type != TokenType::Colon {
        left = Some(parser.parse_expression()?);
    }

    if parser.peek_token().token_type == TokenType::Colon {
        parser.advance();
        let mut right = None;
        if parser.peek_token().token_type != TokenType::RightBracket {
            right = Some(parser.parse_expression()?);
        }
        return Some(ArraySubscript::Sublist(Sublist {
            left: left,
            right: right,
        }));
    }
    return Some(ArraySubscript::Index(left?));
}

fn parse_dictionary_entry(parser: &mut Parser) -> Option<DictionaryEntry> {
    if parser.peek_token().token_type != TokenType::StringLiteral {
        parser.expect_token(TokenType::StringLiteral)?;
    }
    let location = parser.peek_token().location;
    let key = literal(parser.l.token_text(&location));
    parser.advance();
    parser.expect_token(TokenType::Colon)?;
    let value = parse(parser)?;
    return Some(DictionaryEntry {
        key: key.to_string(),
        value: value,
    });
}

fn parse_prefix_expression(parser: &mut Parser) -> Option<Expr> {
    let token = parser.peek_token();
    let start = BytePos(token.location.range.start.try_into().unwrap());
    match token.token_type {
        TokenType::Number => {
            parser.advance();
            return Some(Expr {
                span: Span {
                    start: start,
                    end: parser.last_pos,
                },
                kind: ExprKind::Number(NumberExpression {
                    value: parser.l.token_text(&token.location).parse().unwrap(),
                }),
            });
        }
        TokenType::StringLiteral => {
            parser.advance();
            return Some(Expr {
                span: Span {
                    start: start,
                    end: parser.last_pos,
                },
                kind: ExprKind::StringLiteral(StringLiteralExpression {
                    value: literal(parser.l.token_text(&token.location)).to_string(),
                }),
            });
        }
        TokenType::Ident => return parse_ident_expression(parser),
        TokenType::LeftCurlyBrace => {
            parser.advance();
            let entries =
                parser.parse_list(|p| parse_dictionary_entry(p), TokenType::RightCurlyBrace)?;
            return Some(Expr {
                span: Span {
                    start: start,
                    end: parser.last_pos,
                },
                kind: ExprKind::Dictionary(DictionaryExpression { entries: entries }),
            });
        }
        TokenType::LeftBracket => return parse_array(parser),
        TokenType::LeftParenthesis => {
            parser.advance();
            let expr = parse(parser)?;
            parser.expect_token(TokenType::RightParenthesis)?;
            return Some(Expr {
                span: Span {
                    start: start,
                    end: parser.last_pos,
                },
                kind: ExprKind::Paren(ParenExpression {
                    expr: Box::new(expr),
                }),
            });
        }
        TokenType::Minus | TokenType::Bang => {
            parser.advance();
            return Some(Expr {
                span: Span {
                    start: start,
                    end: parser.last_pos,
                },
                kind: ExprKind::Unary(UnaryExpression {
                    operator: token.token_type,
                    expr: Box::new(parse_prefix_expression(parser)?),
                }),
            });
        }
        _ => {
            parser.error_and_recover("expression", token);
            return None;
        }
    }
}

fn parse_array(parser: &mut Parser) -> Option<Expr> {
    let token = parser.peek_token();
    let start = BytePos(token.location.range.start.try_into().unwrap());
    parser.expect_token(TokenType::LeftBracket);
    let elements = parser.parse_list(|p| parse(p), TokenType::RightBracket)?;
    return Some(Expr {
        span: Span {
            start: start,
            end: parser.last_pos,
        },
        kind: ExprKind::Array(ArrayExpression { elements: elements }),
    });
}

// TODO: this is incorrect, because it does not handle escaping properly.
fn literal(x: &str) -> &str {
    return &x[1..(x.len() - 1)];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    fn parse_and_dump(input: &str) -> serde_json::Value {
        let mut parser = Parser::new(Lexer::new(input));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(parser.peek_token().token_type, TokenType::Eof);
        return expression.unwrap().dump_for_testing();
    }

    #[test]
    fn parses_number_expression() {
        assert_eq!(parse_and_dump("15"), json!({ "number": 15.0 }));
    }

    #[test]
    fn parses_identifier_expression() {
        let mut parser = Parser::new(Lexer::new("my_var"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "identifier": "my_var",
            })
        );
    }

    #[test]
    fn parses_function_with_no_arguments_expression() {
        let mut parser = Parser::new(Lexer::new("myfunc()"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "function": {
                    "callee": { "identifier": "myfunc" },
                    "arguments": [],
                },
            })
        );
    }

    #[test]
    fn parses_function_with_string_literal() {
        let mut parser = Parser::new(Lexer::new("myfunc(\"foo\")"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "function": {
                    "callee": { "identifier": "myfunc" },
                    "arguments": [{"stringLiteral": "foo"}],
                },
            })
        );
    }

    #[test]
    fn parses_function_with_one_arguments_expression() {
        let mut parser = Parser::new(Lexer::new("myfunc(arg)"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "function": {
                    "callee": { "identifier": "myfunc" },
                    "arguments": [{"identifier": "arg"}],
                },
            })
        );
    }

    #[test]
    fn parses_function_with_two_arguments_expression() {
        let mut parser = Parser::new(Lexer::new("myfunc(arg1, arg2)"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "function": {
                    "callee": { "identifier": "myfunc" },
                    "arguments": [
                        {"identifier": "arg1"},
                        {"identifier": "arg2"},
                    ],
                },
            })
        );
    }

    #[test]
    fn parses_infix_with_in_equal_case_sensitive() {
        let mut parser = Parser::new(Lexer::new("a !=# b"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "infix": {
                    "left": { "identifier": "a" },
                    "operator": "`!=#`",
                    "right": { "identifier": "b" },
                },
            })
        );
    }

    #[test]
    fn parses_infix_with_less() {
        let mut parser = Parser::new(Lexer::new("a < b"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "infix": {
                    "left": { "identifier": "a" },
                    "operator": "`<`",
                    "right": { "identifier": "b" },
                },
            })
        );
    }

    #[test]
    fn parses_array_subscript() {
        let mut parser = Parser::new(Lexer::new("a[1]"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "arraySubscript": {
                    "base": {"identifier": "a"},
                    "idx": {"index": {"number": 1.0}},
                },
            })
        );
    }

    #[test]
    fn parses_multiple_infix_expressions() {
        let mut parser = Parser::new(Lexer::new("a . b . c"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "infix": {
                    "left": {
                        "infix": {
                            "left": { "identifier": "a" },
                            "operator": "`.`",
                            "right": { "identifier": "b" },
                        }
                    },
                    "operator": "`.`",
                    "right": {
                        "identifier": "c"
                    }
                }
            })
        );
    }

    #[test]
    fn parses_array_subscript_with_variable() {
        let mut parser = Parser::new(Lexer::new("a[s:e]"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "arraySubscript": {
                    "base": {"identifier": "a"},
                    "idx": {
                        "index": { "identifier": "s:e" },
                    },
                },
            })
        );
    }

    #[test]
    fn parses_sublist() {
        let mut parser = Parser::new(Lexer::new("a[s : e]"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "arraySubscript": {
                    "base": {"identifier": "a"},
                    "idx": {
                        "sublist": {
                            "left": {"identifier": "s"},
                            "right": {"identifier": "e"},
                        },
                    },
                },
            })
        );
    }

    #[test]
    fn parses_sublist_just_end() {
        let mut parser = Parser::new(Lexer::new("a[: e]"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "arraySubscript": {
                    "base": {"identifier": "a"},
                    "idx": {
                        "sublist": {
                            "right": {"identifier": "e"},
                        },
                    },
                },
            })
        );
    }

    #[test]
    fn parses_sublist_just_start() {
        let mut parser = Parser::new(Lexer::new("a[s :]"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "arraySubscript": {
                    "base": {"identifier": "a"},
                    "idx": {
                        "sublist": {
                            "left": {"identifier": "s"},
                        },
                    },
                },
            })
        );
    }

    #[test]
    fn parses_multi_array_subscript() {
        let mut parser = Parser::new(Lexer::new("a[1][2]"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "arraySubscript": {
                    "base": {
                        "arraySubscript": {
                            "base": {"identifier": "a"},
                            "idx": {"index": {"number": 1.0}},
                        },
                    },
                    "idx": {"index": {"number": 2.0}},
                },
            })
        );
    }

    #[test]
    fn parses_math_expressions() {
        let mut parser = Parser::new(Lexer::new("1 + 2 - 3 * 4 / 5"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        // NOTE: we do not have proper priorities yet!
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "infix": {
                    "left": {
                        "infix": {
                            "left": {
                                "infix": {
                                    "left": {
                                        "infix": {
                                            "left": {"number": 1.0},
                                            "operator": "`+`",
                                            "right": {"number": 2.0},
                                        }
                                    },
                                    "operator": "`-`",
                                    "right": {"number": 3.0},
                                }
                            },
                            "operator": "`*`",
                            "right": {"number": 4.0},
                        }
                    },
                    "operator": "`/`",
                    "right": {"number": 5.0},
                },
            })
        );
    }

    #[test]
    fn parses_array() {
        let mut parser = Parser::new(Lexer::new("[a, b]"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "array": {
                    "elements":[
                        {"identifier": "a"},
                        {"identifier": "b"},
                    ],
                }
            })
        );
    }

    #[test]
    fn parses_unary_minus_operator() {
        let mut parser = Parser::new(Lexer::new("-a"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "unary": {
                    "operator": "`-`",
                    "expr": {
                        "identifier": "a",
                    },
                }
            })
        );
    }

    #[test]
    fn parses_unary_bang_operator() {
        let mut parser = Parser::new(Lexer::new("!a"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "unary": {
                    "operator": "`!`",
                    "expr": {
                        "identifier": "a",
                    },
                }
            })
        );
    }

    #[test]
    fn parses_paren_expression() {
        assert_eq!(
            parse_and_dump("(a)"),
            json!({
                "paren": {
                    "identifier": "a",
                }
            })
        );
    }

    #[test]
    fn parses_choose_expression() {
        let mut parser = Parser::new(Lexer::new("a ? b : c"));
        let expression = parse(&mut parser);
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            expression.unwrap().dump_for_testing(),
            json!({
                "choose": {
                    "cond": {
                        "identifier": "a",
                    },
                    "lhs": {
                        "identifier": "b",
                    },
                    "rhs": {
                        "identifier": "c",
                    },
                }
            })
        );
    }

    #[test]
    fn parses_empty_dictionary() {
        assert_eq!(
            parse_and_dump("{}"),
            json!({
                "dictionary": { "entries": [] }
            })
        );
    }

    #[test]
    fn parses_dictionary_with_two_string_keys() {
        assert_eq!(
            parse_and_dump("{'one': 1, 'two': 2}"),
            json!({
                "dictionary": {
                    "entries": [
                        {"key": "one", "value": {"number": 1.0}},
                        {"key": "two", "value": {"number": 2.0}},
                    ]
                }
            })
        );
    }
}
