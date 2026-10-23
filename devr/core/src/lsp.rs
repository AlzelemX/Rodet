//! Runs the language server (rust++-analyzer) and passes its messages to and
//! from the editor. The editor speaks the Language Server Protocol itself;
//! this side only frames the messages (`Content-Length` headers).

use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin};
use tokio::sync::Mutex;

use crate::EventSink;
use crate::toolchain::{Toolchain, search_path};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartParams {
    /// The project folder.
    pub root: PathBuf,
    /// A language server to use instead of the one found in the toolchain.
    #[serde(default)]
    pub server: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct SendParams {
    pub message: String,
}

struct Running {
    child: Child,
    stdin: ChildStdin,
}

pub struct Lsp {
    sink: EventSink,
    running: Mutex<Option<Running>>,
    generation: std::sync::atomic::AtomicU64,
}

impl Lsp {
    pub fn new(sink: EventSink) -> Lsp {
        Lsp { sink, running: Mutex::new(None), generation: Default::default() }
    }

    /// Starts the language server for `root`, stopping the previous one.
    pub async fn start(&self, params: StartParams) -> Result<Value> {
        self.stop().await;
        let server = match params.server {
            Some(server) => server,
            None => match Toolchain::find().analyzer {
                Some(server) => server,
                None => bail!(
                    "rust++-analyzer was not found. Install or update Rust++ to get errors, completion and search."
                ),
            },
        };

        let mut cmd = tokio::process::Command::new(&server);
        cmd.current_dir(&params.root)
            .env("PATH", search_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        #[cfg(windows)]
        cmd.creation_flags(0x0800_0000);
        let mut child =
            cmd.spawn().with_context(|| format!("can't start {}", server.display()))?;
        let stdin = child.stdin.take().context("no stdin")?;
        let stdout = child.stdout.take().context("no stdout")?;
        let stderr = child.stderr.take().context("no stderr")?;

        let generation = self.generation.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        let sink = self.sink.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            loop {
                match read_message(&mut reader).await {
                    Ok(Some(message)) => {
                        sink("lsp.message", json!({ "generation": generation, "message": message }))
                    }
                    Ok(None) => break,
                    Err(err) => {
                        sink("lsp.log", json!({ "line": format!("DEVR+: bad message from the language server: {err}") }));
                        break;
                    }
                }
            }
            sink("lsp.exit", json!({ "generation": generation }));
        });
        let sink = self.sink.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                sink("lsp.log", json!({ "line": line }));
            }
        });

        *self.running.lock().await = Some(Running { child, stdin });
        Ok(json!({ "server": server, "generation": generation }))
    }

    /// Sends one JSON-RPC message to the language server.
    pub async fn send(&self, message: &str) -> Result<()> {
        let mut running = self.running.lock().await;
        let Some(running) = running.as_mut() else { bail!("the language server is not running") };
        let header = format!("Content-Length: {}\r\n\r\n", message.len());
        running.stdin.write_all(header.as_bytes()).await?;
        running.stdin.write_all(message.as_bytes()).await?;
        running.stdin.flush().await?;
        Ok(())
    }

    pub async fn stop(&self) {
        if let Some(mut running) = self.running.lock().await.take() {
            let _ = running.child.start_kill();
            let _ = running.child.wait().await;
        }
    }
}

/// Reads one message: headers, an empty line, then `Content-Length` bytes.
async fn read_message<R: tokio::io::AsyncBufRead + Unpin>(reader: &mut R) -> Result<Option<String>> {
    let mut length = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).await? == 0 {
            return Ok(None);
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some(value) = line.strip_prefix("Content-Length:") {
            length = Some(value.trim().parse::<usize>()?);
        }
    }
    let length = length.context("message without Content-Length")?;
    let mut body = vec![0; length];
    reader.read_exact(&mut body).await?;
    Ok(Some(String::from_utf8(body)?))
}

#[cfg(test)]
mod tests {
    use super::read_message;

    #[tokio::test]
    async fn reads_framed_messages() {
        let input = b"Content-Length: 7\r\nContent-Type: x\r\n\r\n{\"a\":1}Content-Length: 2\r\n\r\n{}";
        let mut reader = tokio::io::BufReader::new(&input[..]);
        assert_eq!(read_message(&mut reader).await.unwrap().as_deref(), Some("{\"a\":1}"));
        assert_eq!(read_message(&mut reader).await.unwrap().as_deref(), Some("{}"));
        assert_eq!(read_message(&mut reader).await.unwrap(), None);
    }
}
