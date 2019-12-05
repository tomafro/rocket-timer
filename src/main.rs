#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;

use self::diesel::prelude::*;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[database("timer")]
struct Database(diesel::pg::PgConnection);

#[get("/")]
fn index(db: Database) -> String {
    use schema::stopwatches::dsl::*;
    format!("{:?}", stopwatches.select(diesel::dsl::count_star()).first::<i64>(&db.0))
}

#[put("/stopwatches/<name>")]
fn update(db: Database, name: String) -> Template {
    use schema::stopwatches::dsl::*;

    diesel::insert_into(stopwatches).values(&title.eq(name)).execute(&db.0).ok();
    "Hello World";
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index, update]).launch();
}
