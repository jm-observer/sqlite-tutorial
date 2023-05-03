use custom_utils::logger::debug;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, ModelTrait, Order, PaginatorTrait, QueryFilter, QueryOrder};
use crate::datas::{init_data, restore_data};
use crate::entities::{bakery, chef};
use crate::entities::prelude::{Bakery, Chef};

pub async fn run_with_db(db: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
    let bakery_id = init_data(db).await?;
    let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_id)
        .one(db)
        .await?
        .unwrap();

    let chefs: Vec<chef::Model> = la_boulangerie.find_related(Chef).order_by(chef::Column::Id, Order::Desc).paginate(db, 2).fetch_page(1).await?;
    // let mut chef_names: Vec<String> = chefs.into_iter().map(|b| b.name).collect();
    // assert_eq!(chef_names, ["Frederic", "Madeleine"]);

    let chefs_id = chefs.get(0).unwrap().id;

    let res = Chef::find().left_join(Bakery).select_also(Bakery).filter(chef::Column::Id.eq(chefs_id)).one(db).await?.unwrap();
    debug!("{:?} {:?}", res.0, res.1);


    restore_data(db, bakery_id).await?;
    Ok(())
}


