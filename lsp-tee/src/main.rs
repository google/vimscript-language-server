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

use chrono::prelude::*;
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

struct TeeReader<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R, W> Read for TeeReader<R, W>
where
    R: Read,
    W: Write,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.reader.read(buf);
        if let Ok(size) = res {
            if size > 0 {
                self.writer
                    .write(&buf[0..size])
                    .expect("failed to write to writer");
                self.writer.flush().expect("flush failed");
            }
        }
        return res;
    }
}

// Reads the content of the next message from given input.
//
// The input is expected to provide a message as described by "Base Protocol" of Language Server
// Protocol.
fn read_message<R: BufRead>(input: &mut R) -> Result<String, io::Error> {
    // Read in the "Content-Length: xx" part.
    let mut size: Option<usize> = None;
    loop {
        let mut buffer = String::new();
        input.read_line(&mut buffer)?;

        // End of input.
        if buffer.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "EOF encountered in the middle of reading LSP headers",
            ));
        }

        // Header section is finished, break from the loop.
        if buffer == "\r\n" {
            break;
        }

        let res: Vec<&str> = buffer.split(' ').collect();

        // Make sure header is valid.
        if res.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Header '{}' is malformed", buffer),
            ));
        }
        let header_name = res[0].to_lowercase();
        let header_value = res[1].trim();

        match header_name.as_ref() {
            "content-length:" => {
                size = Some(usize::from_str_radix(header_value, 10).map_err(|_e| {
                    io::Error::new(io::ErrorKind::InvalidData, "Couldn't read size")
                })?);
            }
            "content-type:" => {
                if header_value != "utf8" && header_value != "utf-8" {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Content type '{}' is invalid", header_value),
                    ));
                }
            }
            // Ignore unknown headers (specification doesn't say what to do in this case).
            _ => (),
        }
    }
    let size = match size {
        Some(size) => size,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message is missing 'content-length' header",
            ));
        }
    };

    let mut content = vec![0; size];
    input.read_exact(&mut content)?;

    String::from_utf8(content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[derive(Serialize)]
struct LspMessage {
    timestamp: String,
    direction: String,
    #[serde(rename = "jsonRpcMessage")]
    json_rpc_message: Value,
}

fn main() {
    let first_arg = env::args().skip(1).next()
        .expect("command line is missing '--' and the name of the command to run");
    let mut output = "lsp-tee.txt".to_string();
    if first_arg != "--" {
        output = first_arg;
    }
    let mut args = env::args().skip_while(|a| a != "--").skip(1);
    let cmd = args
        .next()
        .expect("command line is missing '--' and the name of the command to run");

    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    let child_stdout = child
        .stdout
        .take()
        .expect("failed to get stdout from child process");
    let child_stdin = child
        .stdin
        .take()
        .expect("failed to get stdin from child process");

    let file = File::create(&output).expect("failed to open lsp-tee file");
    let stdout_file = Arc::new(Mutex::new(file));
    let stdin_file = stdout_file.clone();

    let stdout_thread = thread::spawn(move || {
        let mut reader = BufReader::new(TeeReader {
            reader: child_stdout,
            writer: io::stdout(),
        });
        loop {
            let msg = read_message(&mut reader).expect("failed to read message from child stdout");
            let msg: Value = serde_json::from_str(&msg).expect("failed to parse json rpc message");
            let msg = LspMessage {
                timestamp: Utc::now().to_rfc3339(),
                direction: "SERVER_TO_CLIENT".to_string(),
                json_rpc_message: msg,
            };
            let mut w = &(*stdout_file.lock().unwrap());
            serde_json::to_writer(w, &msg).expect("failed to write message to file");
            w.write(b"\n").unwrap();
            w.flush().unwrap();
        }
    });
    let stdin_thread = thread::spawn(move || {
        let mut reader = BufReader::new(TeeReader {
            reader: io::stdin(),
            writer: child_stdin,
        });
        loop {
            let msg = read_message(&mut reader).expect("failed to read message from stdin");
            let msg: Value = serde_json::from_str(&msg).expect("failed to parse json rpc message");
            let msg = LspMessage {
                timestamp: Utc::now().to_rfc3339(),
                direction: "CLIENT_TO_SERVER".to_string(),
                json_rpc_message: msg,
            };
            let mut w = &(*stdin_file.lock().unwrap());
            serde_json::to_writer(w, &msg).expect("failed to write message to file");
            w.write(b"\n").unwrap();
            w.flush().unwrap();
        }
    });
    stdin_thread
        .join()
        .expect("failed to wait for the stdin thread to finish");
    stdout_thread
        .join()
        .expect("failed to wait for the stdout thread to finish");

    let _ecode = child.wait().expect("failed to wait on child");
}
