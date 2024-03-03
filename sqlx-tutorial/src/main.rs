use futures::StreamExt;
use sqlx::{Acquire, Row, SqlitePool};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_dir_path: PathBuf = "../resources".into();
    if !db_dir_path.exists() {
        println!("create_dir_all db_dir_path");
        tokio::fs::create_dir_all(db_dir_path.clone()).await?;
    }
    let db_path: PathBuf = db_dir_path.join("sqlite.db");
    if !db_path.exists() {
        println!("create db");
        tokio::fs::File::create(db_path.clone()).await?;
    }

    let pool = SqlitePool::connect("sqlite://resources/sqlite.db").await?;
    let mut connect = pool.acquire().await?;
    let mut connect_2 = pool.acquire().await?;

    // Executor
    sqlx::query!(
        r#"
    CREATE TABLE IF NOT EXISTS todos
    (
        id          INTEGER PRIMARY KEY NOT NULL,
        description TEXT                NOT NULL,
        done        BOOLEAN             NOT NULL DEFAULT 0
    );
    "#
    )
    .execute(connect.as_mut())
    .await?;

    {
        let mut trans = connect.begin().await?;

        let id = sqlx::query!(
            r#"
            insert into todos(description, done) values("abc", 0)
        "#
        )
        .execute(trans.as_mut())
        .await?
        .last_insert_rowid();
        println!("id={}", id);
        // column name "COUNT(id)" is invalid: "COUNT(id)" is not a valid Rust identifier
        let count = sqlx::query!(
            r#"
            select COUNT(id) as count from todos
        "#
        )
        .fetch_one(trans.as_mut())
        .await?;
        println!("rows={:?}", count.count);

        let count = sqlx::query!(
            r#"
            select COUNT(id) as count  from todos
        "#
        )
        .fetch_one(connect_2.as_mut())
        .await?;
        println!("rows={:?}", count.count);
        trans.commit().await?;
    }
    let rs = sqlx::query!(
        r#"
            select * from todos
        "#
    )
    .fetch_all(connect.as_mut())
    .await?;
    println!("rows={}", rs.len());

    let mut records = sqlx::query("select * from todos where done=?")
        .bind(0)
        .fetch_many(connect.as_mut());
    while let Some(Ok(row)) = records.next().await {
        // map the row into a user-defined domain type
        if let Some(rs) = row.right() {
            let description: &str = rs.try_get("description")?;
            println!("{}", description);
        }
    }
    println!("end");
    Ok(())
}
