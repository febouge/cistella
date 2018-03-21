use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingList {
    pub items: HashMap<String, String>,
    pub date: String
}

