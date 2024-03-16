use axum::{Router, routing::get, response::Html};

mod example;
use example::todo::find_by_all;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hendler))
        .route("/example/todo", get(find_by_all));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn hendler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}