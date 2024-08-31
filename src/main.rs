#[macro_use] extern crate rocket;

pub mod model;
pub mod schema;

use self::schema::items::dsl::*;
use crate::model::Item; 

use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use rocket::response::status::{Created, NoContent, NotFound};
use rocket::serde::json::Json;
use diesel::prelude::*;
use rocket_sync_db_pools::{database, diesel};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}
#[database("my_db")] 
struct MyDatabase(diesel::PgConnection);

#[get("/item/<other_uuid>")]
async fn get_result(db: MyDatabase, other_uuid: i32) -> Result<Json<Item>, NotFound<Json<ApiError>>> {
    
    db
        .run(move |c| items.filter(uuid.eq(other_uuid)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[get("/item")]
async fn get_result_(db: MyDatabase) -> Result<Json<Item>, NotFound<Json<ApiError>>> {
    
    db
        .run(move |c| items.filter(uuid.eq(1)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(MyDatabase::fairing())
        .mount("/", routes![get_result, get_result_])
}