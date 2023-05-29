use std::ops::Deref;

use scdb::database::prelude::{InitDB, DB_POOL};
use sea_orm::Database;
use sea_orm::{entity::*, error::*, query::*, DbConn, FromQueryResult};

// 测试函数，使用 #[test] 属性标记
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

// 另一个测试函数
#[test]
fn test_subtraction() {
    assert_eq!(5 - 3, 2);
}

// 运行测试
fn main() {
    // 使用 cargo test 命令运行所有测试
    // 或者运行 cargo test <function_name> 来运行指定的测试函数
}

#[tokio::test]
async fn find_test() {
    use scdb::entity::prelude::*;

    /* let db = Database::connect("sqlite:/Users/kiritan/Code/scdb/db/scdb")
    .await
    .unwrap(); */

    if let Err(e) = InitDB().await {
        panic!("{}", e)
    }

    let db_pool_guard = DB_POOL.read().await;

    let db = match db_pool_guard.deref() {
        Some(n) => n,
        None => panic!(),
    };

    let data: Option<RawStudent> = Student::find_by_id("1").one(db).await.unwrap();
    match data {
        Some(n) => println!("{}", n.f_name),
        _ => println!("not one!"),
    }
}
