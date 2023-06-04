#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scdb::{
    database::prelude::*,
    entity::prelude::{
        Course, MutCourse, MutSC, MutStudent, RawCourse, RawSC, RawStudent, Student, SC,
    },
};
use sea_orm::EntityTrait;

#[tokio::main]
async fn main() {
    if let Err(e) = InitDB().await {
        panic!("{}", e)
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            student_read_all,
            course_read_all,
            student_insert,
            course_insert,
            sc_insert
        ])
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

/* 添加学生 */
#[tauri::command(async)]
async fn student_insert(student: RawStudent) {
    let db_pool = DB_POOL.read().await;
    let db = db_pool.as_ref().unwrap();

    let active_model: MutStudent = student.into();
    let _ = Student::insert(active_model).exec(db).await;
}

/* 添加课程 */
#[tauri::command(async)]
async fn course_insert(course: RawCourse) {
    let db_pool = DB_POOL.read().await;
    let db = db_pool.as_ref().unwrap();

    let active_model: MutCourse = course.into();

    let _ = Course::insert(active_model).exec(db).await;
}

/* 添加信息 */
#[tauri::command(async)]
async fn sc_insert(sc: RawSC) {
    let db_pool = DB_POOL.read().await;
    let db = db_pool.as_ref().unwrap();

    let active_model: MutSC = sc.into();

    let _ = SC::insert(active_model).exec(db).await;
}
