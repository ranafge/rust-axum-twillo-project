use axum::{body::{Body, HttpBody}, http::{Response, StatusCode}, response::IntoResponse, routing::{get, post, put}, Json, Router};
use tokio::net::TcpListener;
use serde::Serialize;



#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// endpoint /create_user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .expect("unable to create user")
}

// endpoint /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id:1,
            name: "Elijah".to_string(),
            email: "Elijah@doe.com".to_string()
        },
        User {
            id:1,
            name: "Elijah".to_string(),
            email: "Elijah@doe.com".to_string()
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }))
            .route("/create-user", post(create_user))
            .route("/create-user", put(create_user))
            .route("/user", get(list_users))
    ;

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}