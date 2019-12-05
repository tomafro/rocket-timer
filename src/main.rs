#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod fairings;

use self::diesel::prelude::*;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use fairings::{ServerTiming,RequestIdHeader};
use schema::stopwatches;

#[database("timer")]
struct Database(diesel::pg::PgConnection);

#[get("/stopwatches")]
fn index(db: Database) -> Template {
    let mut context: HashMap<&str, i64> = HashMap::new();
    let count = schema::stopwatches::dsl::stopwatches.select(diesel::dsl::count_star()).first::<i64>(&db.0);
    context.insert("count", count.unwrap());
    Template::render("index", &context)
}

#[derive(Insertable)]
#[table_name = "stopwatches"]
struct NewStopwatch<'a> {
    identifier: &'a str,
    name: &'a str,
}


#[put("/stopwatches/<identifier>")]
fn update(db: Database, identifier: String) -> Template {
    let stopwatch = NewStopwatch { identifier: &identifier.to_string(), name: "a"};
    diesel::insert_into(schema::stopwatches::dsl::stopwatches).values(&stopwatch).execute(&db.0).ok();
    let context: HashMap<String, String> = HashMap::new();
    Template::render("update", &context)
}

fn main() {
    rocket::ignite()
        .attach(ServerTiming)
        .attach(RequestIdHeader)
        .attach(Database::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index, update]).launch();
}
