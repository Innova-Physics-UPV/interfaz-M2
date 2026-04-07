use tauri::{AppHandle, Emitter};

fn start_ui_chunking_actor(app_handle: AppHandle) {
    // TODO: Implement agrupation of data at 60HZ
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(move |app| {
            // TODO: Initiatate pipeline
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("Error al ejecutar Tauri");
}