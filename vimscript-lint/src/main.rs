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

use std::env;
use std::fs;
use syntax::parse;

fn main() {
    let mut total_errors = 0;
    for filename in env::args().skip(1) {
        println!("{}", filename);
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut parsed = parse(&contents);
        for error in &parsed.errors {
            println!("{:?}", error);
        }
        total_errors += parsed.errors.len();
        println!("\nError count: {}", parsed.errors.len());
    }
    println!("\n\nTotal error count: {}", total_errors);
}
