use redis::{Commands,ToRedisArgs,RedisResult,FromRedisValue,Client,Connection};

fn get_connection() -> Connection {
    let client = Client::open(dotenv!("REDIS_URL"));
    client.unwrap().get_connection().unwrap()
}

pub fn keys<K: ToRedisArgs, RV: FromRedisValue>(key: K) -> RedisResult<RV> {
    get_connection().keys(key)
}

pub fn get<K: ToRedisArgs, RV: FromRedisValue>(key: K) -> RedisResult<RV> {
    get_connection().get(key)
}

pub fn set<K: ToRedisArgs, V:ToRedisArgs, RV: FromRedisValue>(key: K, value: V) -> RedisResult<RV> {
    get_connection().set(key, value)
}

pub fn del<K: ToRedisArgs, RV: FromRedisValue>(key: K) -> RedisResult<RV> {
    get_connection().del(key)
}

pub fn exists<K: ToRedisArgs, RV: FromRedisValue>(key: K) -> RedisResult<RV> {
    get_connection().exists(key)
}

pub fn set_multiple<K: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(items: &[(K, V)]) -> RedisResult<RV> {
    get_connection().set_multiple(items)
}

pub fn hmset<K: ToRedisArgs, F: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(key: K, items: &[(F, V)]) -> RedisResult<RV> {
    get_connection().hset_multiple(key, items)
}

pub fn hset<K: ToRedisArgs, F: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(key: K, field: F, value: V) -> RedisResult<RV> {
    get_connection().hset(key, field, value)
}
