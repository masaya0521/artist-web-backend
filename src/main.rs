use axum::{Router, routing::get, response::Html};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hendler));

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