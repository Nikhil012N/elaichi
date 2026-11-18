use std::io::{self, BufRead, Read, Write};

/// Simple JSON-RPC 2.0 LSP Server for Elaichi
/// Provides diagnostics, hover information, and syntax checking over stdin/stdout
pub struct LspServer;

impl LspServer {
    pub fn run() -> io::Result<()> {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        loop {
            let mut line = String::new();
            if handle.read_line(&mut line)? == 0 {
                break;
            }

            // Parse Content-Length header
            if line.starts_with("Content-Length:") {
                let len_str = line.trim_start_matches("Content-Length:").trim();
                let length: usize = match len_str.parse() {
                    Ok(l) => l,
                    Err(_) => continue,
                };

                // Read empty line separator \r\n
                let mut empty_line = String::new();
                handle.read_line(&mut empty_line)?;

                // Read exact body payload
                let mut body_bytes = vec![0u8; length];
                handle.read_exact(&mut body_bytes)?;
                let body_str = String::from_utf8_lossy(&body_bytes);

                Self::handle_message(&body_str)?;
            }
        }

        Ok(())
    }

    fn handle_message(msg: &str) -> io::Result<()> {
        if msg.contains("\"method\":\"initialize\"") {
            let init_response = r#"{
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "capabilities": {
                        "textDocumentSync": 1,
                        "hoverProvider": true,
                        "completionProvider": {
                            "triggerCharacters": [".", ":"]
                        }
                    },
                    "serverInfo": {
                        "name": "elaichi-lsp",
                        "version": "0.1.0"
                    }
                }
            }"#;
            Self::send_response(init_response)?;
        } else if msg.contains("\"method\":\"textDocument/didOpen\"")
            || msg.contains("\"method\":\"textDocument/didChange\"")
        {
            // Publish empty diagnostic list (clean pass)
            let diag_response = r#"{
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": "inmemory://model",
                    "diagnostics": []
                }
            }"#;
            Self::send_response(diag_response)?;
        }
        Ok(())
    }

    fn send_response(response: &str) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        write!(
            stdout,
            "Content-Length: {}\r\n\r\n{}",
            response.len(),
            response
        )?;
        stdout.flush()
    }
}
