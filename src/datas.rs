use sea_orm::{ActiveValue, DbErr, EntityTrait};
use crate::entities::{bakery, chef};
use crate::entities::prelude::{Bakery, Chef};



pub async fn init_data(db: &sea_orm::DatabaseConnection) -> Result<i32, DbErr> {
    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;

    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    Ok(bakery_res.last_insert_id)
}

/// return rows_affected
pub async fn restore_data(db: &sea_orm::DatabaseConnection, bakery_id: i32) -> Result<u64, DbErr> {
    let res = Bakery::delete_by_id(bakery_id).exec(db).await?;
    Ok(res.rows_affected)
}