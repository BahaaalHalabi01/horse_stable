// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use db::{
    add_horse_query, create_horse_table, create_user_table, delete_horse_query,
    get_all_horses_query, get_horse_by_id_query, init_db, update_horse_query,
};
use horse_stable::{Horse, User};
use services::{create_user, get_user_by_id, get_user_by_login, has_user};
use tauri::async_runtime;
mod db;
mod services;

// static MY_STABLE: Stable = Stable {
//     count: AtomicUsize::new(0),
// };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Is this real ? inside rust ? {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_horse_by_id(id: u32) -> Horse {
    let conn = init_db().await.unwrap();

    get_horse_by_id_query(id, &conn).await
}

#[tauri::command]
async fn get_all_horses() -> Result<Vec<Horse>, String> {
    let conn = init_db().await.unwrap();

    let horses = get_all_horses_query(&conn).await;

    Ok(horses)
}

#[tauri::command]
async fn add_horse(horse: Horse) -> Result<Horse, String> {
    let conn = init_db().await.unwrap();

    let mut clone = horse.clone();
    let added = add_horse_query(horse, &conn).await;

    clone.id = added;

    Ok(clone)
}

#[tauri::command]
async fn update_horse(horse: Horse) -> Result<Horse, String> {
    println!("{:?}", horse);
    let conn = init_db().await.unwrap();

    Ok(update_horse_query(horse, &conn).await)
}

#[tauri::command]
async fn delete_horse(id: u32) -> Result<bool, String> {
    let conn = init_db().await.unwrap();

    delete_horse_query(id, &conn).await;

    Ok(true)
}

#[tauri::command]
async fn register_user(user: User) -> Result<User, String> {
    println!("Registering user {:?}", user);
    let conn = init_db().await.unwrap();

    let email = user.email.clone();
    if has_user(email, &conn).await {
        return Err("User already exists".to_string());
    }

    Ok(create_user(user, &conn).await)
}

#[tauri::command]
async fn login(email: String, password: String) -> Result<User, String> {
    let conn = init_db().await.unwrap();

    match get_user_by_login(email, password, &conn).await {
        Some(user) => Ok(user),
        None => Err("Invalid credentials".to_string()),
    }
}

fn main() {
    //this is bad, i need to just use migrations
    async_runtime::spawn(async {
        println!("Starting up");
        let conn = init_db().await.unwrap();
        create_horse_table(&conn).await;
        create_user_table(&conn).await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            login,
            get_horse_by_id,
            add_horse,
            get_all_horses,
            delete_horse,
            update_horse,
            register_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
