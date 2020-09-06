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

use crate::lexer::SourceLocation;
use crate::lexer::TokenType;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Stmt {
    pub kind: StmtKind,
}

impl Stmt {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return self.kind.dump_for_testing();
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum StmtKind {
    Let(LetStatement),
    Call(CallStatement),
    Execute(ExecuteStatement),
    Return(ReturnStatement),
    If(IfStatement),
    While(WhileStatement),
    Function(FunctionStatement),
    For(ForStatement),
    Try(TryStatement),
    Set(SetStatement),
    Break(BreakStatement),
}

impl StmtKind {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return match &self {
            StmtKind::Let(x) => json!({ "let": x.dump_for_testing() }),
            StmtKind::If(x) => json!({ "if": x.dump_for_testing() }),
            StmtKind::Call(x) => json!({ "call": x.dump_for_testing() }),
            StmtKind::Return(x) => json!({ "return": x.dump_for_testing() }),
            StmtKind::While(x) => json!({ "while": x.dump_for_testing() }),
            StmtKind::Function(x) => json!({ "function": x.dump_for_testing() }),
            StmtKind::Try(x) => json!({ "try": x.dump_for_testing() }),
            StmtKind::Set(x) => json!({ "set": x.dump_for_testing() }),
            StmtKind::Break(x) => json!({ "break": x.dump_for_testing() }),
            _ => json!({}),
        };
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct LetStatement {
    pub var: Box<ExprKind>,
    pub operator: TokenType,
    pub value: Box<ExprKind>,
}

impl LetStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "var": self.var.dump_for_testing(),
            "operator": self.operator.as_str(),
            "value": self.value.dump_for_testing(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CallStatement {
    pub name: String,
    pub arguments: Vec<ExprKind>,
}

impl CallStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "method": self.name,
            "arguments": self.arguments.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct BreakStatement {}

impl BreakStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({});
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteStatement {
    pub arguments: Vec<ExprKind>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct FunctionStatement {
    pub name: String,
    // TODO change to list of tokens?
    pub arguments: Vec<String>,
    pub body: Vec<Stmt>,
    // true if 'function!'
    pub overwrite: bool,
    pub abort: bool,
}

impl FunctionStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "name": self.name,
            "arguments": self.arguments,
            "body": self.body.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
            "overwrite": self.overwrite,
            "abort": self.abort,
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ForStatement {
    pub loop_variable: LoopVariable,
    pub range: ExprKind,
    pub body: Vec<Stmt>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum LoopVariable {
    Single(String),
    List(Vec<String>),
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub value: Option<ExprKind>,
}

impl ReturnStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        match &self.value {
            Some(value) => return json!({ "value": value.dump_for_testing() }),
            None => return json!({}),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct SetStatement {
    pub option: String,
    pub value: Option<String>,
}

impl SetStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!(self);
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ElseCond {
    None,
    Else(Vec<Stmt>),
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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: ExprKind,
    pub then: Vec<Stmt>,
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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct TryStatement {
    pub body: Vec<Stmt>,
    pub finally: Option<Vec<Stmt>>,
}

impl TryStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        match self.finally.as_ref() {
            None => {
                return json!({
                    "body": self.body.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
                });
            }
            Some(f) => {
                return json!({
                    "body": self.body.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
                    "finally": f.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
                });
            }
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: ExprKind,
    pub body: Vec<Stmt>,
}

impl WhileStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "condition": self.condition.dump_for_testing(),
            "body": self.body.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ExprKind {
    Identifier(IdentifierExpression),
    Number(NumberExpression),
    Infix(InfixExpression),
    // TODO: rename to Call?
    Function(FunctionExpression),
    StringLiteral(StringLiteralExpression),
    ArraySubscript(ArraySubscriptExpression),
    Array(ArrayExpression),
    Unary(UnaryExpression),
    Paren(ParenExpression),
    Choose(ChooseExpression),
    Dictionary(DictionaryExpression),
}

impl ExprKind {
    pub fn to_string(&self) -> String {
        match self {
            ExprKind::Number(expr) => format!("{}", expr.value),
            _ => format!("not implemented"),
        }
    }
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return match self {
            ExprKind::Number(e) => json!({"number":  e.dump_for_testing()}),
            ExprKind::Identifier(e) => json!({"identifier":  e.dump_for_testing()}),
            ExprKind::Function(e) => json!({"function":  e.dump_for_testing()}),
            ExprKind::StringLiteral(e) => json!({"stringLiteral":  e.dump_for_testing()}),
            ExprKind::Infix(e) => json!({"infix":  e.dump_for_testing()}),
            ExprKind::ArraySubscript(e) => json!({"arraySubscript":  e.dump_for_testing()}),
            ExprKind::Array(e) => json!({"array":  e.dump_for_testing()}),
            ExprKind::Unary(e) => json!({"unary":  e.dump_for_testing()}),
            ExprKind::Paren(e) => json!({"paren":  e.dump_for_testing()}),
            ExprKind::Choose(e) => json!({"choose":  e.dump_for_testing()}),
            ExprKind::Dictionary(e) => json!({"dictionary":  e.dump_for_testing()}),
        };
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct IdentifierExpression {
    pub name: String,
    pub name_location: SourceLocation,
}

impl IdentifierExpression {
    pub fn name(&self) -> &str {
        return &self.name;
    }
    pub fn name_location(&self) -> &SourceLocation {
        return &self.name_location;
    }
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!(self);
    }
}

impl Serialize for IdentifierExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct StringLiteralExpression {
    pub value: String,
}

impl StringLiteralExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!(self);
    }
}

impl Serialize for StringLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ParenExpression {
    pub expr: Box<ExprKind>,
}

impl ParenExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return self.expr.dump_for_testing();
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ChooseExpression {
    pub cond: Box<ExprKind>,
    pub lhs: Box<ExprKind>,
    pub rhs: Box<ExprKind>,
}

impl ChooseExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "cond": self.cond.dump_for_testing(),
            "lhs": self.lhs.dump_for_testing(),
            "rhs": self.rhs.dump_for_testing(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: TokenType,
    pub expr: Box<ExprKind>,
}

impl UnaryExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "operator": self.operator.as_str(),
            "expr": self.expr.dump_for_testing(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct InfixExpression {
    pub left: Box<ExprKind>,
    pub operator: TokenType,
    pub right: Box<ExprKind>,
}

impl InfixExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "left": self.left.dump_for_testing(),
            "operator": self.operator.as_str(),
            "right": self.right.dump_for_testing(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub callee: Box<ExprKind>,
    pub arguments: Vec<ExprKind>,
}

impl FunctionExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "callee": self.callee.dump_for_testing(),
            "arguments": self.arguments.iter().map(|a| a.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct NumberExpression {
    pub value: f64,
}

impl NumberExpression {
    pub fn value(&self) -> f64 {
        return self.value;
    }
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!(self.value);
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ArraySubscript {
    Index(ExprKind),
    Sublist(Sublist),
}

impl ArraySubscript {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return match self {
            ArraySubscript::Index(e) => json!({"index": e.dump_for_testing()}),
            ArraySubscript::Sublist(e) => json!({"sublist": e.dump_for_testing()}),
        };
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Sublist {
    pub left: Option<ExprKind>,
    pub right: Option<ExprKind>,
}

impl Sublist {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        if let Some(x) = &self.left {
            if let Some(y) = &self.right {
                return json!({
                    "left": x.dump_for_testing(),
                    "right": y.dump_for_testing()
                });
            }
            return json!({"left": x.dump_for_testing()});
        }
        if let Some(y) = &self.right {
            return json!({"right": y.dump_for_testing()});
        }
        return json!({});
    }
}

// Represents `base[idx]`
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ArraySubscriptExpression {
    pub base: Box<ExprKind>,
    pub idx: Box<ArraySubscript>,
}

impl ArraySubscriptExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "base": self.base.dump_for_testing(),
            "idx": self.idx.dump_for_testing(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ArrayExpression {
    pub elements: Vec<ExprKind>,
}

impl ArrayExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "elements": self.elements.iter().map(|e| e.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub key: String,
    pub value: ExprKind,
}

impl DictionaryEntry {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "key": self.key,
            "value": self.value.dump_for_testing(),
        });
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct DictionaryExpression {
    pub entries: Vec<DictionaryEntry>,
}

impl DictionaryExpression {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "entries": self.entries.iter().map(|e| e.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
        });
    }
}
