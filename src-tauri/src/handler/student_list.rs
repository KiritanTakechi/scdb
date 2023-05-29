use sea_orm::EntityTrait;

use crate::entity::prelude::*;

use crate::database::prelude::DB_POOL;


#[tauri::command(async)]
pub async fn student_read_all() -> Vec<RawStudent> {
    let db_pool = DB_POOL.read().await;

    let db = db_pool.as_ref().unwrap();

    Student::find().all(db).await.unwrap()
}
