use axum::{async_trait, response::IntoResponse, routing::get, Extension, Router};
use winter::*;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(handler))
        .layer(Extension(MyConstructor))
        .layer(Extension(Message(uuid::Uuid::new_v4().to_string())));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

#[derive(Clone)]
struct Message(String);

#[derive(Debug, Clone)]
struct MyConstructor;

#[async_trait]
impl Constructor for MyConstructor {
    type Target = Message;

    async fn build(&self) -> Self::Target {
        dbg!("running MyConstructor");
        let random = uuid::Uuid::new_v4();
        Message(format!("hello[{random}]"))
    }
}

async fn handler(
    singleton: SingletonFlake<Message>,
    transient: TransientFlake<MyConstructor>,
    request: RequestFlake<MyConstructor>,
) -> impl IntoResponse {
    let debug_transient = transient.get().await.0;
    let debug_transient_2 = transient.get().await.0;
    let debug_request = &request.get().0;
    let debug_request_2 = &request.get().0;
    let debug_singleton = &singleton.get().0;
    dbg!(
        debug_transient,
        debug_transient_2,
        debug_request,
        debug_request_2,
        debug_singleton
    );
}
