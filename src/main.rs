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

#[get("/stopwatches")]
fn index(db: Database) -> Template {
    use schema::stopwatches::dsl::*;
    let mut context: HashMap<&str, i64> = HashMap::new();
    let count = stopwatches.select(diesel::dsl::count_star()).first::<i64>(&db.0);
    context.insert("count", count.unwrap());
    Template::render("index", &context)
}

#[put("/stopwatches/<name>")]
fn update(db: Database, name: String) -> Template {
    use schema::stopwatches::dsl::*;

    diesel::insert_into(stopwatches).values(&title.eq(name)).execute(&db.0).ok();
    let context: HashMap<String, String> = HashMap::new();
    Template::render("update", &context)
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index, update]).launch();
}
