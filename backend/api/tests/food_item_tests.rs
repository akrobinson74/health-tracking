mod common;

use api::{data::food_item_input::FoodItemInput, service::Repo};
use entity::food_item;

#[tokio::test]
async fn add_food_item() {
    let db = common::TEST_DB.force().await.get_connection();
    {
        let model = FoodItemInput {
            date: Some("2025-11-15".to_owned()),
            time: Some("16:31".to_owned()),
            name: "Cherry Coke Zero".to_owned(),
            weight: 355.0,
            calories: 0.0,
            notes: None,
            url: None,
        };
        let converted_date = model.date_as_native_date();
        let converted_time = model.time_as_native_time();

        let food_item = Repo::add_food_item(&db, model).await.unwrap();

        assert_eq!(
            food_item,
            food_item::ActiveModel {
                id: sea_orm::ActiveValue::Unchanged(1),
                date: sea_orm::ActiveValue::Unchanged(converted_date),
                time: sea_orm::ActiveValue::Unchanged(converted_time),
                name: sea_orm::ActiveValue::Unchanged("Cherry Coke Zero".to_owned()),
                weight: sea_orm::ActiveValue::Unchanged(355.0),
                calories: sea_orm::ActiveValue::Unchanged(0.0),
                notes: sea_orm::ActiveValue::Unchanged(None),
                url: sea_orm::ActiveValue::Unchanged(None),
            }
        );
    }

    {
        let db = common::TEST_DB.force().await.get_connection();
        let food_item = Repo::get_food_item_by_id(&db, 1).await.unwrap().unwrap();
        assert_eq!(food_item.id, 1);
        assert_eq!(food_item.name, "Cherry Coke Zero");
    }
}
