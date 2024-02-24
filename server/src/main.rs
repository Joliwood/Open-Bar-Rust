// use rocket::http::Status;

#[macro_use]
extern crate rocket;

mod routes;
// TODO - Add them later in the project
// mod models;
// mod db;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![routes::test::index, routes::characters::characters],
    )
    // ? This is the old way of attaching the database
    // .attach(db::Db::fairing())
}
