use axum::{
    extract::{Path, Query}, response::{Html, IntoResponse}, routing::get, Router
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().merge(routes_hello()) ;
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING http://0.0.0.0:3000 \n");
    axum::serve(listener, app).await.unwrap();
}

//Routes Hello
fn routes_hello() -> Router{
        Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

// Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>,
}

// e.g, `/hello?name=batman`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse{
    println!("->> {:<12} - handler_hello - {params:?}","HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello, <strong>{name}</strong>"))
}

// e.g, `/hello2/batman`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse{
    println!("->> {:<12} - handler_hello2 - {name:?}","HANDLER");
    Html(format!("Hello, <strong>{name}</strong>"))

}
