#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate openssl;


extern crate openssl_probe;

mod schema;
mod models;
mod fairings;

use self::diesel::prelude::*;
use rocket::http::*;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use fairings::{ServerTiming,RequestIdHeader};
use schema::stopwatches;
use rocket::response::status;

enum CreatedOrUpdated {
    Created(Template),
    Updated(Template)
}

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
    name: Option<&'a str>,
}

#[put("/stopwatches/<identifier>?<name>")]
fn update(db: Database, identifier: String, name: Option<String>) -> Template {
    let stopwatch = NewStopwatch { identifier: &identifier.to_string(), name: name.as_deref()};
    diesel::insert_into(schema::stopwatches::dsl::stopwatches).values(&stopwatch).execute(&db.0).ok();
    let context: HashMap<String, String> = HashMap::new();
    Template::render("update", &context)
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    rocket::ignite()
        .attach(ServerTiming)
        .attach(RequestIdHeader)
        .attach(Database::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index, update]).launch();
}
