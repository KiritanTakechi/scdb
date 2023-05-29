use once_cell::sync::Lazy;
use sea_orm::{
    sea_query::{SqliteQueryBuilder, TableCreateStatement},
    ConnectionTrait, Database, DatabaseConnection, DbBackend, DbConn, EntityTrait, Schema,
};
use std::{error::Error, fs::OpenOptions, path::Path, sync::Arc};
use tokio::{fs::create_dir_all, sync::RwLock};

use crate::entity::prelude::{Course, Student, SC};

pub static DB_POOL: Lazy<Arc<RwLock<Option<DatabaseConnection>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

pub async fn init_db() -> Result<(), Box<dyn Error>> {
    let db_path = "./db/scdb.sqlite";
    ensure_file_exists(db_path).await?;

    let db_pool = Database::connect(&format!("sqlite:{}", db_path)).await?;
    *DB_POOL.write().await = Some(db_pool);

    Ok(())
}

async fn init_table<T>(entity: T, db: &DbConn) -> Result<(), Box<dyn Error>>
where
    T: EntityTrait,
{
    let schema = Schema::new(DbBackend::Sqlite);
    let stmt: TableCreateStatement = schema.create_table_from_entity(entity);
    stmt.build(SqliteQueryBuilder);

    db.execute(db.get_database_backend().build(&stmt)).await?;
    Ok(())
}

async fn ensure_file_exists(file_path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file_path);
    if !path.exists() {
        if let Some(dir) = path.parent() {
            create_dir_all(dir).await?;
            let file = OpenOptions::new().write(true).create(true).open(file_path);
            match file {
                Ok(_) => {
                    // 如果文件不存在并成功创建，执行一些额外的函数
                    let db_pool = Database::connect(&format!("sqlite:{}", file_path)).await?;
                    init_table(Student, &db_pool).await?;
                    init_table(Course, &db_pool).await?;
                    init_table(SC, &db_pool).await?;
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }
    }
    Ok(())
}
