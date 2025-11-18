use async_lazy::Lazy;
use sea_orm::{Database, DatabaseConnection};
use entity::food_item;

pub struct TestDB {
    pub connection: DatabaseConnection,
}

impl TestDB {
    pub async fn new() -> Self {
        let connection = Database::connect("sqlite::memory:").await.unwrap();

        connection.get_schema_builder()
            .register(food_item::Entity)
            .apply(&connection)
            .await
            .expect("unable to create test db");

        TestDB { connection }
    }

    pub fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}

pub async fn setup() -> TestDB {
    TestDB::new().await
}

pub static TEST_DB: Lazy<TestDB> = Lazy::new(|| Box::pin(async { setup().await }));
