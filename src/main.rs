//use std::iter::Map;
//use rocket::Response::NamedFile;
use rocket::fs::NamedFile;
use rocket::get;
use std::path::{Path, PathBuf};
use rocket::fairing::AdHoc;
//use rocket::serde::{Serialize, Deserialize};
use rocket::{Build, Rocket};
use sqlx::postgres::PgPool;
use sqlx::Connection;
use serde::{Serialize, Deserialize};
use rocket_sync_db_pools::databse;
//use std::mem::size_of;
//use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;
use sqlx::FromRow;

#[derive(FromRow)]
struct RowCount(i64);

#[derive(Serialize, Deserialize)]
struct Item {
    uuid: i64,
    name: String,
    number: i64
}

/*#[get("/items")]
async fn get_items(pool: &rocket::State<PgPool>){
    let items = sqlx::query_as!(Item, "SELECT id, name, description FROM items")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(items)
}*/

/*#[get("/count")]
async fn count_records() -> Result<String, sqlx::Error> {
    let row: CountRow = sqlx::query_as!("SELECT COUNT(*) as count FROM my_table")
        .fetch_one(&pool)
        .await?; // Make sure to use .await here because this is async

    Ok(row.count.to_string())
}*/

#[get("/one")]
async fn get_recd() -> Result<Item, sqlx::error> {
    let recd = sqlx::query_as!(
        Item,
        "select * from (select (1) as uuid, 'J' as name, 9 as number) my_table where uuid = ?",
        1i32)
    .fetch_one(&pool)
    .await?;
}

async fn init_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to connect to databse")
}

//use rocket::http::RawStr;
#[macro_use] extern crate rocket;

/*#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}*/

#[get("/hello/<name>")]
fn hello(name: /*&RawStr*/String) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/health_check")]
fn health_check() -> &'static str {
    "OK"
}

#[get("/file/<file_name>")]
async fn get_file(file_name: String) -> Option<NamedFile> {
    //Construct the path to the file
    let path: PathBuf = Path::new("file").join(file_name);

    //Return the file if it exists, otherwsise return None
    NamedFile::open(path).await.ok()
}

/*#[get("/")]
async fn index(db: Db) -> String {
    //Example query
    let row: (i64,) = sqlx.query-as("SELECT COUNT(*) FROM my_table")
        .fetch_one(&*db)
        .await.unwrap();

    format!("Number of records: {}", row.0)
}*/



#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(AdHoc::on_ignite("Database Pool", |rocket| async {
            let pool  = init_pool().await;
            rocket.manage(pool)
        }))
        .mount("/", routes![hello, health_check, get_file, get_items])
}

/*let data = Map*/