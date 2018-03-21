use std::collections::HashMap;

#[derive(Debug)]
pub struct ShoppingList {
    pub items: HashMap<String, String>,
    pub date: String
}

