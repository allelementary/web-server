use crate::models::user::User;
use bcrypt::{hash, DEFAULT_COST};
use hyper::{Body, Response, StatusCode};
use sqlx::PgPool;

pub async fn register_user_service(user_data: User, pool: PgPool) -> hyper::Result<Response<Body>> {
    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to hash password"))
                .unwrap())
        }
    };

    let result = sqlx::query!(
        "INSERT INTO users (login, email, password, wallet_address) VALUES ($1, $2, $3, $4)",
        user_data.login,
        user_data.email,
        hashed_password,
        user_data.wallet_address
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Response::new(Body::from("User registered successfully"))),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Failed to insert user: {}", e)))
            .unwrap()),
    }
}
