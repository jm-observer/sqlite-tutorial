use custom_utils::logger::debug;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use crate::entities::{bakery, chef};
use crate::entities::prelude::{Bakery, Chef};
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

pub async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {

    let happy_bakery = bakery::ActiveModel {
        id: Default::default(),
        name: ActiveValue::Set("Happy bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(db).await?;

    let john = chef::ActiveModel {
        id: Default::default(),
        bakery_id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("John".to_owned()),
        contact_details: Default::default(),
    };
    Chef::insert(john).exec(db).await?;

    let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
    for bakery in bakeries {
        debug!("{:?}", bakery);
    }

    let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(res.last_insert_id).one(db).await?;
    assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

    let _sad_bakery: Option<bakery::Model> = Bakery::find().filter(bakery::Column::Name.eq("Sad Bakery"))
        .one(db).await?;

    let john = chef::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        ..Default::default()
    };
    john.delete(db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        ..Default::default()
    };
    sad_bakery.delete(db).await?;

    let _bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
    // assert!(bakeries.is_empty());
    Ok(())
}