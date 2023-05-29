use scdb::database::pool::init_table;
use scdb::database::prelude::InitDB;
use scdb::entity::prelude::{Course, Student, SC};

#[warn(unused_must_use)]
#[tokio::test]
async fn init_test() {
    let _ = InitDB().await;
}
