use database;
use model::ShoppingList;
use redis::RedisError;

pub fn get_shopping_list(key: String) -> Option<ShoppingList> {
    let result = database::get(&key);
    if result.is_err() {
        return None;
    }
    Some(ShoppingList {
        items: result.unwrap(),
        date: key
    })    
}

pub fn save_shopping_list(shopping_list: ShoppingList) -> Result<Vec<(String,String)>, RedisError>{
    let items: Vec<(String, String)> = shopping_list.items.clone().into_iter().collect();
    database::hmset(shopping_list.date, items.as_slice())
}

pub fn delete_shopping_list(key: String) -> Option<ShoppingList> {
    let result = database::del(&key);
    if result.is_err() {
        return None;
    }
    Some(ShoppingList {
        items: result.unwrap(),
        date: key
    }) 
}
