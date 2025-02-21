// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::{get_horse_db, get_main_db_conn, get_stable_db, init_user_table};
use horse_stable::{Horse, HorseCreate, Stable, StableCreate, User};
use services::{
    create_horse, create_user, delete_horse, get_all_horses, get_horse_by_id, get_user_by_login,
    has_user, update_horse,
};
use tauri::{async_runtime, App, Manager, State};
use tokio::sync::Mutex;
mod db;
mod services;

type Result<T> = std::result::Result<T, String>;

#[derive(Default)]
pub struct AppStateInner {
    pub user_id: String,
}

pub type AppState<'a> = State<'a, Mutex<AppStateInner>>;

fn map_err(e: libsql::Error) -> String {
    e.to_string()
}

#[tauri::command]
async fn get_current_user(state: AppState<'_>) -> Result<Option<User>> {
    let app_state = state.lock().await;

    let user_id = app_state.user_id.clone();
    let conn = get_main_db_conn().await.unwrap();

    services::get_user_by_id(user_id, &conn)
        .await
        .map_err(map_err)
}

#[tauri::command]
async fn create_stable(state: AppState<'_>, stable: StableCreate) -> Result<Option<Stable>> {
    let conn = get_stable_db(state).await.map_err(map_err)?;

    services::create_stable(stable, &conn)
        .await
        .map_err(map_err)
}

#[tauri::command]
async fn list_stables(state: AppState<'_>) -> Result<Vec<Stable>> {
    let conn = get_stable_db(state).await.map_err(map_err)?;

    services::list_stables(&conn).await.map_err(map_err)
}

#[tauri::command]
async fn get_stable(state: AppState<'_>, id: u32) -> Result<Option<Stable>> {
    let conn = get_stable_db(state).await.map_err(map_err)?;

    services::get_stable(id, &conn).await.map_err(map_err)
}

#[tauri::command]
async fn get_horse(state: AppState<'_>, id: u32) -> Result<Option<Horse>> {
    let conn = get_horse_db(state).await.map_err(map_err)?;

    get_horse_by_id(id, &conn).await.map_err(map_err)
}

#[tauri::command]
async fn list_all_horses(state: AppState<'_>) -> Result<Vec<Horse>> {
    let conn = get_horse_db(state).await.unwrap();

    get_all_horses(&conn).await.map_err(map_err)
}

#[tauri::command]
async fn add_horse(
    state: AppState<'_>,
    stable_id: u32,
    horse: HorseCreate,
) -> Result<Option<Horse>> {
    let conn = get_horse_db(state).await.map_err(map_err)?;

    create_horse(stable_id, horse, &conn).await.map_err(map_err)
}

#[tauri::command]
async fn edit_horse(state: AppState<'_>, horse: Horse) -> Result<Option<Horse>> {
    let conn = get_horse_db(state).await.map_err(map_err)?;

    update_horse(horse, &conn).await.map_err(map_err)
}

#[tauri::command]
async fn remove_horse(state: AppState<'_>, id: String) -> Result<bool> {
    let conn = get_horse_db(state).await.unwrap();

    Ok(delete_horse(id, &conn).await.is_ok())
}

#[tauri::command]
async fn register_user(state: AppState<'_>, user: User) -> Result<Option<User>> {
    println!("Registering user {:?}", user);

    let conn = get_main_db_conn().await.unwrap();

    let email = user.email.clone();
    if has_user(email, &conn).await {
        return Err("User already exists".to_string());
    }

    let res = create_user(user, &conn).await;

    if let Ok(Some(ref user)) = res {
        state.lock().await.user_id = user.id.clone();
    };

    res.map_err(|e| e.to_string())
}

#[tauri::command]
async fn login(state: AppState<'_>, email: String, password: String) -> Result<User> {
    let conn = get_main_db_conn().await.unwrap();

    if let Ok(Some(user)) = get_user_by_login(email, password, &conn).await {
        state.lock().await.user_id = user.id.clone();
        Ok(user)
    } else {
        Err("Invalid credentials".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            login,
            add_horse,
            register_user,
            get_horse,
            list_all_horses,
            remove_horse,
            edit_horse,
            create_stable,
            list_stables,
            get_stable,
            get_current_user,
        ])
        .setup(|app| async_runtime::block_on(setup(app)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Setting up");
    // this is bad i guess,
    // i need to just use migrations

    app.manage(Mutex::new(AppStateInner::default()));

    let conn = get_main_db_conn().await?;
    println!("Setting up db");
    init_user_table(&conn).await?;

    Ok(())
}
