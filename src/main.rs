#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate redis;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate dotenv_codegen;
#[macro_use] extern crate log;

mod model;
mod controller;
mod database;

use model::ShoppingList;
use rocket_contrib::Json;

#[get("/")]
fn index() -> &'static str {
    "Cistella"
}

#[get("/shoppinglist/<key>")]
fn get(key: String) -> Option<Json<ShoppingList>> {
    let shopping_list = controller::get_shopping_list(key);
    parse_to_json(shopping_list)
}

#[post("/shoppinglist", format = "application/json; charset=utf-8", data = "<shoppinglist>")]
fn save(shoppinglist: Json<ShoppingList>) -> Option<Json<ShoppingList>> {
    let shopping_list = controller::save_shopping_list(shoppinglist.0);
    parse_to_json(shopping_list)
}

#[delete("/shoppinglist/<key>")]
fn delete(key: String) -> Option<Json<ShoppingList>> {
    let shopping_list = controller::delete_shopping_list(key);
    parse_to_json(shopping_list)
}

#[get("/shoppinglists")]
fn find_all() -> Option<Json<Vec<String>>> {
    let shopping_lists_dates = controller::get_all_shopping_lists_dates();
    if shopping_lists_dates.is_none() {
        return None;
    }
    Some(Json(shopping_lists_dates.unwrap()))
}

#[get("/shoppinglists/<key_pattern>")]
fn find_by_pattern(key_pattern: String) -> Option<Json<Vec<String>>> {
    let shopping_lists_dates = controller::get_shopping_lists_dates(key_pattern);
    if shopping_lists_dates.is_none() {
        return None;
    }
    Some(Json(shopping_lists_dates.unwrap()))
}

fn parse_to_json(shopping_list: Option<ShoppingList>) -> Option<Json<ShoppingList>> {
    if shopping_list.is_none() {
        return None;
    }
    Some(Json(shopping_list.unwrap()))
}

fn main() {
    rocket::ignite().mount("/", routes![index, save, get, delete, find_all, find_by_pattern]).launch();
}

