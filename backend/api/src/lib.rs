pub mod service;

use actix_web::middleware::Logger;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, post, web};
use entity::food_item::ActiveModel;
use env_logger::Env;
use listenfd::ListenFd;
use sea_orm::{Database, DatabaseConnection, InsertResult};
use serde::Deserialize;
use service::Repo;
use std::env;
use actix_cors::Cors;

const DEFAULT_ITEMS_PER_PAGE: u64 = 25;

#[derive(Debug, Clone)]
struct AppState {
    connection: DatabaseConnection,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    date: Option<String>,
    page: Option<u64>,
    items_per_page: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct FoodItemInput {
    pub date: Option<String>,
    pub time: Option<String>,
    pub name: String,
    pub weight: f64,
    pub calories: f64,
    pub notes: Option<String>,
    pub url: Option<String>,
}

#[post("/foodItem")]
async fn create_food_item(
    data: web::Data<AppState>,
    json: web::Json<FoodItemInput>,
) -> Result<HttpResponse, Error> {
    let conn = &data.connection;
    let form = json.into_inner();

    let insert_result: InsertResult<ActiveModel>;

    if form.date.is_some() && form.time.is_some() {
        insert_result = Repo::add_food_item(conn, form)
            .await
            .expect("could not add food_item");
    } else {
        insert_result = Repo::add_food_item_now(conn, form)
            .await
            .expect("could not add food_item");
    }

    let food_item_id = insert_result.last_insert_id;

    Ok(HttpResponse::Accepted()
        .append_header(("location", format!("/foodItem/{}", food_item_id)))
        .finish())
}

#[post("/foodItems")]
async fn create_food_items(
    data: web::Data<AppState>,
    json: web::Json<Vec<FoodItemInput>>,
) -> Result<HttpResponse, Error> {
    let conn = &data.connection;
    let data = json.into_inner();

    Repo::add_food_items(conn, data)
        .await
        .expect("could not add food_items");
    
    Ok(HttpResponse::Accepted().finish())
}

#[get("/foodItem")]
async fn list_food_items(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.connection;
    let params = web::Query::<Params>::from_query(req.query_string())?;

    let page = params.page.unwrap_or(1);
    let items_per_page = params.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);
    let date = params.date.clone();

    let items = Repo::get_food_items_in_page(conn, date, page, items_per_page)
        .await
        .expect("Cannot find food_item(s) in page");

    Ok(HttpResponse::Ok().json(items))
}

#[get("/foodItem/{id}")]
async fn get_food_item(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let conn = &data.connection;
    let id = path.into_inner();

    let item = Repo::get_food_item_by_id(conn, id)
        .await
        .expect(&format!("Cannot find food_item for id {}", id));

    if let Some(model) = item {
        Ok(HttpResponse::Ok().json(model))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/foodItems")]
async fn get_food_items(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.connection;

    let items = Repo::get_food_items(conn)
        .await
        .expect("Cannot find food_items");

    Ok(HttpResponse::Ok().json(items))
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_food_item);
    cfg.service(create_food_items);
    cfg.service(get_food_item);
    cfg.service(get_food_items);
    cfg.service(list_food_items);
}

async fn not_found(request: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().body(format!("Not found: {}", request.path())))
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let connection = Database::connect(&db_url).await.unwrap();
    let state = AppState { connection };

    let mut listen_fd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Cors::default()
                .allowed_origin("http://localhost:3000") // Replace with your actual frontend origin
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::ACCEPT,
                    actix_web::http::header::CONTENT_TYPE,
                ])
                .max_age(3600))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i")) // enable logger
            .default_service(web::route().to(not_found))
            .configure(init)
    });

    server = match listen_fd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
