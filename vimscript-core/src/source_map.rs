// Copyright 2020 Google LLC
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

use lsp_types::Url;
use std::collections::HashMap;

pub struct SourceMap {
    files: HashMap<Url, String>,
}

impl SourceMap {
    pub fn new() -> SourceMap {
        SourceMap { files: HashMap::new() }
    }

    pub fn add(&mut self, uri: &Url, content: String) {
        self.files.insert(uri.clone(), content);
    }

    pub fn get_content(&self, uri: &Url) -> Option<String> {
        Some(self.files.get(uri)?.to_string())
    }
}
