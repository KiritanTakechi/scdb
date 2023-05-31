#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scdb::{
    database::prelude::*,
    entity::prelude::{Course, RawCourse, RawStudent, Student},
};
use sea_orm::EntityTrait;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    if let Err(e) = InitDB().await {
        panic!("{}", e)
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![student_read_all, course_read_all])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/* 学生表 */
#[tauri::command(async)]
async fn student_read_all() -> Vec<RawStudent> {
    let db_pool = DB_POOL.read().await;

    let db = db_pool.as_ref().unwrap();

    Student::find().all(db).await.unwrap()
}
/* 课程表 */
#[tauri::command(async)]
async fn course_read_all() -> Vec<RawCourse> {
    let db_pool = DB_POOL.read().await;

    let db = db_pool.as_ref().unwrap();

    Course::find().all(db).await.unwrap()
}
