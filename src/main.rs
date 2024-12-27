use axum::{
    extract::{Path, Query}, http::{StatusCode, Uri}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router
};
use serde::Deserialize;
// use tower_http::services::ServeDir;
pub use self::error::{Error, Result};
//error handling
mod error;
mod web;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback(fallback);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING http://0.0.0.0:3000 \n");
    axum::serve(listener, app).await.unwrap();
}

// fn route_static() -> Router{
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
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

//fallback
async fn fallback(uri: Uri) -> impl IntoResponse {
    println!("->> {:<12} - fallback","FALLBACK");
    (StatusCode::NOT_FOUND, format!("You are lost!, No route for {uri}"))
}
