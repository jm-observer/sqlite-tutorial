use custom_utils::logger::debug;
use sea_orm::{ConnectionTrait, DbErr, FromQueryResult, Statement};
use crate::datas::{init_data, restore_data};



pub async fn custom_query(db: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
    let id = init_data(db).await?;

    let chef = CustomChef::find_by_statement(Statement::from_string(db.get_database_backend(),       r#"
        select chef.id, chef.bakery_id, chef.name, chef.contact_details, bakery.name as bakery_name from chef left join bakery on chef.bakery_id = bakery.id
        "#.to_string()
    )).one(db).await?.unwrap();
    debug!("{:?}", chef);
    restore_data(db, id).await?;
    Ok(())
}



#[derive(Clone, Debug, PartialEq, FromQueryResult, Eq)]
pub struct CustomChef {
    pub id: i32,
    pub bakery_id: i32,
    pub bakery_name: String,
    pub name: String,
    pub contact_details: Option<String>,
}
