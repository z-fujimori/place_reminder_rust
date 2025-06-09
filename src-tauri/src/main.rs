// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;
pub(crate) mod database;
use crate::database::data;


fn main() {
    // db作成
    use tauri::async_runtime::block_on;
    const DATABASE_FILE: &str = "db.sqlite";
    let app_dir = ProjectDirs::from("com", "dev", "mokumoku")
        .ok_or("プロジェクトディレクトリの取得に失敗しました")?;
    let database_dir = app_dir.data_dir();
    // データベースファイルが存在するかチェックする
    let db_exists = database_dir.exists();
    // 存在しないなら、ファイルを格納するためのディレクトリを作成する
    if !db_exists {
        std::fs::create_dir(&database_dir)?;
    }
    // データベースURLを作成する
    let database_dir_str = dunce::canonicalize(&database_dir)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let database_url = format!("sqlite://{}/{}", database_dir_str, DATABASE_FILE); 
    // SQLiteのコネクションプールを作成する
    let sqlite_pool = block_on(data::create_sqlite_pool(&database_url))?;
    //  データベースファイルが存在しなかったなら、マイグレーションSQLを実行する
    if !db_exists {
        block_on(data::migrate_database(&sqlite_pool))?;
        println!("db作成");
    }

    place_reminder_rust_lib::run()
}
