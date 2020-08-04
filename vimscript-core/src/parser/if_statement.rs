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

use crate::lexer::TokenType;
use crate::parser::Expression;
use crate::parser::Parser;
use crate::parser::Statement;
use serde_json::json;

#[derive(PartialEq, Debug)]
pub enum ElseCond {
    None,
    Else(Vec<Statement>),
    ElseIf(Box<IfStatement>),
}

impl ElseCond {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return match self {
            ElseCond::None => serde_json::Value::Null,
            ElseCond::Else(stmts) => serde_json::Value::Array(
                stmts
                    .iter()
                    .map(|s| s.dump_for_testing())
                    .collect::<Vec<serde_json::Value>>(),
            ),
            ElseCond::ElseIf(stmt) => stmt.dump_for_testing(),
        };
    }
}

#[derive(PartialEq, Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub then: Vec<Statement>,
    pub else_cond: ElseCond,
}

impl IfStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "condition": self.condition.dump_for_testing(),
            "then": self.then.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
            "else": self.else_cond.dump_for_testing(),
        });
    }
}

// Precondition - if was already read.
//
// If ::= 'if' Expression NewLine Statement* 'endif'
pub fn parse(parser: &mut Parser) -> Option<IfStatement> {
    let condition = parser.parse_expression()?;

    parser.expect_end_of_statement()?;

    let mut stmts = Vec::new();
    while parser.peek_token().token_type != TokenType::Eof {
        if parser.peek_token().token_type == TokenType::EndIf {
            parser.advance();
            parser.expect_end_of_statement()?;
            return Some(IfStatement {
                condition: condition,
                then: stmts,
                else_cond: ElseCond::None,
            });
        }
        if parser.peek_token().token_type == TokenType::Else {
            parser.advance();
            parser.expect_end_of_statement()?;
            let else_cond = parser.parse_statements_until(TokenType::EndIf)?;
            return Some(IfStatement {
                condition: condition,
                then: stmts,
                else_cond: ElseCond::Else(else_cond),
            });
        }
        if parser.peek_token().token_type == TokenType::ElseIf {
            parser.advance();
            return Some(IfStatement {
                condition: condition,
                then: stmts,
                else_cond: ElseCond::ElseIf(Box::new(parse(parser)?)),
            });
        }

        if let Some(stmt) = parser.parse_statement() {
            stmts.push(stmt);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_if_statement() {
        let mut parser = Parser::new(Lexer::new(
            "
             if l:foo !=# l:bar
                 call my#method()
             endif
             ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "if": {
                    "condition": {
                        "infix": {
                            "left": {"identifier":"l:foo"},
                            "operator": "`!=#`",
                            "right": {"identifier":"l:bar"},
                        }
                    },
                    "then": [{
                        "call": {
                            "method": "my#method",
                            "arguments": [],
                        }
                    }],
                    "else": serde_json::Value::Null,
                },
            }])
        );
    }

    #[test]
    fn parses_if_statement_with_else() {
        let mut parser = Parser::new(Lexer::new(
            "
             if l:foo !=# l:bar
                 call my#method1()
             else
                 call my#method2()
             endif
             ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "if": {
                    "condition": {
                        "infix": {
                            "left": {"identifier":"l:foo"},
                            "operator": "`!=#`",
                            "right": {"identifier":"l:bar"},
                        }
                    },
                    "then": [{
                        "call": {
                            "method": "my#method1",
                            "arguments": [],
                        }
                    }],
                    "else": [{
                        "call": {
                            "method": "my#method2",
                            "arguments": [],
                        }
                    }],
                },
            }])
        );
    }

    #[test]
    fn parses_if_statement_with_elseif() {
        let mut parser = Parser::new(Lexer::new(
            "
             if l:foo1 !=# l:bar1
                 call my#method1()
             elseif l:foo2 !=# l:bar2
                 call my#method2()
             else
                 call my#method3()
             endif
             ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "if": {
                    "condition": {
                        "infix": {
                            "left": {"identifier":"l:foo1"},
                            "operator": "`!=#`",
                            "right": {"identifier":"l:bar1"},
                        }
                    },
                    "then": [{
                        "call": {
                            "method": "my#method1",
                            "arguments": [],
                        }
                    }],
                    "else": {
                        "condition": {
                            "infix": {
                                "left": {"identifier":"l:foo2"},
                                "operator": "`!=#`",
                                "right": {"identifier":"l:bar2"},
                            }
                        },
                        "then": [{
                            "call": {
                                "method": "my#method2",
                                "arguments": [],
                            }
                        }],
                        "else": [{
                            "call": {
                                "method": "my#method3",
                                "arguments": [],
                            }
                        }],
                    },
                },
            }])
        );
    }
}
