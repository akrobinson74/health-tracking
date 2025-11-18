use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table("food_item")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(date("date").not_null())
                    .col(time("time").not_null())
                    .col(double("weight").not_null())
                    .col(double("calories").not_null())
                    .col(string("notes"))
                    .col(string("url"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("weigh_in")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(date("date").not_null())
                    .col(time("time").not_null())
                    .col(double("weight").not_null())
                    .col(string("notes"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("food_item").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("weigh_in").to_owned())
            .await
    }
}

