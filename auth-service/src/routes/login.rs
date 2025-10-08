use axum::response::Html;
pub async fn login() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}