use diesel::{Selectable, Insertable, Queryable, QueryableByName, {pg::Pg}};
use super::schema::items;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[derive(QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct Item {
    pub uuid: i32,
    pub name: Option<String>,
    pub number: Option<i32>,
}

// This is for deserializing incoming JSON payloads
#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub number: &'a i32,
}
