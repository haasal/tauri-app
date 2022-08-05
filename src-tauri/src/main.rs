#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mac_window_ext::{ToolbarThickness, WindowExt};
use tauri::Manager;

mod mac_window_ext;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            win.set_transparent_titlebar(ToolbarThickness::Thick);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
