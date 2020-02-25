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

use std::io;
use vimscript_core::lsp::run;
use vimscript_core::protocol::read_message;
use vimscript_core::protocol::write_message;
use vimscript_core::server::Read;
use vimscript_core::server::Server;
use vimscript_core::server::Write;

struct Reader {}

impl Read for Reader {
    fn read_packet(&mut self) -> Result<String, io::Error> {
        read_message(&mut std::io::stdin().lock())
    }
}

struct Writer;

impl Write for Writer {
    fn write_packet(&self, packet: String) -> Result<(), io::Error> {
        write_message(&packet)
    }
}

fn main() {
    let server = Server::new(Reader {}, Writer {});
    run(server);
}
