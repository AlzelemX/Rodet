//! DEVR+ backend.
//!
//! Everything the editor needs from the computer goes through [`Core::call`]:
//! files, the Rodet toolchain, the language server (rodet-analyzer), Cargo,
//! terminals, search and plugins. Results come back as JSON; things that happen later
//! (language server messages, terminal output, file changes) are sent as
//! events through the [`EventSink`].
//!
//! The same core runs inside the desktop app (Tauri) and inside
//! `devr-server`, which serves the editor over a WebSocket.

mod book;
mod cargo;
mod files;
mod lsp;
mod plugins;
mod search;
mod settings;
mod terminal;
mod toolchain;
mod updates;

use std::sync::Arc;

use anyhow::{Context, Result, bail};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

/// Receives events: `(name, payload)`.
pub type EventSink = Arc<dyn Fn(&str, Value) + Send + Sync>;

/// The DEVR+ version: `0.1.N` in release builds (set by CI), else the crate's.
pub const VERSION: &str = match option_env!("DEVR_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

pub struct Core {
    sink: EventSink,
    lsp: lsp::Lsp,
    terminals: terminal::Terminals,
    watcher: files::Watcher,
}

impl Core {
    pub fn new(sink: EventSink) -> Arc<Core> {
        Arc::new(Core {
            lsp: lsp::Lsp::new(sink.clone()),
            terminals: terminal::Terminals::new(sink.clone()),
            watcher: files::Watcher::new(sink.clone()),
            sink,
        })
    }

    /// Sends an event to the editor.
    pub fn emit(&self, event: &str, payload: Value) {
        (self.sink)(event, payload)
    }

    /// Runs one request from the editor.
    pub async fn call(self: &Arc<Self>, method: &str, params: Value) -> Result<Value> {
        let core = self.clone();
        let method = method.to_owned();
        // File system and process work is blocking: keep it off the async threads.
        match method.as_str() {
            "lsp.start" | "lsp.send" | "lsp.stop" => self.call_async(&method, params).await,
            _ => tokio::task::spawn_blocking(move || core.call_blocking(&method, params))
                .await
                .context("request panicked")?,
        }
    }

    async fn call_async(&self, method: &str, params: Value) -> Result<Value> {
        match method {
            "lsp.start" => {
                let p: lsp::StartParams = parse(params)?;
                self.lsp.start(p).await
            }
            "lsp.send" => {
                let p: lsp::SendParams = parse(params)?;
                self.lsp.send(&p.messages).await?;
                Ok(Value::Null)
            }
            "lsp.stop" => {
                self.lsp.stop().await;
                Ok(Value::Null)
            }
            _ => unreachable!(),
        }
    }

    fn call_blocking(&self, method: &str, params: Value) -> Result<Value> {
        Ok(match method {
            "app.info" => json!({
                "version": VERSION,
                "os": std::env::consts::OS,
                "arch": std::env::consts::ARCH,
                "home": dirs::home_dir(),
                "pathSeparator": std::path::MAIN_SEPARATOR.to_string(),
                "toolchain": toolchain::Toolchain::find(),
            }),

            "fs.readDir" => to_json(files::read_dir(&parse::<files::PathParam>(params)?.path)?)?,
            "fs.readFile" => to_json(files::read_file(&parse::<files::PathParam>(params)?.path)?)?,
            "fs.writeFile" => {
                let p: files::WriteParams = parse(params)?;
                files::write_file(&p.path, &p.text)?;
                Value::Null
            }
            "fs.createFile" => {
                files::create_file(&parse::<files::PathParam>(params)?.path)?;
                Value::Null
            }
            "fs.createDir" => {
                files::create_dir(&parse::<files::PathParam>(params)?.path)?;
                Value::Null
            }
            "fs.rename" => {
                let p: files::RenameParams = parse(params)?;
                files::rename(&p.from, &p.to)?;
                Value::Null
            }
            "fs.delete" => {
                files::delete(&parse::<files::PathParam>(params)?.path)?;
                Value::Null
            }
            "fs.stat" => to_json(files::stat(&parse::<files::PathParam>(params)?.path))?,
            "fs.watch" => {
                self.watcher.watch(&parse::<files::PathParam>(params)?.path)?;
                Value::Null
            }
            "fs.listFiles" => to_json(files::list_files(&parse::<files::PathParam>(params)?.path))?,

            "book.info" => to_json(book::info())?,
            "book.read" => Value::String(book::read(&parse(params)?)?),
            "book.asset" => to_json(book::asset(&parse(params)?)?)?,

            "search.text" => to_json(search::search(parse(params)?)?)?,

            "term.spawn" => to_json(self.terminals.spawn(parse(params)?)?)?,
            "term.write" => {
                let p: terminal::WriteParams = parse(params)?;
                self.terminals.write(p.id, &p.data)?;
                Value::Null
            }
            "term.resize" => {
                let p: terminal::ResizeParams = parse(params)?;
                self.terminals.resize(p.id, p.cols, p.rows)?;
                Value::Null
            }
            "term.kill" => {
                self.terminals.kill(parse::<terminal::IdParam>(params)?.id);
                Value::Null
            }

            "cargo.project" => to_json(cargo::project(&parse::<files::PathParam>(params)?.path)?)?,
            "cargo.new" => {
                let p: cargo::NewParams = parse(params)?;
                to_json(cargo::new_project(&p)?)?
            }
            "cargo.latestVersion" => {
                let p: cargo::CrateParam = parse(params)?;
                to_json(cargo::latest_version(&p.name)?)?
            }

            "settings.get" => settings::load(),
            "settings.set" => {
                settings::save(&params)?;
                Value::Null
            }
            "recent.get" => to_json(settings::recent())?,
            "recent.add" => {
                settings::add_recent(&parse::<files::PathParam>(params)?.path)?;
                Value::Null
            }

            "updates.check" => to_json(updates::check()?)?,

            "plugins.dir" => to_json(plugins::dir())?,
            "plugins.list" => to_json(plugins::list())?,
            "plugins.read" => Value::String(plugins::read(&parse(params)?)?),
            "plugins.install" => Value::String(plugins::install(&parse(params)?)?),
            "plugins.create" => to_json(plugins::create(&parse(params)?)?)?,
            "plugins.remove" => {
                plugins::remove(&parse::<plugins::IdParam>(params)?.id)?;
                Value::Null
            }
            "plugins.dataGet" => plugins::data_get(&parse::<plugins::IdParam>(params)?.id)?,
            "plugins.dataSet" => {
                plugins::data_set(&parse(params)?)?;
                Value::Null
            }

            other => bail!("unknown request `{other}`"),
        })
    }

    /// Stops the language server and all terminals.
    pub async fn shutdown(&self) {
        self.lsp.stop().await;
        self.terminals.kill_all();
    }
}

fn parse<T: DeserializeOwned>(params: Value) -> Result<T> {
    serde_json::from_value(params).context("bad request parameters")
}

fn to_json<T: serde::Serialize>(value: T) -> Result<Value> {
    Ok(serde_json::to_value(value)?)
}
