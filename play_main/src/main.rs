#![allow(dead_code)]
#![allow(unused)]

use anyhow::{Context, Ok};
use axum::{
    http::StatusCode, response::IntoResponse, routing::get, Json, Router
};
use serde::{Serialize, Deserialize};

// Error variant must implement `IntoResponse` (error to response)
// need newtype to warp anyhow error
struct AppError (anyhow::Error);

// This allows ? to automatically convert 
// anyhow::Error to AppError 
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

// implement IntoResponse,the actual response format for the `error` 
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}


/// Generates the message,
fn generate_message() -> anyhow::Result<&'static str>{
    if rand::random(){
        anyhow::bail!("no message for you!");
    }
    Ok("Welcome to Axum Play!")
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    
    // route method binds the home function to the GET HTTP verb 
    // on the '/' path. 
    // request Header -> GET / HTTP/1.0 ...
    // response Header -> HTTP/1.0 200 OK ...

    let app = Router::new()
        .route("/", get(home_json))
        .layer(tower_http::catch_panic::CatchPanicLayer::new()); // add middlwware
    
    // creates a listener on port 3000
    // not bind service to a specific IP - '0.0.0.0'
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;
        // .unwrap();

    // call web service with set up `lintener`, `app` 
    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;
        //.expect("failed to bind TCP listener");
        //.unwrap();

    // both the listener` and `axum` may return `error`
    // simply allow  to panic with `unwrap()`
    // expect is useful when the error is unexpected,
    // add context, need to use anyhow::Context; 
    // to add the context method to error types. 

    Ok(())
}

async fn home() -> &'static str{
    "Welcome to Axum Play!"
}

async fn home_json() -> Result<(StatusCode, Json<Response>), AppError> {
    let response = Response {
        message: generate_message().context("failed to generate message")?,
    };

    Ok((StatusCode::OK, Json(response)))
}



// `Anyhow`` should only be used in applications
// For libraries, it is recommended `thiserror`,
// CatchPanic middleware from the tower-http crate. 