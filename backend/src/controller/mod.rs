use database;
use model::ShoppingList;
use redis::RedisError;
use std::collections::HashMap;

pub fn get_all_shopping_lists_dates() -> Option<Vec<String>> {
    get_shopping_lists_dates(String::from("*"))
}

pub fn get_shopping_lists_dates(key: String) -> Option<Vec<String>> {
    let result: Result<Vec<String>, RedisError> = database::keys(&key);
    if result.is_err() {
        error!("Redis error: {}", result.err().unwrap());
        return None;
    }
    info!("Retrieved all the keys that match '{}' pattern", key);
    Some(result.unwrap())
}

pub fn get_shopping_list(key: String) -> Option<ShoppingList> {
    let result: Result<HashMap<String, String>, RedisError> = database::hgetall(&key);
    if result.is_err() {
        error!("Redis error: {}", result.err().unwrap());
        return None;
    }
    info!("Retrieved '{}' key, content: {:?}", &key, result);
    Some(ShoppingList {
        items: result.unwrap(),
        date: key
    })    
}

pub fn save_shopping_list(shopping_list: ShoppingList) -> Option<ShoppingList>{
    let items: Vec<(String, String)> = shopping_list.items.clone().into_iter().collect();
    let result: Result<(), RedisError> = database::hmset(&shopping_list.date, &items);
    if result.is_err() {
        error!("Redis error: {}", result.err().unwrap());
        return None;
    }
    Some(shopping_list)
}

pub fn delete_shopping_list(key: String) -> Option<ShoppingList> {
    let result = database::del(&key);
    if result.is_err() {
        error!("Redis error: {}", result.err().unwrap());
        return None;
    }
    info!("Deleted shopping list with date {}", key);
    Some(ShoppingList {
        items: result.unwrap(),
        date: key
    }) 
}
