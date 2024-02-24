// TODO - Add a status to the response
// use rocket::http::Status;

#[get("/test")]
pub fn index() -> String {
    String::from("Hello, world!")
}
