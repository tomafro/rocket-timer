use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::schema::stopwatches;
use rocket::response::status;
use diesel::prelude::*;
use crate::Database;

#[get("/stopwatches")]
fn index(db: Database) -> Template {
    let mut context: HashMap<&str, i64> = HashMap::new();
    let count = stopwatches::dsl::stopwatches.select(diesel::dsl::count_star()).first::<i64>(&db.0);
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
    diesel::insert_into(stopwatches::dsl::stopwatches).values(&stopwatch).execute(&db.0).ok();
    let context: HashMap<String, String> = HashMap::new();
    Template::render("update", &context)
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![index, update]
}
