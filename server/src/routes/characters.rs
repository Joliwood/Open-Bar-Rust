use rocket::http::Status;

#[get("/characters/<_server>/<_character>?<_msg>")]
pub fn characters(_server: &str, _character: &str, _msg: Option<&str>) -> Status {
    // info!("Server: {}", _server);
    Status::NoContent
}
