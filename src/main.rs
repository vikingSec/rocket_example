use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rocket::response::status;
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Item<'r> {
    itemvalue: &'r str,
    username: str
}

#[macro_use] extern crate rocket;

// Create
#[post("/create")]
fn create_record() -> &'static str {
    "Hit create_record"
}
// Read
#[get("/read")]
fn read() -> status::Accepted<String>  {
    let mut file = match File::open("./data.txt") {
        Err(e) => {
            println!("Error on file open: {}",e);
            return status::Accepted(Some(String::from("Error: Error on file open!")));
        }
        Ok(file) => file,
    };
    let mut fstring = String::new();
    match file.read_to_string(&mut fstring) {
        Err(e) => {
            println!("Error on string read: {}",e);
            return status::Accepted(Some(String::from("Error: Error on file open!")));
        }
        Ok(_) => println!("Successfully read to string: {}",fstring)
    };

    status::Accepted(Some(format!("values: '{}'",fstring)))
}
// Update
#[post("/push_record")]
fn push_record(item: Json<Item<'_>>) -> status::Accepted<String> {
    let mut file = match File::open("./data.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("There was an error opening the file: {}",e);
            return status::Accepted(Some(String::from("Error: Error on file open!")));
        }
    };
    // Access the value that was sent
    status::Accepted(Some(String::from("Hit push_record")))
}
// Delete
#[post("/delete_record")]
fn del_record() -> &'static str {
    "Hit delete_record"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,create_record, read, del_record])
}