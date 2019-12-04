#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;

use self::diesel::prelude::*;

#[database("timer")]
struct Database(diesel::pg::PgConnection);

#[get("/")]
fn index(db: Database) -> String {
    use schema::stopwatches::dsl::*;
    format!("{:?}", stopwatches.select(diesel::dsl::count_star()).first::<i64>(&db.0))
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/", routes![index]).launch();
}
