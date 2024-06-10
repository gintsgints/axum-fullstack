use axum::{
    http::{HeaderMap, StatusCode}, routing::{get, post}, Json, Router
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use super::model::{Claims, LoginInfo, LoginResponse};
pub struct UsersRouter;

impl UsersRouter {
    pub fn new_router() -> Router {

        Router::new()
            .route("/login", post(UsersRouter::login_handler))
            .route("/info", get(UsersRouter::get_info_handler))
    }

    async fn login_handler(
        Json(login_info): Json<LoginInfo>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
        let username = &login_info.username;
        let password = &login_info.password;

        let is_valid = UsersRouter::is_valid_user(username, password);

        if is_valid {
            let claims = Claims {
                sub: username.clone(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            };

            let token = match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            ) {
                Ok(tok) => tok,
                Err(err) => {
                    eprintln!("Error creating token: {}", err);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };

            Ok(Json(LoginResponse { token }))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    async fn get_info_handler(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
        if let Some(auth_header) = header_map.get("Authorization") {
            if let Ok(auth_header_str) = auth_header.to_str() {
                if auth_header_str.starts_with("Bearer ") {
                    let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                    return if let Err(err) = decode::<Claims>(
                        &token,
                        &DecodingKey::from_secret("secret".as_ref()),
                        &Validation::default(),
                    ) {
                        eprintln!("Error creating token: {}", err);
                        Err(StatusCode::UNAUTHORIZED)
                    } else {
                        let info = "You are valid. Here is info".to_string();
                        Ok(Json(info))
                    };
                }
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }

    fn is_valid_user(username: &str, password: &str) -> bool {
        !username.is_empty() && !password.is_empty()
    }
}
