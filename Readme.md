# Cistella

Cistella is a really simple web app (just a REST Json API with a minimal client). Its backend just contains a simple CRUD for shopping lists (key-hashmap) and it is intended to run using Redis as database. On the frontend, a React client is used to get shopping lists

## Dependencies

It uses Redis as a database.

### Backend (Rust)
- [Rocket](https://rocket.rs/)
- [Serde](https://github.com/serde-rs/serde)
- [redis-rs](https://github.com/mitsuhiko/redis-rs)
- [log](https://github.com/rust-lang-nursery/log)
- [dotenv](https://github.com/purpliminal/rust-dotenv) 

### Frontend
- [React](https://reactjs.org/)

## Disclaimer

This is just a proof of concept.
