use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::schema::stopwatches;
use diesel::prelude::*;
use crate::Database;

#[get("/")]
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

#[put("/<identifier>?<name>")]
fn update(db: Database, identifier: String, name: Option<String>) -> Template {
    let stopwatch = NewStopwatch { identifier: &identifier.to_string(), name: name.as_deref()};
    diesel::insert_into(stopwatches::dsl::stopwatches).values(&stopwatch).execute(&db.0).ok();
    let context: HashMap<String, String> = HashMap::new();
    Template::render("update", &context)
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![index, update]
}

#[cfg(test)]
mod test {
    use crate::prepare;
    use super::routes;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn simple_test() {
        let client = Client::new(prepare().mount("/", routes())).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
