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

mod stopwatches;
use rocket_contrib::templates::Template;
use fairings::{ServerTiming,RequestIdHeader};
use dotenv;

#[database("timer")]
struct Database(diesel::pg::PgConnection);

fn prepare_environment() {
    dotenv::dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();
}

fn launch_rocket() {
    rocket::ignite()
        .attach(ServerTiming)
        .attach(RequestIdHeader)
        .attach(Database::fairing())
        .attach(Template::fairing())
        .mount("/", stopwatches::routes()).launch();
}

fn main() {
    prepare_environment();
    launch_rocket();
}
