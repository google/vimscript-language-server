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

use crate::ast::BreakStatement;
use crate::ast::CallStatement;
use crate::ast::ExecuteStatement;
use crate::ast::ForStatement;
use crate::ast::FunctionStatement;
use crate::ast::LetStatement;
use crate::ast::LoopVariable;
use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::lexer::SourceLocation;
use crate::lexer::SourcePosition;
use crate::lexer::Token;
use crate::lexer::TokenPosition;
use crate::lexer::TokenType;
pub use crate::parser::expression::*;
pub use crate::parser::if_statement::ElseCond;
pub use crate::parser::if_statement::IfStatement;
use serde_json::json;

pub mod expression;

pub mod if_statement;
pub mod let_statement;
pub mod return_statement;
pub mod set_statement;
pub mod try_statement;
pub mod while_statement;
use std::iter::Iterator;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!(self
            .statements
            .iter()
            .map(|s| s.dump_for_testing())
            .collect::<Vec<serde_json::Value>>());
    }
}

#[derive(PartialEq, Debug)]
pub struct ParseError {
    pub message: String,
    pub position: TokenPosition,
}

pub struct Parser<'a> {
    pub l: Lexer<'a>,
    tokens: Vec<Token>,
    lexer: Peekable<std::vec::IntoIter<Token>>,
    pub errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Parser {
        let tokens = lexer.lex();
        return Parser {
            l: lexer,
            tokens: tokens.clone(),
            lexer: tokens.into_iter().peekable(),
            errors: Vec::new(),
        };
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.lexer.peek() != None {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }
        return Program {
            statements: statements,
        };
    }

    pub fn resolve_location(&self, loc: SourceLocation) -> TokenPosition {
        self.l.token_position(&loc)
    }

    pub fn find_token(&self, pos: SourcePosition) -> Result<Token, ()> {
        // TODO: This is very naive implementation, we can do a lot of optimizations here.
        for token in &self.tokens {
            let token_pos = self.resolve_location(token.location.clone());
            if token_pos.start <= pos && pos <= token_pos.end {
                return Ok(token.clone());
            }
        }
        Err(())
    }

    // Parses a statement, including the new line at the end of statement.
    // Returns None when statement failed to parse.
    fn parse_statement(&mut self) -> Option<Statement> {
        let token = self.lexer.next()?;
        match token.token_type {
            TokenType::Let => {
                if let Some(stmt) = self.parse_let_statement() {
                    return Some(Statement::Let(stmt));
                }
            }
            TokenType::Break => {
                self.expect_end_of_statement()?;
                return Some(Statement::Break(BreakStatement {}));
            }
            TokenType::Call => {
                if let Some(stmt) = self.parse_call_statement() {
                    return Some(Statement::Call(stmt));
                }
            }
            TokenType::Return => {
                if let Some(stmt) = return_statement::parse(self) {
                    return Some(Statement::Return(stmt));
                }
            }
            TokenType::Try => {
                if let Some(stmt) = try_statement::parse(self) {
                    return Some(Statement::Try(stmt));
                }
            }
            TokenType::Set => {
                if let Some(stmt) = set_statement::parse(self) {
                    return Some(Statement::Set(stmt));
                }
            }
            TokenType::Execute => return self.parse_execute_statement(),
            TokenType::If => {
                if let Some(stmt) = self.parse_if_statement() {
                    return Some(Statement::If(stmt));
                }
            }
            TokenType::Function => {
                if let Some(stmt) = self.parse_function_statement() {
                    return Some(Statement::Function(stmt));
                }
            }
            TokenType::For => {
                if let Some(stmt) = self.parse_for_statement() {
                    return Some(Statement::For(stmt));
                }
            }
            TokenType::While => {
                if let Some(stmt) = while_statement::parse(self) {
                    return Some(Statement::While(stmt));
                }
            }
            TokenType::NewLine => {}
            TokenType::Pipe => {}
            _ => {
                self.errors.push(ParseError {
                    message: format!("expected keyword, found {}", self.token_text(&token)),
                    position: self.l.token_position(&token.location),
                });
                self.consume_until_end_of_statement();
            }
        }
        return None;
    }

    fn parse_call_statement(&mut self) -> Option<CallStatement> {
        let name = self.expect_identifier()?;

        self.expect_token(TokenType::LeftParenthesis)?;
        let arguments = self.parse_list(|p| p.parse_expression(), TokenType::RightParenthesis)?;

        return Some(CallStatement {
            name: name,
            arguments: arguments,
        });
    }

    pub fn end_of_statement_token(token: TokenType) -> bool {
        return token == TokenType::NewLine || token == TokenType::Eof || token == TokenType::Pipe;
    }

    fn parse_execute_statement(&mut self) -> Option<Statement> {
        let mut arguments = Vec::new();
        while !Parser::end_of_statement_token(self.peek_token().token_type) {
            arguments.push(self.parse_expression()?);
        }

        return Some(Statement::Execute(ExecuteStatement {
            arguments: arguments,
        }));
    }

    // Let = 'let' VarName = Expression (NewLine | EOF)
    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        return let_statement::parse(self);
    }

    fn consume_until_end_of_statement(&mut self) {
        loop {
            match self.lexer.next() {
                None => break,
                Some(token) => {
                    if Parser::end_of_statement_token(token.token_type) {
                        break;
                    }
                }
            }
        }
    }

    pub fn token_text(&self, token: &Token) -> String {
        match token.token_type {
            TokenType::NewLine => "new line".to_string(),
            _ => format!("`{}`", self.l.token_text(&token.location).to_string()),
        }
    }

    // Precondition - if was already read.
    //
    // If ::= 'if' Expression NewLine Statement* 'endif'
    fn parse_if_statement(&mut self) -> Option<IfStatement> {
        return if_statement::parse(self);
    }

    fn parse_for_statement(&mut self) -> Option<ForStatement> {
        let loop_variable = self.parse_loop_variable()?;

        self.expect_token(TokenType::In)?;

        let range = self.parse_expression()?;
        self.expect_end_of_statement()?;

        let statements = self.parse_statements_until(TokenType::EndFor)?;

        Some(ForStatement {
            loop_variable: loop_variable,
            range: range,
            body: statements,
        })
    }

    fn parse_loop_variable(&mut self) -> Option<LoopVariable> {
        let token = self.peek_token();
        match token.token_type {
            TokenType::LeftBracket => self.parse_list_loop_variable(),
            TokenType::Ident => Some(LoopVariable::Single(self.expect_identifier()?)),
            _ => {
                self.error_and_recover("`(` or identifier", token);
                None
            }
        }
    }

    fn parse_list_loop_variable(&mut self) -> Option<LoopVariable> {
        self.expect_token(TokenType::LeftBracket)?;
        let vars = self.parse_list(|p| p.expect_identifier(), TokenType::RightBracket)?;
        return Some(LoopVariable::List(vars));
    }

    // Parses statements until the next statement starts with given token or EOF is encountered.
    fn parse_statements_until(&mut self, token_type: TokenType) -> Option<Vec<Statement>> {
        let mut stmts = Vec::new();
        while self.peek_token().token_type != TokenType::Eof
            && self.peek_token().token_type != token_type
        {
            // TODO: It would be nice to pass the expected token here, so that error message can
            // include it as well.
            if let Some(stmt) = self.parse_statement() {
                stmts.push(stmt);
            }
        }
        self.expect_token(token_type)?;
        self.expect_end_of_statement()?;
        return Some(stmts);
    }

    fn parse_function_statement(&mut self) -> Option<FunctionStatement> {
        let mut abort = false;
        let mut overwrite = false;

        if self.peek_token().token_type == TokenType::Bang {
            self.advance();
            overwrite = true;
        }

        let name = self.expect_identifier()?;

        self.expect_token(TokenType::LeftParenthesis)?;

        let arguments = self.parse_list(|p| p.expect_identifier(), TokenType::RightParenthesis)?;

        if self.peek_token().token_type == TokenType::Abort {
            self.advance();
            abort = true;
        }
        self.expect_end_of_statement()?;

        let body = self.parse_statements_until(TokenType::EndFunction)?;

        return Some(FunctionStatement {
            name: name,
            arguments: arguments,
            body: body,
            abort: abort,
            overwrite: overwrite,
        });
    }

    // Number ::= 0 | [1-9][0-9]*
    // StringLiteral ::= '.*'
    // Expression =
    fn parse_expression(&mut self) -> Option<Expression> {
        return expression::parse(self);
    }

    // parse_list(|p| {p.parse_expression()}, TokenType::RightParenthesis)
    pub fn parse_list<F, T>(&mut self, mut f: F, end: TokenType) -> Option<Vec<T>>
    where
        F: FnMut(&mut Parser) -> Option<T>,
    {
        let mut result = Vec::new();
        let token = self.peek_token();
        if token.token_type == end {
            self.advance();
        } else {
            result.push(f(self)?);
            loop {
                let token = self.peek_token();
                match token.token_type {
                    x if x == end => {
                        self.advance();
                        break;
                    }
                    TokenType::Comma => {
                        self.advance();
                        // TODO: should this be optional? It is required for dictionary literals
                        // (which can have trailing comma), but not sure about other statements /
                        // expressions.
                        if self.peek_token().token_type == end {
                            self.advance();
                            break;
                        }
                        result.push(f(self)?);
                    }
                    _ => {
                        // TODO: use end instead of `)`
                        self.error_and_recover("`,` or `)`", token);
                        return None;
                    }
                }
            }
        }
        return Some(result);
    }

    fn expect_end_of_statement(&mut self) -> Option<()> {
        let token = self.peek_token();
        if Parser::end_of_statement_token(token.token_type) {
            self.advance();
            return Some(());
        }
        self.error_and_recover("new line", token);
        return None;
    }

    fn expect_token(&mut self, token_type: TokenType) -> Option<()> {
        let token = self.peek_token();
        if token.token_type == token_type {
            self.advance();
            return Some(());
        }
        self.error_and_recover(token_type.as_str(), token);
        return None;
    }

    pub fn error_and_recover(&mut self, expected: &str, found: Token) {
        self.errors.push(ParseError {
            message: format!("expected {}, found {}", expected, self.token_text(&found)),
            position: self.l.token_position(&found.location),
        });
        self.consume_until_end_of_statement();
    }

    // If peek is identifier, returns name and advances.
    // Otherwise, consume until end of statement.
    fn expect_identifier(&mut self) -> Option<String> {
        let token = self.peek_token();
        let name = match token.token_type {
            TokenType::Ident => self.identifier_name(&token),
            _ => {
                self.error_and_recover("identifier", token);
                return None;
            }
        };
        self.advance();
        Some(name)
    }

    pub fn identifier_name(&self, token: &Token) -> String {
        return self.l.token_text(&token.location).to_string();
    }

    pub fn advance(&mut self) {
        self.lexer.next();
    }

    pub fn peek_token(&mut self) -> Token {
        match self.lexer.peek() {
            Some(token) => token.clone(),
            None => self.l.eof_token(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::SourcePosition;
    use pretty_assertions::assert_eq;

    #[test]
    fn returns_one_error_per_line() {
        let mut parser = Parser::new(Lexer::new("unknown xx()"));
        parser.parse();
        assert_eq!(
            parser.errors,
            &[ParseError {
                message: "expected keyword, found `unknown`".to_string(),
                position: TokenPosition {
                    start: SourcePosition {
                        line: 0,
                        character: 0,
                    },
                    end: SourcePosition {
                        line: 0,
                        character: 7,
                    },
                }
            }]
        );
    }

    // #[test]
    // fn parses_call_statements() {
    //     let mut parser = Parser::new(Lexer::new("call func(l:a, l:b)"));
    //     let program = parser.parse();
    //     assert_eq!(parser.errors, &[]);
    //     assert_eq!(
    //         program.statements,
    //         &[Statement::Call(CallStatement {
    //             name: "func".to_string(),
    //             arguments: vec![
    //                 Expression::Identifier(IdentifierExpression {
    //                     name: "l:a".to_string()
    //                 }),
    //                 Expression::Identifier(IdentifierExpression {
    //                     name: "l:b".to_string()
    //                 })
    //             ],
    //         })]
    //     );
    // }

    // #[test]
    // fn parses_execute_statements() {
    //     let mut parser = Parser::new(Lexer::new("execute l:a l:b . l:c"));
    //     let program = parser.parse();
    //     assert_eq!(parser.errors, &[]);
    //     assert_eq!(
    //         program.statements,
    //         &[Statement::Execute(ExecuteStatement {
    //             arguments: vec![
    //                 Expression::Identifier(IdentifierExpression {
    //                     name: "l:a".to_string()
    //                 }),
    //                 Expression::Infix(InfixExpression {
    //                     left: Box::new(Expression::Identifier(IdentifierExpression {
    //                         name: "l:b".to_string()
    //                     })),
    //                     operator: TokenType::Dot,
    //                     right: Box::new(Expression::Identifier(IdentifierExpression {
    //                         name: "l:c".to_string()
    //                     })),
    //                 })
    //             ],
    //         })]
    //     );
    // }

    #[test]
    fn parses_function_statement() {
        let mut parser = Parser::new(Lexer::new(
            "
            function! my#method(arg1, arg2) abort
                call guess()
            endfunction
            ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(
            program.statements,
            &[Statement::Function(FunctionStatement {
                name: "my#method".to_string(),
                arguments: vec!["arg1".to_string(), "arg2".to_string()],
                body: vec![Statement::Call(CallStatement {
                    name: "guess".to_string(),
                    arguments: vec![],
                })],
                overwrite: true,
                abort: true,
            })]
        );
    }

    // #[test]
    // fn parses_for_statement_with_one_variable() {
    //     let mut parser = Parser::new(Lexer::new(
    //         "
    //         for item in copy(mylist)
    //             call guess()
    //         endfor
    //         ",
    //     ));
    //     let program = parser.parse();
    //     assert_eq!(parser.errors, &[]);
    //     assert_eq!(
    //         program.statements,
    //         &[Statement::For(ForStatement {
    //             loop_variable: LoopVariable::Single("item".to_string()),
    //             range: Expression::Function(FunctionExpression {
    //                 name: "copy".to_string(),
    //                 arguments: vec![Expression::Identifier(IdentifierExpression {
    //                     name: "mylist".to_owned(),
    //                 })],
    //             }),
    //             body: vec![Statement::Call(CallStatement {
    //                 name: "guess".to_string(),
    //                 arguments: vec![],
    //             })],
    //         })]
    //     );
    // }

    #[test]
    fn parses_for_statement_with_multiple_variables() {
        let mut parser = Parser::new(Lexer::new(
            "
            for [a1, a2, a3] in copy(mylist)
                call guess()
            endfor
            ",
        ));
        let program = parser.parse();
        assert_eq!(parser.errors, &[]);
        assert_eq!(program.statements.len(), 1);
        let for_stmt = match &program.statements[0] {
            Statement::For(stmt) => stmt,
            stmt => panic!(format!("expected for statement, got {:?}", stmt)),
        };
        assert_eq!(
            for_stmt.loop_variable,
            LoopVariable::List(vec!["a1".to_string(), "a2".to_string(), "a3".to_string()])
        );
        match &for_stmt.range {
            Expression::Function(_) => {}
            expr => panic!(format!("expected function expression, got {:?}", expr)),
        };
        assert_eq!(
            for_stmt.body,
            vec![Statement::Call(CallStatement {
                name: "guess".to_string(),
                arguments: vec![],
            })]
        );
    }
}
