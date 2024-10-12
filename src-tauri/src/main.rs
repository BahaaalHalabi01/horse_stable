// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use db::{add_horse_query, init_db};
use horse_stable::{Horse, Stable};
use std::sync::atomic::AtomicUsize;
mod db;

static MY_STABLE: Stable = Stable {
    count: AtomicUsize::new(0),
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Is this real ? inside rust ? {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_horse(name: &str) -> Horse {
    Horse::new(name.to_string(), &MY_STABLE)
}

#[tauri::command]
async fn add_horse(horse: Horse) -> Result<Horse, String> {
    println!("adding horse {:?}", horse);
    let conn = init_db().await.unwrap();

    // let horse: Horse = serde_json::from_str(horse).expect("missing");

    let added = add_horse_query(horse, &conn).await;

    Ok(added)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, get_horse, add_horse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
