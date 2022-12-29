#[macro_use] extern crate rocket;

// Create
#[post("/create")]
fn create_record() -> &'static str {
    "Hit create_record"
}
// Read
#[get("/read")]
fn read_records() -> &'static str {
    "Hit read_records"
}
// Update
#[post("push_record")]
fn push_record() -> &'static str {
    "Hit push_record"
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
    rocket::build().mount("/", routes![index])
}