use super::*;
use std::io;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

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
fn responds_to_initialize() {
    let (client, server) = create_client_and_server();
    let t = std::thread::spawn(move || {
        run(server);
    });

    client
        .send(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": serde_json::Value::Null,
                "rootUri": serde_json::Value::Null,
                "capabilities": {
                },
            },
        }))
        .unwrap();
    client.recv().unwrap();
    client
        .send(json!({
            "jsonrpc": "2.0",
            "method": "exit",
        }))
        .unwrap();

    t.join().unwrap();
}

#[test]
fn document_hightlight_highlights_the_same_variable() {
    // TODO: This has to be refactor to make writing tests easy.
    let (client, server) = create_client_and_server();
    let t = std::thread::spawn(move || {
        run(server);
    });

    // Initialize
    client
        .send(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": serde_json::Value::Null,
                "rootUri": serde_json::Value::Null,
                "capabilities": {
                },
            },
        }))
        .unwrap();
    // Receive initialized
    // TODO: verify this.
    client.recv().unwrap();

    // Open document (notification)
    client
        .send(json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///home/user/test.vim",
                    "languageId": "vim",
                    "version": 1,
                    "text": "let myvar = 1\nlet myvar = 2\n",
                },
            },
        }))
        .unwrap();
    // Diagnostic notification
    client.recv().unwrap();

    // Request hightlights
    client
        .send(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "textDocument/documentHighlight",
            "params": {
                "textDocument": {
                    "uri": "file:///home/user/test.vim",
                },
                "position": {
                    "line": 0,
                    "character": 5,
                },
            },
        }))
        .unwrap();
    let response = client.recv().unwrap();
    let result = response.get("result").unwrap().clone();
    let x: Vec<DocumentHighlight> = serde_json::from_value(result).unwrap();
    // TODO: This is invalid response, we should actually report both variables not just the
    // first one.
    assert_eq!(
        x,
        vec![DocumentHighlight {
            kind: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 4,
                },
                end: Position {
                    line: 0,
                    character: 9,
                },
            },
        }]
    );

    // Exit
    client
        .send(json!({
            "jsonrpc": "2.0",
            "method": "exit",
        }))
        .unwrap();

    t.join().unwrap();
}
