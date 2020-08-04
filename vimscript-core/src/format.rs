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

use crate::parser::ElseCond;
use crate::parser::Expression;
use crate::parser::FunctionStatement;
use crate::parser::IfStatement;
use crate::parser::LetStatement;
use crate::parser::Program;
use crate::parser::Statement;

pub fn format(program: &Program) -> String {
    let mut res = "".to_string();
    for statement in &program.statements {
        res += &format_statement(&statement, 0)
    }
    return res;
}

fn format_statement(stmt: &Statement, spaces: usize) -> String {
    return match stmt {
        Statement::Function(s) => format_statement_function(s, spaces),
        Statement::If(s) => format_if_statement(s, spaces),
        Statement::Let(s) => format_let_statement(s, spaces),
        _ => "<unknown statement>\n".to_string(),
    };
}

fn format_statement_function(stmt: &FunctionStatement, spaces: usize) -> String {
    let mut s = String::new();
    s.push_str(&" ".repeat(spaces * 4));
    s.push_str("function ");
    s.push_str(&stmt.name);
    s.push_str("()\n");

    s.push_str(
        &stmt
            .body
            .iter()
            .map(|s| return format_statement(s, spaces + 1))
            .collect::<Vec<String>>()
            .join(""),
    );

    s.push_str(&" ".repeat(spaces * 4));
    s.push_str("endfunction\n");
    return s;
}

fn format_expression(expr: &Expression) -> String {
    return match expr {
        Expression::Identifier(e) => e.name().to_string(),
        Expression::Number(e) => e.value().to_string(),
        _ => "<unknown expression>".to_string(),
    };
}

fn format_let_statement(stmt: &LetStatement, spaces: usize) -> String {
    let mut s = String::new();
    s.push_str(&" ".repeat(spaces * 4));
    s.push_str("let ");
    s.push_str(&format_expression(&stmt.var()));
    s.push_str(" ");
    s.push_str(stmt.operator().to_str());
    s.push_str(" ");
    s.push_str(&format_expression(&stmt.value()));
    s.push_str("\n");
    return s;
}

fn format_if_statement(stmt: &IfStatement, spaces: usize) -> String {
    let mut s = String::new();
    s.push_str(&" ".repeat(spaces * 4));
    s.push_str("if ");
    s.push_str(&format_expression(&stmt.condition));
    s.push_str("\n");
    s.push_str(&format_if_statement_internal(stmt, spaces));
    s.push_str("endif\n");
    return s;
}

fn format_if_statement_internal(stmt: &IfStatement, spaces: usize) -> String {
    let mut s = String::new();
    for st in stmt.then.iter() {
        s.push_str(&format_statement(&st, spaces + 1))
    }
    match &stmt.else_cond {
        ElseCond::Else(stmts) => {
            s.push_str(&" ".repeat(spaces * 4));
            s.push_str("else");
            s.push_str("\n");
            for st in stmts.iter() {
                s.push_str(&format_statement(&st, spaces + 1))
            }
        }
        ElseCond::ElseIf(stmt) => {
            s.push_str("elseif ");
            s.push_str(&format_expression(&stmt.condition));
            s.push_str("\n");
            s.push_str(&format_if_statement_internal(stmt, spaces))
        }
        _ => {}
    }
    s.push_str(&" ".repeat(spaces * 4));
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn formats_functions() {
        let mut parser = Parser::new(Lexer::new(
            r#"   function    Test(   )
let    l:x=1
  endfunction"#,
        ));
        let program = parser.parse();
        let formatted = format(&program);
        assert_eq!(
            formatted,
            r#"function Test()
    let l:x = 1
endfunction
"#
        );
    }

    #[test]
    fn formats_if_statements() {
        let mut parser = Parser::new(Lexer::new(
            r#"
  if l:ver
let    l:x=1
    endif
  "#,
        ));
        let program = parser.parse();
        let formatted = format(&program);
        assert_eq!(
            formatted,
            r#"if l:ver
    let l:x = 1
endif
"#
        );
    }

    #[test]
    fn formats_if_statements_with_else() {
        let mut parser = Parser::new(Lexer::new(
            r#"
  if l:ver
let    l:x=1
else
let    l:x=2
    endif
  "#,
        ));
        let program = parser.parse();
        let formatted = format(&program);
        assert_eq!(
            formatted,
            r#"if l:ver
    let l:x = 1
else
    let l:x = 2
endif
"#
        );
    }

    #[test]
    fn formats_if_statements_with_elseif() {
        let mut parser = Parser::new(Lexer::new(
            r#"
  if l:ver
let    l:x=1
  elseif   l:ver2
let    l:x=2
    endif
  "#,
        ));
        let program = parser.parse();
        let formatted = format(&program);
        assert_eq!(
            formatted,
            r#"if l:ver
    let l:x = 1
elseif l:ver2
    let l:x = 2
endif
"#
        );
    }
}
