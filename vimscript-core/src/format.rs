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
    let mut state = State {
        options: Options { indent: 2 },
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

struct State<'a, W: Write> {
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

    fn write(&mut self, s: &str) {
        self.out.write_all(s.as_bytes()).unwrap();
    }

    fn format_stmt(&mut self, stmt: &Stmt) {
        return match &stmt.kind {
            StmtKind::Function(s) => self.format_statement_function(&s),
            StmtKind::If(s) => self.format_if_statement(&s),
            StmtKind::Let(s) => self.format_let_statement(&s),
            StmtKind::Return(s) => self.format_return_statement(&s),
            _ => panic!("some statement is not supported by formatter yet"),
        };
    }

    fn format_statement_function(&mut self, stmt: &FunctionStatement) {
        self.write_indent();
        self.write("function ");
        self.write(&stmt.name);
        self.write("()\n");

        self.indent += 1;
        for s in &stmt.body {
            self.format_stmt(&s);
        }
        self.indent -= 1;

        self.write_indent();
        self.write("endfunction\n");
    }

    fn format_return_statement(&mut self, _stmt: &ReturnStatement) {
        self.write_indent();
        self.write("return");
        self.write("\n");
    }

    fn write_indent(&mut self) {
        self.write(&" ".repeat(self.options.indent * self.indent));
    }

    fn format_let_statement(&mut self, stmt: &LetStatement) {
        self.write_indent();
        self.write("let ");
        self.format_expression(&stmt.var.kind);
        self.write(" ");
        // TODO: Fix it for other operatrs
        self.write("=");
        self.write(" ");
        self.format_expression(&stmt.value.kind);
        self.write("\n");
    }

    fn format_expression(&mut self, expr: &ExprKind) {
        match expr {
            ExprKind::Identifier(e) => self.write(&e.name().to_string()),
            ExprKind::Number(e) => self.write(&e.value().to_string()),
            _ => panic!("unknown expression"),
        };
    }

    fn format_if_statement(&mut self, stmt: &IfStatement) {
        self.write_indent();
        self.write("if ");
        self.format_expression(&stmt.condition.kind);
        self.write("\n");
        self.format_if_statement_internal(stmt);
        self.write("endif\n");
    }

    fn format_if_statement_internal(&mut self, stmt: &IfStatement) {
        self.indent += 1;
        for st in stmt.then.iter() {
            self.format_stmt(&st)
        }
        self.indent -= 1;
        match &stmt.else_cond {
            ElseCond::Else(stmts) => {
                self.write_indent();
                self.write("else\n");
                self.indent += 1;
                for st in stmts.iter() {
                    self.format_stmt(&st)
                }
                self.indent -= 1;
            }
            ElseCond::ElseIf(stmt) => {
                self.write("elseif ");
                self.format_expression(&stmt.condition.kind);
                self.write("\n");
                self.format_if_statement_internal(stmt);
            }
            _ => {}
        }
        self.write_indent();
    }
}
