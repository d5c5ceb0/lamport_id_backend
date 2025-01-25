use super::users_message::*;
use crate::{
    app::SharedState, 
    common::{error::{AppResult, AppError}, consts}, 
    server::{middlewares::AuthToken, user::{UserResponse, User}, auth::auth_service::*},
    helpers::eip191::verify_signature,


};
use axum::{
    debug_handler,
    extract::{State, Path, Json as EJson},
    Json, 
};


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


// register
#[debug_handler]
pub async fn register(
    State(state): State<SharedState>,
    EJson(req): EJson<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    if cfg!(not(debug_assertions)) {
        let verified= verify_signature(&req.data, &req.sig, &req.data.address)?;
        if !verified {
            return Err(AppError::InvalidSignature);
        }
        tracing::info!("signature verified success");

        let redis_client = RedisClient::from(state.redis.clone());

        if let Ok(nonce) = redis_client.get_nonce(req.data.address.as_str()).await {
            tracing::info!("got nonce: {:?} by key: {:?}", nonce, req.data.address);
            if nonce != req.data.nonce {
                return Err(AppError::InputValidateError("nonce verification error".into()));
            }
        } else {
            tracing::error!("got nonce err: wrong address:{:?} ", req.data.address);
            return Err(AppError::InputValidateError(
                    "nonce is not existing".into(),
            ));
        }

        match redis_client.del_nonce(req.data.address.as_str()).await {
            Ok(_) => {
                tracing::info!("delete nonce success");
            }
            Err(e) => {
                tracing::error!("delete nonce err: {:?}", e);
            }
        }
    }


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
    
    //get user by uid
    let verifier = state.store.get_user_by_uid(claim.sub.as_str()).await?;
    if !verifier.verified {
        return Err(AppError::InputValidateError("verifier is not verified".into()));
    }

    let _user = state.store.update_user(address.as_str(), claim.sub.as_str()).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "status": "success",
            "address": address
        }
    })))
}

//login
#[debug_handler]
pub async fn login(
    State(state): State<SharedState>,
    EJson(req): EJson<LoginRequest>,
) -> AppResult<Json<serde_json::Value>> {
    if cfg!(not(debug_assertions)) {
        let verified= verify_signature(&req.data, &req.sig, &req.data.address)?;
        if !verified {
            return Err(AppError::InvalidSignature);
        }
        tracing::info!("signature verified success");

        let redis_client = RedisClient::from(state.redis.clone());

        if let Ok(nonce) = redis_client.get_nonce(req.data.address.as_str()).await {
            tracing::info!("got nonce: {:?} by key: {:?}", nonce, req.data.address);
            if nonce != req.data.nonce {
                return Err(AppError::InputValidateError("nonce verification error".into()));
            }
        } else {
            tracing::error!("got nonce err: wrong address:{:?} ", req.data.address);
            return Err(AppError::InputValidateError(
                    "nonce is not existing".into(),
            ));
        }

        match redis_client.del_nonce(req.data.address.as_str()).await {
            Ok(_) => {
                tracing::info!("delete nonce success");
            }
            Err(e) => {
                tracing::error!("delete nonce err: {:?}", e);
            }
        }
    }

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

