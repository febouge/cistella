#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate redis;

#[macro_use]
extern crate dotenv_codegen;

mod database;

#[get("/")]
fn index() -> &'static str {
    // if database::fetch_an_integer().is_err() {
    //     return "Error from redis";
    // }
    "Success from Redis"
}

#[get("/set/<key>/<value>")]
fn set(key: String, value: String) -> String {
    let redis_result = database::set(key, value);
    if redis_result.is_err() {
        return "Error setting value".to_owned();
    }
    redis_result.unwrap()
}

#[get("/get/<key>")]
fn get(key: String) -> String {
    let redis_result = database::get(key);
    if redis_result.is_err() {
        return "Error getting value".to_owned();
    }
    redis_result.unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![index, set, get]).launch();
}

