// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;
pub(crate) mod database;
use crate::database::data;


fn main() {
    tauri::Builder::<tauri::Wry>::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_opener::init::<tauri::Wry>())
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
