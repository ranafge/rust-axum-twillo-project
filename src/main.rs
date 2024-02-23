use axum::{body::{Body, HttpBody}, extract::Path, http::{Response, StatusCode}, response::{Html, IntoResponse}, routing::{delete, get, post, put}, Json, Router};
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
    println!("list user handler");
    let users = vec![
        User {
            id:2,
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


// Error handaling 


async fn delete_user(Path(user_id):Path<u64>) -> Result<Json<User>, impl IntoResponse> {
    println!("delete_user handler");
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User {
            id: user_id,
            name: "Elijah".to_string(),
            email: "Elijah@doe.com".to_string()
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("Failed to delete user: {}", e))
        ))
    }
}

async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    if user_id == 1 {
        Err("User cannot be deleted".to_string())
    }else {
        // logic to delete a user ...
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }))
            .route("/create-user", post(create_user))
            .route("/create-user", put(create_user))
            .route("/user", get(list_users))
            .route("/delete-user/:user_id", delete(delete_user))
    ;

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running on http://localhost:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}