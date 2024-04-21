use crate::models::user::User;
use crate::services::user::register_user_service;
use hyper::{Body, Request, Response, StatusCode};
use serde_json;
use sqlx::PgPool;

pub async fn register_user(req: Request<Body>, pool: PgPool) -> hyper::Result<Response<Body>> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let user_data: Result<User, _> = serde_json::from_slice(&whole_body);

    match user_data {
        Ok(data) => match register_user_service(data, pool).await {
            Ok(_) => Ok(Response::new(Body::from("User registered successfully"))),
            Err(_) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to register user"))
                .unwrap()),
        },
        Err(_) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid user data"))
            .unwrap()),
    }
}
