use redis::{Commands,ToRedisArgs,RedisResult,FromRedisValue,Client,Connection};

fn get_connection() -> Connection {
    let client = Client::open(dotenv!("REDIS_URL"));
    client.unwrap().get_connection().unwrap()
}

pub fn keys<K: ToRedisArgs, RV: FromRedisValue>(key: K) -> RedisResult<RV> {
    get_connection().keys(key)
}

pub fn del<K: ToRedisArgs, RV: FromRedisValue>(key: K) -> RedisResult<RV> {
    get_connection().del(key)
}

pub fn hmset<K: ToRedisArgs, F: ToRedisArgs, V: ToRedisArgs, RV: FromRedisValue>(key: K, items: &[(F, V)]) -> RedisResult<RV> {
    get_connection().hset_multiple(key, items)
}

pub fn hgetall<K: ToRedisArgs, RV: FromRedisValue>(key: K) -> RedisResult<RV> {
    get_connection().hgetall(key)
}
