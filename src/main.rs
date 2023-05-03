mod entities;
pub mod db;
mod insert_update_delete;
mod query;
mod datas;
mod transaction;
mod custom_query;


use sea_orm::{ConnectionTrait};
use crate::custom_query::custom_query;
use crate::db::connect;

use crate::insert_update_delete::run;
use crate::query::run_with_db;
use crate::transaction::run_with_transcation;

#[tokio::main]
async fn main() {
    custom_utils::logger::logger_stdout_debug();
    let db = connect().await;
    let _database = db.get_database_backend();
    run(&db).await.unwrap();
    if let Err(err) = run_with_db(&db).await {
        panic!("{}", err);
    }

    run_with_transcation(&db).await.unwrap();
     custom_query(&db).await.unwrap();
}
