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

use crate::ast::SetStatement;
use crate::lexer::TokenType;
use crate::parser::Parser;

pub fn parse(parser: &mut Parser) -> Option<SetStatement> {
    let option = parser.expect_identifier()?;
    if parser.peek_token().token_type != TokenType::Assign {
        parser.expect_end_of_statement()?;
        return Some(SetStatement {
            option: option,
            value: None,
        });
    }
    parser.advance();
    let value = parser.expect_identifier()?;

    return Some(SetStatement {
        option: option,
        value: Some(value),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use serde_json::Value;

    #[test]
    fn parses_set_statement() {
        let mut parser = Parser::new(Lexer::new("set paste"));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            program.dump_for_testing(),
            json!([{"set": {"option": "paste", "value": Value::Null}}])
        );
    }

    #[test]
    fn parses_set_statement_with_value() {
        let mut parser = Parser::new(Lexer::new("set selection=exclusive"));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            program.dump_for_testing(),
            json!([{"set": {"option": "selection", "value": "exclusive"}}])
        );
    }
}
