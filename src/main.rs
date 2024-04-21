mod api;
mod db;
mod models;
mod services;

use crate::api::user::register_user;
use crate::db::establish_connection;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use sqlx::PgPool;
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let pool = establish_connection().await.expect("Failed to create pool");

    let make_svc = make_service_fn(move |_conn| {
        let pool = pool.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let pool = pool.clone();
                router(req, pool)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Serving on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn router(req: Request<Body>, pool: PgPool) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/register") => match register_user(req, pool).await {
            Ok(response) => Ok(response),
            Err(_e) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap()),
        },

        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}
