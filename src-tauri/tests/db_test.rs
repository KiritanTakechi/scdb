use scdb::database::prelude::{InitDB, DB_POOL};
use scdb::entity::prelude::{Course, RawStudent, Student, SC};
use sea_orm::EntityTrait;

#[warn(unused_must_use)]
#[tokio::test]
async fn init_test() {
    let _ = InitDB().await;
}
#[warn(unused_must_use)]
#[tokio::test]
async fn student_read_all() {
    let _ = InitDB().await;

    let db_pool = DB_POOL.read().await;

    let db = db_pool.as_ref().unwrap();

    let res = Student::find().all(db).await.unwrap();

    println!("{}", res[0].f_name);
}
