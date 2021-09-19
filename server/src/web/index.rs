use axum::response::Html;

use crate::web::SharedState;

pub async fn index() -> Html<&'static str> {
    Html::from("<h1>Cyber-Beep</h1>")
}

pub async fn ping() -> &'static str {
    "pong"
}