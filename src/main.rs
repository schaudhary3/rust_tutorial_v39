#[macro_use] extern crate rocket;

//use rocket::fs::NamedFile;
use rocket::get;
//use sqlx::postgres::PgDatabaseError;
//use std::path::{Path, PathBuf};
//use rocket::fairing::AdHoc;
//use rocket::{Build, Rocket};
//use rocket::Rocket;
//use sqlx::PgPool;
//use sqlx::{PgConnection, Error};
use sqlx::Error;
use serde::{Serialize, Deserialize};
//use rocket_sync_db_pools::database;
use rocket_db_pools::{Connection, Database};
use rocket_db_pools::deadpool_postgres;
use dotenv::dotenv;
use sqlx::FromRow;

#[derive(Database)]
//struct Db(PgPool);
#[database("sqlx")]
pub struct Db(deadpool_postgres::Pool);
//struct RowCount(i64);

#[derive(FromRow, Serialize, Deserialize)]
pub struct Item {
    #[sqlx(default)]
    uuid: Option<i32>,
    #[sqlx(default)]
    name: Option<String>,
    #[sqlx(default)]
    number: Option<i32>
}

#[get("/one")]
async fn get_recd(db: Connection<Db>) -> Result<Item, Error> {
    let recd = sqlx::query_as!(Item, "SELECT * FROM (SELECT (1) as uuid, 'J' as name, 9 as number) AS item where uuid = $1", 1)
    .fetch_one(& **db)
    .await;
    
    recd
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Db::init())
        .mount("/", routes![get_recd])
}

//.map_err(|e: PgDatabaseError| e.to_string())?;