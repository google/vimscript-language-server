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
use crate::lexer::TokenPosition;
use crate::lexer::TokenType;
use crate::lexer::SourcePosition;
use crate::parser::Parser;
use crate::parser::Program;
use crate::parser::Statement;
use crate::parser::Expression;
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::TextEdit;
use std::collections::HashMap;

pub fn rename(source: &str, pos: Position, new_name: &str) -> Result<Vec<TextEdit>, ()> {
    let mut parser = Parser::new(Lexer::new(source));
    let program = parser.parse();
    let mut rename_op = Rename::new();
    rename_op.visit(&program, &parser);
    rename_op.rename(&parser, pos, new_name)
}

fn token_position_to_range(position: &TokenPosition) -> Range {
    Range {
        start: source_position_to_position(&position.start),
        end: source_position_to_position(&position.end),
    }
}

fn source_position_to_position(position: &SourcePosition) -> Position {
    Position {
        line: position.line as u64,
        character: position.character as u64,
    }
}

struct Rename {
    token_to_positions: HashMap<String, Vec<TokenPosition>>,
}

impl Rename {
    fn new() -> Rename {
        return Rename{token_to_positions: HashMap::new()}
    }
    fn visit(&mut self, program: &Program, parser: &Parser) {
        for stmt in &program.statements {
            self.visit_statement(&stmt, parser);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement, parser: &Parser) {
        match stmt {
            // Statement::Let(stmt) => {
            //     let positions = self.token_to_positions.entry(stmt.name().to_string()).or_insert(Vec::new());
            //     positions.push(parser.resolve_location(stmt.name_location().clone()));
            // }
            Statement::Call(stmt) => {
                for expr in &stmt.arguments {
                    self.visit_expression(expr, parser)
                }
            }
            _ => {}
        }
    }

    fn visit_expression(&mut self, expr: &Expression, parser: &Parser) {
        match expr {
            Expression::Identifier(expr) => {
                let positions = self.token_to_positions.entry(expr.name().to_string()).or_insert(Vec::new());
                positions.push(parser.resolve_location(expr.name_location().clone()));
            }
            _ => {}
        }
    }

    pub fn rename(&self, parser: &Parser, pos: Position, new_name: &str) -> Result<Vec<TextEdit>, ()> {
        let token = parser.find_token(SourcePosition{line: pos.line as i32, character: pos.character as i32})?;
        if token.token_type != TokenType::Ident {
            return Err(());
        }
        let val = parser.identifier_name(&token);
        let positions = &self.token_to_positions[&val];
        let mut edits = Vec::new();
        for pos in positions {
            edits.push(TextEdit{
                new_text: new_name.to_string(),
                range: token_position_to_range(pos),
            });
        }
        Ok(edits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::Range;
    use pretty_assertions::assert_eq;

    // This is still WIP.
    #[test]
    fn test() {
        let res = rename(
            "let l:a = 5\ncall echo(l:a)",
            Position {
                line: 0,
                character: 5,
            },
            "l:b",
        )
        .unwrap();
        assert_eq!(
            res,
            &[
                // TextEdit {
                //     range: Range {
                //         start: Position {
                //             line: 0,
                //             character: 4,
                //         },
                //         end: Position {
                //             line: 0,
                //             character: 7,
                //         },
                //     },
                //     new_text: "l:b".to_string(),
                // },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 10,
                        },
                        end: Position {
                            line: 1,
                            character: 13,
                        },
                    },
                    new_text: "l:b".to_string(),
                }
            ]
        );
    }
}
