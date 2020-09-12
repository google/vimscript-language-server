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

extern crate vimscript_core;

use pretty_assertions::assert_eq;
use std::path::PathBuf;
use vimscript_core::format::format;
use vimscript_core::lexer::Lexer;
use vimscript_core::parser::Parser;

#[derive(Debug)]
struct TestCase {
    before: PathBuf,
    after: PathBuf,
}

#[derive(PartialEq, Eq)]
#[doc(hidden)]
pub struct PrettyString<'a>(pub &'a str);

/// Make diff to display string as multi-line string
impl<'a> std::fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

fn read_test_cases() -> Vec<TestCase> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/format");

    let mut entries = std::fs::read_dir(d)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    entries.sort();

    let before = entries
        .iter()
        .filter(|path| path.to_str().unwrap().ends_with(".before.vim"));
    let after = entries
        .iter()
        .filter(|path| path.to_str().unwrap().ends_with(".after.vim"));

    before
        .zip(after)
        .map(|pair| TestCase {
            before: pair.0.clone(),
            after: pair.1.clone(),
        })
        .collect()
}

#[test]
fn test_format() {
    println!("Running");
    for case in read_test_cases() {
        println!("Testing {:?}", case);
        let content = std::fs::read_to_string(&case.before).unwrap();
        let mut parser = Parser::new(Lexer::new(&content));
        let program = parser.parse();
        assert_eq!(parser.errors, vec![]);

        let formatted = format(&program);
        let expected = std::fs::read_to_string(&case.after).unwrap();
        assert_eq!(
            PrettyString(&formatted),
            PrettyString(&expected),
            "invalid formatting of {:?}",
            case.before.file_name()
        );
    }
}
