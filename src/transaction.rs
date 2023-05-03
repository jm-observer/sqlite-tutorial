use custom_utils::logger::debug;
use sea_orm::{ActiveValue, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, TransactionTrait};
use crate::entities::{bakery};
use crate::entities::prelude::{Bakery};



pub async fn run_with_transcation(db: &DatabaseConnection) -> Result<(), DbErr> {
    Bakery::delete_by_id(1).exec(db).await?;
    Bakery::delete_by_id(2).exec(db).await?;
    Bakery::delete_by_id(3).exec(db).await?;
    no_commit(db).await?;
    assert!(Bakery::find_by_id(1).one(db).await?.is_none());
    commit(db).await?;
    assert!(Bakery::find_by_id(1).one(db).await?.is_some());
    let bakery_id = 1i32;
    db.transaction::<_, _, DbErr>(move |txn| {
        Box::pin(async move {
            call(txn, bakery_id).await?;
            Ok(())
        })
    }).await.unwrap();
    assert_eq!(Bakery::find_by_id(2).one(db).await?.unwrap().name, "====");

    let res = db.transaction::<_, _, DbErr>(move |txn| {
        Box::pin(async move {
            call_error(txn, bakery_id).await?;
            Ok(())
        })
    }).await;
    debug!("{:?}", res);
    assert!(Bakery::find_by_id(1).one(db).await?.is_some());
    Ok(())
}

///
async fn call(transcation: &DatabaseTransaction, _bakery_id: i32) -> Result<(), DbErr> {
    let happy_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(2),
        name: ActiveValue::Set("====".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
    };
    let _res = Bakery::insert(happy_bakery).exec(transcation).await?;
    Ok(())
}
async fn call_error(transcation: &DatabaseTransaction, bakery_id: i32) -> Result<(), DbErr> {
    Bakery::delete_by_id(bakery_id).exec(transcation).await?;
    let happy_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(3),
        name: ActiveValue::Set("====".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
    };
    let _res = Bakery::insert(happy_bakery).exec(transcation).await?;
    let happy_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(3),
        name: ActiveValue::Set("====".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
    };
    let _res = Bakery::insert(happy_bakery).exec(transcation).await?;
    Ok(())
}

async fn no_commit(db: &DatabaseConnection) -> Result<(), DbErr> {
    let transcation = db.begin().await?;
    let happy_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(1),
        name: ActiveValue::Set("Happy bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
    };
    let _res = Bakery::insert(happy_bakery).exec(&transcation).await?;
    Ok(())
}
async fn commit(db: &DatabaseConnection) -> Result<(), DbErr> {
    let transcation = db.begin().await?;
    let happy_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(1),
        name: ActiveValue::Set("Happy bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
    };
    let _res = Bakery::insert(happy_bakery).exec(&transcation).await?;
    transcation.commit().await?;
    Ok(())
}