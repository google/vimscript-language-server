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
use crate::parser::Program;
use std::io::Write;

pub fn format(program: &Program) -> String {
    let mut w = Vec::new();
    let mut state = State{
        options: Options{indent: 2},
        out: &mut w,
        indent: 0,
    };
    state.format(&program);
    return String::from_utf8(w).unwrap();
}

// TODO: Make this struct public.
struct Options {
    // Number of spaces to use for indentation.
    // TODO: Support spaces and tabs.
    indent: usize,
}

struct State<'a, W: Write>{
    options: Options,
    out: &'a mut W,
    // Current identation level
    indent: usize,
}

impl<'a, W: Write> State<'a, W> {
    fn format(&mut self, program: &Program) {
        for statement in &program.statements {
            self.format_stmt(&statement)
        }
    }

    fn format_stmt(&mut self, stmt: &Stmt) {
        return match &stmt.kind {
            // StmtKind::Function(s) => self.format_statement_function(&s),
            // StmtKind::If(s) => self.format_if_statement(&s),
            // StmtKind::Let(s) => self.format_let_statement(&s),
            StmtKind::Return(s) => self.format_return_statement(&s),
            _ => panic!("some statement is not supported by formatter yet"),
        };
    }

    fn format_return_statement(&mut self, _stmt: &ReturnStatement) {
        self.write_indent();
        write!(self.out, "return");
        write!(self.out, "\n");
    }

    fn write_indent(&mut self) {
        write!(self.out, "{}", &" ".repeat(self.options.indent * self.indent));
    }
}

fn format_statement(stmt: &Stmt, spaces: usize) -> String {
    return match &stmt.kind {
        StmtKind::Function(s) => format_statement_function(&s, spaces),
        StmtKind::If(s) => format_if_statement(&s, spaces),
        StmtKind::Let(s) => format_let_statement(&s, spaces),
        StmtKind::Return(s) => format_return_statement(&s, spaces),
        _ => panic!("some statement is not supported by formatter yet"),
    };
}

fn format_statement_function(stmt: &FunctionStatement, spaces: usize) -> String {
    let mut s = String::new();
    s.push_str(&" ".repeat(spaces * 2));
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

    s.push_str(&" ".repeat(spaces * 2));
    s.push_str("endfunction\n");
    return s;
}

fn format_expression(expr: &ExprKind) -> String {
    return match expr {
        ExprKind::Identifier(e) => e.name().to_string(),
        ExprKind::Number(e) => e.value().to_string(),
        _ => "<unknown expression>".to_string(),
    };
}

fn format_let_statement(stmt: &LetStatement, spaces: usize) -> String {
    let mut s = String::new();
    s.push_str(&" ".repeat(spaces * 2));
    s.push_str("let ");
    s.push_str(&format_expression(&stmt.var.kind));
    s.push_str(" ");
    s.push_str(stmt.operator.to_str());
    s.push_str(" ");
    s.push_str(&format_expression(&stmt.value.kind));
    s.push_str("\n");
    return s;
}

fn format_return_statement(_stmt: &ReturnStatement, spaces: usize) -> String {
    let mut s = String::new();
    s.push_str(&" ".repeat(spaces * 2));
    s.push_str("return");
    s.push_str("\n");
    return s;
}

fn format_if_statement(stmt: &IfStatement, spaces: usize) -> String {
    let mut s = String::new();
    s.push_str(&" ".repeat(spaces * 2));
    s.push_str("if ");
    s.push_str(&format_expression(&stmt.condition.kind));
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
            s.push_str(&" ".repeat(spaces * 2));
            s.push_str("else");
            s.push_str("\n");
            for st in stmts.iter() {
                s.push_str(&format_statement(&st, spaces + 1))
            }
        }
        ElseCond::ElseIf(stmt) => {
            s.push_str("elseif ");
            s.push_str(&format_expression(&stmt.condition.kind));
            s.push_str("\n");
            s.push_str(&format_if_statement_internal(stmt, spaces))
        }
        _ => {}
    }
    s.push_str(&" ".repeat(spaces * 2));
    return s;
}
