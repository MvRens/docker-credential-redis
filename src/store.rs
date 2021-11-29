pub mod redis_store
{
  pub struct Credentials
  {
    pub username: String,
    pub secret: String
  }


  pub fn store(server_url: String, username: String, secret: String) -> Result<(), redis::RedisError>
  {
    let client = get_client()?;
    let mut connection = client.get_connection()?;

    redis::cmd("HMSET").arg(get_key(&server_url))
      .arg("username").arg(username)
      .arg("secret").arg(secret)
      .query(&mut connection)?;

    Ok({})
  }


  pub fn get(server_url: &str) -> Result<Option<Credentials>, redis::RedisError>
  {
    let client = get_client()?;
    let mut connection = client.get_connection()?;

    let hm: Vec<String> = redis::cmd("HMGET").arg(get_key(server_url))
      .arg("username")
      .arg("secret")
      .query(&mut connection)?;

    if hm.len() < 2
    {
      return Ok(None)
    }

    Ok(Some(Credentials {
      username: String::from(&hm[0]),
      secret: String::from(&hm[1])
    }))
  }


  pub fn erase(server_url: &str) -> Result<(), redis::RedisError>
  {
    let client = get_client()?;
    let mut connection = client.get_connection()?;

    redis::cmd("DEL").arg(get_key(server_url)).query(&mut connection)?;

    Ok({})
  }


  fn get_client() -> Result<redis::Client, redis::RedisError>
  {
    // TODO support connection parameters
    redis::Client::open("redis://127.0.0.1/15")
  }


  fn get_key(server_url: &str) -> String
  {
    // TODO support key prefixes
    String::from(server_url)
  }
}