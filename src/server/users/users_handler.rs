//use super::users_message::*;
use crate::{
    app::SharedState, 
    common::{error::{AppResult, AppError}, consts}, 
    server::{middlewares::AuthToken, user::{UserResponse,User, user_service}}
};
use axum::{
    debug_handler,
    extract::{State, Path, Json as EJson},
    Json, 
};
use serde::{Deserialize, Serialize};


//check username
#[debug_handler]
pub async fn check_username(
    State(state): State<SharedState>,
    Path(username): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let existing= state.store.is_user_exists_by_username(&username).await;
    match existing {
        Ok(true) => {
            Err(AppError::ConflictError("Name is already taken".to_string()))
        }
        Ok(false) => {
            Ok(Json(serde_json::json!({
                "result": "OK"
            })))
        }
        Err(e) => {
            Err(e)
        }
    }
}


//request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub data: UserInfo,
    pub invited_by: Option<String>,
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
            invited_by: None,
            invite_code: user_service::gen_invite_code(8),
        }
    }
}


// register
#[debug_handler]
pub async fn register(
    State(state): State<SharedState>,
    EJson(req): EJson<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    //TODO veirfy sig, nonce ? as middleware??

    let user_info: RegisterRequest = req.clone();

    let created_user = if state
        .store
        .is_user_exists_by_address(user_info.data.address.as_ref())
        .await?
    {
        state
            .store
            .get_user_by_address(user_info.data.address.as_ref())
            .await?
    } else {
        let user: User = User::from(user_info.clone());

        //points
        let user = match req.invited_by {
            Some(invited) => user.add_invited_by(invited.as_str()),
            None => user,
        };

        let created_user = match state.store.create_user(user.into()).await {
            Ok(u) => u,
            Err(AppError::UserExisted(_)) => {
                tracing::info!("user has already existed, log in");
                state
                    .store
                    .get_user_by_username(user_info.data.user_name.as_ref())
                    .await?
            }
            Err(e) => return Err(e),
        };

        state
            .store
            .create_energy(created_user.clone().lamport_id, consts::ENERGY_REGISTER, consts::ENERGY_REGISTER_VALUE)
            .await?;

        if let Some(invited_by)  = created_user.invited_by.as_deref() {
            let inviter = state.store.get_inviter_by_code(invited_by).await?;

            //award point
            state
                .store
                .award_points(inviter.lamport_id.clone(), consts::POINTS_INVITE, consts::POINTS_INVITE_VALUE, "invite reward")
                .await?;

            //consume energy 
            state
                .store
                .create_energy(inviter.lamport_id, consts::ENERGY_INVITE, consts::ENERGY_INVITE_VALUE)
                .await?;

        }


        tracing::info!("[auth_token] database  user info: {:?}", created_user);
        created_user
    };

    let secret = state.jwt_handler.clone();
    let token: String =
        secret.create_token(&created_user.lamport_id, &created_user.name, &created_user.user_name);

    tracing::info!("[auth_token] jwt token: {:?}", token);

    Ok(Json(serde_json::json!({
        "result": {
            "access_token": token,
            "user_info": UserResponse::from(created_user)
        }
    })))
}

// verify user
#[debug_handler]
pub async fn verify_user(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    Path(address): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();
    
    //TODO  verifier must be verifyed, 

    let _user = state.store.update_user(address.as_str(), claim.sub.as_str()).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "status": "success",
            "address": address
        }
    })))
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

//login
#[debug_handler]
pub async fn login(
    State(state): State<SharedState>,
    EJson(req): EJson<LoginRequest>,
) -> AppResult<Json<serde_json::Value>> {
    //TODO verify signature and address

    let user = state.store.get_user_by_address(&req.data.address).await?;
    let secret = state.jwt_handler.clone();
    let token: String =
        secret.create_token(&user.lamport_id, &user.name, &user.user_name);

    tracing::info!("[auth_token] jwt token: {:?}", token);

    Ok(Json(serde_json::json!({
        "result": {
            "access_token": token,
            "user_info": UserResponse::from(user)
        }
    })))
}

