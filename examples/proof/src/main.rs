use std::{future::Future, task::Poll};

use axum::{async_trait, response::IntoResponse, routing::get, Extension, Router};
use winter::*;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(service))
        .layer(Extension(MyConstructor))
        .layer(Extension(Message(uuid::Uuid::new_v4().to_string())));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

#[derive(Clone)]
struct Message(String);

fn message() -> Message {
    let random = uuid::Uuid::new_v4();
    Message(format!("hello[{random}]"))
}

#[derive(Debug, Clone)]
struct MyConstructor;

#[async_trait]
impl Constructor for MyConstructor {
    type Target = Message;

    async fn build(&self) -> Self::Target {
        dbg!("running MyConstructor");
        message()
    }
}

async fn service(
    singleton: SingletonFlake<Message>,
    flake: TransientFlake<MyConstructor>,
) -> impl IntoResponse {
    let message = flake.get().await;
    message.0 + " " + singleton.get().0.as_str()
}

impl Future for MyConstructor {
    type Output = Message;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Ready(message())
    }
}
