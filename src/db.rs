use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr};
use sea_orm::RuntimeErr::SqlxError;

const DATABASE_URL: &str = "sqlite://./resources/sqlite.db";



pub async fn connect() -> DatabaseConnection {
    let db = _connect().await;

    db.execute_unprepared(r#"
CREATE TABLE if not exists Bakery  (
	id integer primary key autoincrement,
	name text not null,
	profit_margin REAL not null

);"#).await.unwrap();
    db.execute_unprepared(r#"
CREATE TABLE if not exists Chef (
        id integer primary key autoincrement,
        bakery_id integer not null,
        name text not null,
        contact_details text,
        CONSTRAINT bakery_id_FK FOREIGN KEY (bakery_id) REFERENCES Bakery(id) ON DELETE CASCADE ON UPDATE CASCADE
    );"#).await.unwrap();
    db
}


async fn _connect() -> DatabaseConnection {
    let db = Database::connect(DATABASE_URL).await;
    if let Err(DbErr::Conn(SqlxError(sqlx::Error::Database(err)))) = &db {
        if let Some(code) = err.code() {
            if code.parse() == Ok(14) {
                std::fs::File::create("./resources/sqlite.db").unwrap();
                return Database::connect(DATABASE_URL).await.unwrap();
            }
        }
    }
    db.unwrap()
}