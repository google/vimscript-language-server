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

use crate::ast::ReturnStatement;
use crate::parser::Parser;

pub fn parse(parser: &mut Parser) -> Option<ReturnStatement> {
    if Parser::end_of_statement_token(parser.peek_token().token_type) {
        parser.advance();
        return Some(ReturnStatement { value: None });
    }
    let value = parser.parse_expression()?;
    parser.expect_end_of_statement()?;
    return Some(ReturnStatement { value: Some(value) });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn parses_return_statement() {
        let mut parser = Parser::new(Lexer::new("return l:foo"));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            program.dump_for_testing(),
            json!([{"return": {"value": {"identifier": "l:foo"}}}]),
        );
    }

    #[test]
    fn parses_empty_return() {
        let mut parser = Parser::new(Lexer::new("return"));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.dump_for_testing(), json!([{"return": {}}]));
    }
}
