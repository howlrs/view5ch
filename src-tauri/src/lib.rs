use crate::responses::invoke;

mod responses;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            invoke::get_list,
            invoke::get_comments,
            invoke::save_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
