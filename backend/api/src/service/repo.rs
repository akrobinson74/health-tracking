use crate::FoodItemInput;
use ::entity::{food_item, food_item::ActiveModel, food_item::Entity as FoodItem};
use chrono::{Local, NaiveDate, NaiveTime};
use sea_orm::*;
// use sea_orm::sea_query::Keyword::Default;

pub struct Repo;

impl Repo {
    pub async fn add_food_item(
        db: &DatabaseConnection,
        model: FoodItemInput,
    ) -> Result<InsertResult<ActiveModel>, DbErr> {
        let item = Self::active_model(&model);
        let result = FoodItem::insert(item).exec(db).await?;
        Ok(result)
    }

    pub async fn add_food_items(
        db: &DatabaseConnection,
        data: Vec<FoodItemInput>,
    ) -> Result<InsertManyResult<ActiveModel>, DbErr> {
        let food_items = data.iter().map(Self::active_model).collect::<Vec<_>>();
        FoodItem::insert_many(food_items).exec(db).await
    }

    pub async fn add_food_item_now(
        db: &DatabaseConnection,
        model: FoodItemInput,
    ) -> Result<InsertResult<ActiveModel>, DbErr> {
        let item = ActiveModel {
            date: Set(Local::now().date_naive()),
            time: Set(Local::now().time()),
            name: Set(model.name.to_owned()),
            weight: Set(model.weight.to_owned()),
            calories: Set(model.calories.to_owned()),
            notes: Set(model.notes.to_owned()),
            url: Set(model.url.to_owned()),
            ..Default::default()
        };

        let result = FoodItem::insert(item).exec(db).await?;
        Ok(result)
    }

    pub async fn get_food_item_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<food_item::Model>, DbErr> {
        FoodItem::find_by_id(id).one(db).await
    }

    pub async fn get_food_items_in_page(
        db: &DatabaseConnection,
        date: Option<String>,
        page: u64,
        items_per_page: u64,
    ) -> Result<(Vec<food_item::Model>, u64), DbErr> {
        let paginator = if date.is_none() {
            FoodItem::find()
                .order_by_asc(food_item::Column::Id)
                .paginate(db, items_per_page)
        } else {
            FoodItem::find()
                .filter(food_item::Column::Date.eq(date))
                .order_by_asc(food_item::Column::Id)
                .paginate(db, items_per_page)
        };

        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    fn active_model(item: &FoodItemInput) -> ActiveModel {
        let date_parse_result =
            NaiveDate::parse_from_str(&item.date.clone().unwrap(), "%Y-%m-%d");
        let Ok(_) = date_parse_result else {
            panic!("date parse error");
        };
        let time_parse_result =
            NaiveTime::parse_from_str(&item.time.clone().unwrap(), "%H:%M");
        let Ok(_) = time_parse_result else {
            panic!("time parse error");
        };

        ActiveModel {
            date: Set(date_parse_result.unwrap()),
            time: Set(time_parse_result.unwrap()),
            name: Set(item.name.to_owned()),
            weight: Set(item.weight.to_owned()),
            calories: Set(item.calories.to_owned()),
            notes: Set(item.notes.to_owned()),
            url: Set(item.url.to_owned()),
            ..Default::default()
        }
    }
}
