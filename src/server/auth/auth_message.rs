use crate::common::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::convert::From;
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct OAuthCallbackParams {
    pub code: String,
    #[allow(unused)]
    pub state: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct OAuthParams {
    #[validate(length(min = 1, message = "Code cannot be empty"))]
    pub code: Option<String>,
    #[validate(length(min = 1, message = "State cannot be empty"))]
    pub state: Option<String>,
    #[validate(url)]
    pub redirect_uri: Option<String>,
    #[allow(unused)]
    pub invited_by: Option<String>,
}

impl OAuthParams {
    pub fn validate_items(&self) -> AppResult<()> {
        if self.code.is_none() {
            return Err(AppError::InputValidateError("code not found".to_string()));
        }

        if self.state.is_none() {
            return Err(AppError::InputValidateError("state not found".to_string()));
        }

        if self.redirect_uri.is_none() {
            return Err(AppError::InputValidateError(
                "redirect_uri not found".to_string(),
            ));
        }

        Ok(self.validate()?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OauthUserInfo {
    pub data: UserInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub username: String,
    pub profile_image_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeTokenRespose {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}

