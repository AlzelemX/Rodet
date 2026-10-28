//! Terminals: a shell, or a command such as `cargo run`, in a pseudo
//! terminal so colors and progress bars work like in a real terminal.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::EventSink;
use crate::toolchain::search_path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnParams {
    pub cwd: PathBuf,
    pub cols: u16,
    pub rows: u16,
    /// The program and its arguments; the user's shell when empty.
    #[serde(default)]
    pub command: Vec<String>,
}

#[derive(Serialize)]
pub struct Spawned {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct WriteParams {
    pub id: u32,
    pub data: String,
}

#[derive(Deserialize)]
pub struct ResizeParams {
    pub id: u32,
    pub cols: u16,
    pub rows: u16,
}

#[derive(Deserialize)]
pub struct IdParam {
    pub id: u32,
}

struct Terminal {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    killer: Box<dyn portable_pty::ChildKiller + Send + Sync>,
}

pub struct Terminals {
    sink: EventSink,
    next_id: AtomicU32,
    open: Arc<Mutex<HashMap<u32, Terminal>>>,
}

impl Terminals {
    pub fn new(sink: EventSink) -> Terminals {
        Terminals { sink, next_id: AtomicU32::new(1), open: Default::default() }
    }

    pub fn spawn(&self, params: SpawnParams) -> Result<Spawned> {
        let pty = native_pty_system();
        let pair = pty
            .openpty(PtySize { rows: params.rows.max(2), cols: params.cols.max(10), pixel_width: 0, pixel_height: 0 })
            .context("can't open a terminal")?;

        let mut cmd = match params.command.split_first() {
            Some((program, args)) => {
                let mut cmd = CommandBuilder::new(program);
                cmd.args(args);
                cmd
            }
            None => CommandBuilder::new(default_shell()),
        };
        cmd.cwd(&params.cwd);
        cmd.env("PATH", search_path());
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        cmd.env("CARGO_TERM_COLOR", "always");
        cmd.env("RODET_LANG", crate::toolchain::language());

        let mut child = pair.slave.spawn_command(cmd).context("can't start the command")?;
        drop(pair.slave);
        let killer = child.clone_killer();
        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.open.lock().unwrap().insert(id, Terminal { master: pair.master, writer, killer });

        let sink = self.sink.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; 16 * 1024];
            let mut pending: Vec<u8> = Vec::new();
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        pending.extend_from_slice(&buf[..n]);
                        // Send only whole UTF-8 characters; keep a cut one for the next read.
                        let valid = match std::str::from_utf8(&pending) {
                            Ok(_) => pending.len(),
                            Err(err) if err.error_len().is_none() => err.valid_up_to(),
                            Err(_) => pending.len(),
                        };
                        let text = String::from_utf8_lossy(&pending[..valid]).into_owned();
                        pending.drain(..valid);
                        sink("term.data", json!({ "id": id, "data": text }));
                    }
                }
            }
        });

        let sink = self.sink.clone();
        let open = self.open.clone();
        std::thread::spawn(move || {
            let code = child.wait().map(|status| status.exit_code()).unwrap_or(1);
            // Let the reader send the last output first.
            std::thread::sleep(std::time::Duration::from_millis(100));
            open.lock().unwrap().remove(&id);
            sink("term.exit", json!({ "id": id, "code": code }));
        });

        Ok(Spawned { id })
    }

    pub fn write(&self, id: u32, data: &str) -> Result<()> {
        let mut open = self.open.lock().unwrap();
        let terminal = open.get_mut(&id).context("the terminal is closed")?;
        terminal.writer.write_all(data.as_bytes())?;
        terminal.writer.flush()?;
        Ok(())
    }

    pub fn resize(&self, id: u32, cols: u16, rows: u16) -> Result<()> {
        let open = self.open.lock().unwrap();
        let terminal = open.get(&id).context("the terminal is closed")?;
        terminal.master.resize(PtySize { rows: rows.max(2), cols: cols.max(10), pixel_width: 0, pixel_height: 0 })?;
        Ok(())
    }

    pub fn kill(&self, id: u32) {
        if let Some(mut terminal) = self.open.lock().unwrap().remove(&id) {
            let _ = terminal.killer.kill();
        }
    }

    pub fn kill_all(&self) {
        for (_, mut terminal) in self.open.lock().unwrap().drain() {
            let _ = terminal.killer.kill();
        }
    }
}

fn default_shell() -> String {
    if cfg!(windows) {
        "powershell.exe".to_owned()
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| {
            if cfg!(target_os = "macos") { "/bin/zsh".to_owned() } else { "/bin/bash".to_owned() }
        })
    }
}
