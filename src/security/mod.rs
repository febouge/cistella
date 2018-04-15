extern crate rocket_cors;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

pub fn get_cors() -> rocket_cors::Cors {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&[dotenv!("CLIENT_URL")]);
    assert!(failed_origins.is_empty());
    
    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}
