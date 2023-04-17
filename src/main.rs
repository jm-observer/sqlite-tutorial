use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let db = Database::connect("sqlite://./resources/sqlite3.db")
        .await
        .unwrap();

    db.execute_unprepared(r#"CREATE TABLE `cake` (
                        id INTEGER PRIMARY KEY AUTOINCREMENT)"#).await.unwrap();

    println!("Hello, world!");
}

//
// fn is_exist_table(db: &DatabaseConnection, name: &str) -> anyhow::Result<bool> {
//     let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name=?1";
//
//     let a =  db.get_database_backend().build(sql);
//
//     let res = stmt
//         .exists([name])
//         .map_err(|e| Error::CheckExists(name.to_owned(), e))?;
//
//     Ok(res)
// }