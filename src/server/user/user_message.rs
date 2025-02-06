use crate::database::services::binding;
use crate::database::dals::telegram_binding;
use crate::helpers::utils::*;
use crate::database::entities::users;
use crate::server::auth::OauthUserInfo;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::convert::Into;

#[derive(Serialize, Deserialize, Debug)]
pub struct PointsResponse {
    pub invite_count: u64,
    pub point: u64,
    pub energy: u64,
    pub daily_point: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountResponse {
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub lamport_id: String,
    pub name: String,
    pub address: String,
    pub x_id: String,
    pub username: String,
    pub image: String,
    pub email: String,
    pub verified: bool,
    pub invited_by: Option<String>,
    pub invited_channel: Option<String>,
    pub invite_code: String,
}

impl From<OauthUserInfo> for User {
    fn from(item: OauthUserInfo) -> Self {
        Self {
            lamport_id: gen_lamport_id(),
            name: item.data.name,
            address: gen_address(),  //TODO mock
            x_id: item.data.id,
            username: item.data.username,
            image: item.data.profile_image_url,
            email: "".to_string(),
            verified: false,
            invited_by: None,
            invited_channel: None,
            invite_code: gen_invite_code(8),
        }
    }
}

impl User {
    pub fn add_invited_by(mut self, invited: &str) -> Self {
        self.invited_by = Some(invited.to_string());
        self
    }
}

impl Into<users::ActiveModel> for User {
    fn into(self) -> users::ActiveModel {
        users::ActiveModel {
            id: NotSet,
            lamport_id: Set(self.lamport_id),
            name: Set(self.name),
            address: Set(self.address),
            x_id: Set(self.x_id),
            user_name: Set(self.username),
            image: Set(self.image),
            email: Set(self.email),
            verified: Set(self.verified),
            verified_by: Set(None),
            invited_by: Set(self.invited_by),
            invited_channel: Set(self.invited_channel),
            invite_code: Set(self.invite_code),
            created_at: Set(Some(chrono::Utc::now().into())),
            updated_at: Set(Some(chrono::Utc::now().into())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserResponse {
    pub lamport_id: String,
    pub name: String,
    pub invite_code: String,
    pub invited_by: Option<String>,
    pub image: String,
    pub verified: bool,
    pub email: String,
    pub address: String,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            lamport_id: user.lamport_id,
            name: user.name,
            invite_code: user.invite_code,
            invited_by: user.invited_by,
            image: user.image,
            verified: user.verified,
            email: user.email,
            address: user.address,
        }
    }
}

//request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub data: UserInfo,
    pub invited_by: Option<String>,
    pub invited_channel: Option<String>,
    pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_name: String,
    pub email: String,
    pub image: String,
    pub address: String,
    pub nonce: String,
}

impl From<RegisterRequest> for User {
    fn from(item: RegisterRequest) -> Self {
        Self {
            lamport_id: gen_lamport_id(),
            name: item.data.user_name.clone(),
            address: item.data.address,
            x_id: "".to_string(),
            username: item.data.user_name,
            image: item.data.image,
            email: item.data.email,
            verified: false,
            invited_by: item.invited_by,
            invited_channel: item.invited_channel,
            invite_code: gen_invite_code(8),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub data: LoginData,
    pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub content: String,
    pub address: String,
    pub nonce: String,
}

//struct for get binding twitter and impl from twitter_binding::Model
#[derive(Debug, Serialize, Deserialize)]
pub struct BindingTwitterResponse {
    pub x_id: String,
    pub name: String,
    pub user_name: String,
    pub image_url: String,
}

impl From<binding::TwitterBinding> for BindingTwitterResponse {
    fn from(binding: binding::TwitterBinding) -> Self {
        Self {
            x_id: binding.x_id,
            name: binding.name,
            user_name: binding.user_name,
            image_url: binding.image_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramParams {
    pub user_id: String,
    pub token: String,
}

//telgram binding response
#[derive(Debug, Serialize, Deserialize)]
pub struct BindingTelegramResponse {
    pub lamport_id: String,
    pub user_id: String,
}

impl From<telegram_binding::TelegramBindingModel> for BindingTelegramResponse {
    fn from(binding: telegram_binding::TelegramBindingModel) -> Self {
        Self {
            lamport_id: binding.lamport_id,
            user_id: binding.telegram_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OauthDiscordInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: u64,
    pub flags: u64,
    pub banner: Option<String>,
    pub accent_color: Option<String>,
    pub global_name: String,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub clan: Option<String>,
    pub primary_guild: Option<String>,
    pub mfa_enabled: bool,
    pub locale: String,
    pub premium_type: u64,
    pub email: String,
    pub verified: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
}

