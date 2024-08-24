//use std::iter::Map;
//use rocket::Response::NamedFile;
use rocket::fs::NamedFile;
use rocket::get;
use std::path::{Path, PathBuf};

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, health_check, get_file])
}

//let data = Map