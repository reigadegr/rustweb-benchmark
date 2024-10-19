use redis::Commands;

pub async fn redis_write<T: redis::ToRedisArgs>(key: &str, value: T) -> Result<(), anyhow::Error> {
    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // Throw away the result, just make sure it does not fail
    let _: () = con.set(key, value)?;
    Ok(())
}

pub async fn redis_write_and_rm<T: redis::ToRedisArgs>(
    key: &str,
    value: T,
    time: i64,
) -> Result<(), anyhow::Error> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set(key, value)?;
    let _: () = con.expire(key, time)?;
    Ok(())
}

pub async fn redis_read(key: &str) -> Result<String, anyhow::Error> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let rs = con.get(key)?;
    Ok(rs)
}

pub async fn redis_delete(key: &str) -> Result<(), anyhow::Error> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // delete the key
    let key = String::from(key);
    let _: () = con.del(key)?;

    Ok(())
}
