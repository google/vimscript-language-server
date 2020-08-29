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

use crate::ast::WhileStatement;
use crate::lexer::TokenType;
use crate::parser::Parser;

// Precondition - `while` token was already read.
pub fn parse(parser: &mut Parser) -> Option<WhileStatement> {
    let condition = parser.parse_expression()?;
    parser.expect_end_of_statement()?;
    let body = parser.parse_statements_until(TokenType::EndWhile)?;
    return Some(WhileStatement {
        condition: condition,
        body: body,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn parses_while_statement() {
        let mut parser = Parser::new(Lexer::new(
            "
             while l:foo !=# l:bar
                 call my#method()
             endwhile
             ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.dump_for_testing(),
            json!([{
                "while": {
                    "condition": {
                        "infix": {
                            "left": {"identifier":"l:foo"},
                            "operator": "`!=#`",
                            "right": {"identifier":"l:bar"},
                        }
                    },
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
}
