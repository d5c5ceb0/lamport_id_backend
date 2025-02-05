use oauth2::CsrfToken;
use redis::AsyncCommands;
use serde::Serialize;
use crate::common::error::{AppResult, AppError};

pub fn gen_csrf_token() -> String {
    CsrfToken::new_random().secret().to_string()
}

#[derive(Clone)]
pub struct RedisClient(redis::Client);

impl From<redis::Client> for RedisClient {
    fn from(item: redis::Client) -> Self {
        Self(item)
    }
}

impl RedisClient {
    #![allow(dead_code)]
    pub async fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;

        Ok(Self(client))
    }

    pub async fn cache_csrf_token(&self) -> Result<String, redis::RedisError> {
        let token: String = gen_csrf_token();

        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let _: () = conn.set_ex(&token, "lamport_id", 600).await?;

        Ok(token)
    }
    pub async fn get_csrf_token(&self, token: &str) -> Result<String, redis::RedisError> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let cached_token: Option<String> = conn.get(token).await?;

        match cached_token {
            Some(val) => Ok(val),
            None => Err(redis::RedisError::from((
                redis::ErrorKind::ResponseError,
                "Token not found",
            ))),
        }
    }

    pub async fn del_csrf_token(&self, token: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let _: () = conn.del(token).await?;

        Ok(())
    }

    pub async fn cache_nonce(&self, address: &str) -> Result<String, redis::RedisError> {
        let token: String = gen_csrf_token();

        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let _: () = conn.set_ex(address, token.as_str(), 600).await?;

        Ok(token)
    }

    pub async fn get_nonce(&self, address: &str) -> Result<String, redis::RedisError> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let cached_token: Option<String> = conn.get(address).await?;

        match cached_token {
            Some(val) => Ok(val),
            None => Err(redis::RedisError::from((
                redis::ErrorKind::ResponseError,
                "Token not found",
            ))),
        }
    }

    pub async fn del_nonce(&self, address: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let _: () = conn.del(address).await?;

        Ok(())
    }

    //get data from cache
    pub async fn get_data<T>(&self, key: &str) -> AppResult<T>
    where
        T: for<'de> serde::de::Deserialize<'de>
    {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let cached_data: Option<String> = conn.get(key).await?;

        match cached_data {
            Some(val) => Ok(serde_json::from_str(&val)?),
            None => Err(AppError::CustomError("Data not found".to_string())),
        }

    }

    //set data to cache
    pub async fn set_data(&self, key: &str, value: impl Serialize) -> AppResult<()> {
        let mut conn = self.0.get_multiplexed_async_connection().await?;
        let _: () = conn.set_ex(key, serde_json::to_string(&value)?, 3600).await?;

        Ok(())
    }

}
