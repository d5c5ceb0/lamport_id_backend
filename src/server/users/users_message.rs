use crate::server::user::{User, user_service};
use serde::{Deserialize, Serialize};


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
            lamport_id: user_service::gen_lamport_id(),
            name: item.data.user_name.clone(),
            address: item.data.address,
            x_id: "".to_string(),
            username: item.data.user_name,
            image: item.data.image,
            email: item.data.email,
            verified: false,
            invited_by: item.invited_by,
            invited_channel: item.invited_channel,
            invite_code: user_service::gen_invite_code(8),
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
