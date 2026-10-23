//! The DEVR+ desktop app: a window with the editor UI (ui/dist), talking to
//! devr-core through one command (`devr_call`) and one event (`devr-event`).

use std::sync::Arc;

use devr_core::Core;
use serde_json::{Value, json};
use tauri::{Emitter, Manager, State};

struct App(Arc<Core>);

#[tauri::command]
async fn devr_call(app: State<'_, App>, method: String, params: Value) -> Result<Value, String> {
    app.0.call(&method, params).await.map_err(|err| format!("{err:#}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let core = Core::new(Arc::new(move |event: &str, payload: Value| {
                let _ = handle.emit("devr-event", json!({ "event": event, "payload": payload }));
            }));
            app.manage(App(core));
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let core = window.state::<App>().0.clone();
                tauri::async_runtime::block_on(core.shutdown());
            }
        })
        .invoke_handler(tauri::generate_handler![devr_call])
        .run(tauri::generate_context!())
        .expect("DEVR+ could not start");
}
