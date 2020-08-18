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

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::io;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

/// The Read trait allows for reading utf-8 packets from a source.
pub trait Read {
    fn read_packet(&mut self) -> Result<String, io::Error>;
}

/// The Write trait allows for writing utf-8 packets to a destination.
pub trait Write {
    fn write_packet(&self, packet: String) -> Result<(), io::Error>;
}

pub enum Message {
    Request(Request),
    Notification(Notification),
}

pub struct Request {
    pub method: String,
    pub params: serde_json::Value,
    pub response_handle: ResponseHandle,
}

pub struct Notification {
    pub method: String,
    pub params: serde_json::Value,
}

pub struct ResponseHandle {
    id: Id,
    writer: Arc<Mutex<dyn Write + Send>>,
}

impl ResponseHandle {
    pub fn respond(self, response: Result<serde_json::Value, serde_json::Value>) {
        // TODO: Improve error handling if responding fails.
        self.writer
            .lock()
            .unwrap()
            .write_packet(match response {
                Ok(result) => {
                    json!({ "jsonrpc": "2.0", "id": self.id, "result": result}).to_string()
                }
                Err(error) => json!({ "jsonrpc": "2.0", "id": self.id, "error": error}).to_string(),
            })
            .unwrap();
    }
}

/// The LspSender allows to send messages (requests and notification) to the client.
pub struct LspSender {
    next_id: Arc<Mutex<Counter>>,
    writer: Arc<Mutex<dyn Write + Send>>,
    running_requests: Weak<Mutex<MyMap>>,
}

impl LspSender {
    pub fn send_notification(&self, method: &str, params: serde_json::Value) {
        // TODO: how to properly handle errors here?
        self.writer
            .lock()
            .unwrap()
            .write_packet(
                json!(
                {
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": params}
                )
                .to_string(),
            )
            .unwrap();
    }

    pub fn send_request(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let running_requests = match self.running_requests.upgrade() {
            Some(x) => x,
            None => panic!("failed to upgrade running_requests"),
        };
        let id: Id = Id::Number(self.next_id.lock().unwrap().next());
        let (sender, receiver) = channel();
        running_requests.lock().unwrap().insert(id.clone(), sender);
        // TODO: how to properly handle errors here?
        self.writer
            .lock()
            .unwrap()
            .write_packet(
                json!(
                {
                    "jsonrpc": "2.0",
                    "id": id,
                    "method": method,
                    "params": params}
                )
                .to_string(),
            )
            .unwrap();
        return receiver.recv().unwrap();
    }
}

type ResultOrError = Result<serde_json::Value, serde_json::Value>;
type MyMap = HashMap<Id, Sender<ResultOrError>>;

#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(untagged)]
enum Id {
    Number(i64),
    String(String),
}

/// Basic implementation of LSP Server.
///
/// Server is responsible for abstracting the communication between the client and the server. The
/// server:
/// * serializes and deserializes packets into proper LSP Messages,
/// * hides the concept of request ID by providing APIs to send new requests, reply to messages
///   and receive responses,
/// * handles the `exit` notification, to stop the iterator from receiving any more messages.
///
/// It exits after receiving `exit` notification and forwards all other requests and responses to
/// handler passed in run method.
///
/// TODO: Server also returns when error is encountered, but errors are not properly reported yet.
pub struct Server<R: Read, W: Write> {
    reader: R,
    writer: Arc<Mutex<W>>,
    // Map of requests that are currently waiting for the response from client.
    running_requests: Arc<Mutex<MyMap>>,
}

impl<R, W> Iterator for Server<R, W>
where
    R: Read,
    W: Write + Send + 'static,
{
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        loop {
            let packet = match self.reader.read_packet() {
                Ok(packet) => packet,
                // TODO: Save the error
                Err(_) => return None,
            };
            let json: serde_json::Value = match serde_json::from_str(&packet) {
                Ok(value) => value,
                // TODO: We should probably reply with error?
                Err(_) => return None,
            };
            match &json {
                serde_json::Value::Object(map) => {
                    if let Some(serde_json::Value::String(method)) = map.get("method") {
                        if method == "exit" {
                            return None;
                        }
                        if let Some(id_val) = map.get("id") {
                            let id: Id = serde_json::from_value(id_val.clone()).unwrap();
                            return Some(Message::Request(Request {
                                method: method.to_string(),
                                params: json["params"].clone(),
                                response_handle: ResponseHandle {
                                    id: id,
                                    writer: self.writer.clone(),
                                },
                            }));
                        }
                        return Some(Message::Notification(Notification {
                            method: method.to_string(),
                            params: json["params"].clone(),
                        }));
                    }
                    if let Some(result) = map.get("result") {
                        if let Some(id_val) = map.get("id") {
                            let id: Id = serde_json::from_value(id_val.clone()).unwrap();
                            self.running_requests.lock().unwrap()[&id]
                                .send(Ok(result.clone()))
                                .unwrap();
                            continue;
                        }
                    }
                    // TODO: I think we should just respond with error here.
                    return None;
                }
                // TODO: I think we should just respond with error here.
                _ => return None,
            }
        }
    }
}

impl<R, W> Server<R, W>
where
    R: Read,
    W: Write + Send + 'static,
{
    pub fn new(reader: R, writer: W) -> Server<R, W> {
        return Server {
            reader: reader,
            writer: Arc::new(Mutex::new(writer)),
            running_requests: Arc::new(Mutex::new(HashMap::new())),
        };
    }

    pub fn sender(&self) -> LspSender {
        return LspSender {
            next_id: Arc::new(Mutex::new(Counter::new())),
            writer: self.writer.clone(),
            running_requests: Arc::downgrade(&self.running_requests),
        };
    }
}

struct Counter {
    id: i64,
}

impl Counter {
    fn new() -> Counter {
        Counter { id: 0 }
    }

    fn next(&mut self) -> i64 {
        self.id += 1;
        return self.id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::Receiver;

    struct FakeReader {
        receiver: Receiver<String>,
    }

    impl FakeReader {
        fn new() -> (Sender<String>, FakeReader) {
            let (sender, receiver) = channel();
            return (sender, FakeReader { receiver: receiver });
        }
    }

    impl Read for FakeReader {
        fn read_packet(&mut self) -> Result<String, io::Error> {
            self.receiver
                .recv()
                .map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "EOF encountered"))
        }
    }
    struct FakeWriter {
        sender: Sender<String>,
    }

    impl FakeWriter {
        fn new() -> (Receiver<String>, FakeWriter) {
            let (sender, receiver) = channel();
            return (receiver, FakeWriter { sender: sender });
        }
    }

    impl Write for FakeWriter {
        fn write_packet(&self, packet: String) -> Result<(), io::Error> {
            self.sender.send(packet).unwrap();
            Ok(())
        }
    }

    struct Client {
        sender: Sender<String>,
        receiver: Receiver<String>,
    }

    impl Client {
        fn recv(&self) -> Result<serde_json::Value, ()> {
            Ok(self.receiver.recv().unwrap().parse().unwrap())
        }
        fn send(&self, req: serde_json::Value) -> Result<(), ()> {
            self.sender.send(req.to_string()).unwrap();
            Ok(())
        }
    }

    fn exit_notification() -> serde_json::Value {
        return json!({
            "jsonrpc": "2.0",
            "method": "exit",
        });
    }

    fn create_client_and_server() -> (Client, Server<FakeReader, FakeWriter>) {
        let (writer_ch, writer) = FakeWriter::new();
        let (reader_ch, reader) = FakeReader::new();
        let client = Client {
            sender: reader_ch,
            receiver: writer_ch,
        };
        let server = Server::new(reader, writer);
        return (client, server);
    }

    #[test]
    fn server_exits_after_exit_notification() {
        let (client, server) = create_client_and_server();
        client.send(exit_notification()).unwrap();

        assert_eq!(server.count(), 0);
    }

    #[test]
    fn server_exits_when_reader_returns_eof() {
        let (client, server) = create_client_and_server();
        std::mem::drop(client);

        assert_eq!(server.count(), 0);
    }

    #[test]
    fn server_receives_notifications() {
        let notification = json!({
            "jsonrpc": "2.0",
            "method": "someMethod",
            "params": {
                "key": "value",
            }
        });
        let (client, mut server) = create_client_and_server();

        client.send(notification.clone()).unwrap();
        client.send(exit_notification()).unwrap();

        let message = server.next().unwrap();
        match message {
            Message::Notification(n) => {
                assert_eq!(n.method, "someMethod");
                assert_eq!(n.params, json!({"key": "value"}));
            }
            _ => panic!("invalid message received, want notification"),
        }
    }

    #[test]
    fn server_receives_requests() {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "someMethod",
            "params": {
                "key": "value",
            }
        });
        let (client, mut server) = create_client_and_server();

        client.send(request.clone()).unwrap();
        client.send(exit_notification()).unwrap();

        let message = server.next().unwrap();
        match message {
            Message::Request(r) => {
                assert_eq!(r.method, "someMethod");
                assert_eq!(r.params, json!({"key": "value"}));
                r.response_handle.respond(Ok(json!({"my": "response"})));
            }
            _ => panic!("invalid message received, want request"),
        }
        assert_eq!(
            client.recv().unwrap(),
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "my": "response",
                }
            })
        )
    }

    #[test]
    fn server_can_send_notifications() {
        let (client, server) = create_client_and_server();

        server
            .sender()
            .send_notification("someMethod", json!({"key": "value"}));

        assert_eq!(
            client.recv().unwrap(),
            json!({
                "jsonrpc": "2.0",
                "method": "someMethod",
                "params": {
                    "key": "value",
                }} )
        );
    }

    #[test]
    fn server_can_send_requests() {
        let (client, server) = create_client_and_server();

        let sender = server.sender();

        let t = std::thread::spawn(move || {
            let res = sender
                .send_request("someMethod", json!({"key": "value"}))
                .unwrap();
            assert_eq!(res, json!({"key1": "value1"}));
        });
        let t2 = std::thread::spawn(move || {
            // Just consume all items.
            server.count();
        });

        assert_eq!(
            client.recv().unwrap(),
            json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "someMethod",
            "params": {
                "key": "value",
            }})
        );
        client
            .send(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "key1": "value1",
                }
            }))
            .unwrap();
        client
            .send(json!({
                "jsonrpc": "2.0",
                "method": "exit",
            }))
            .unwrap();

        t.join().unwrap();
        t2.join().unwrap();
    }
}
