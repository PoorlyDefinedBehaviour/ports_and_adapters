use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Extension, Router};
use boilerplate::Context;

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();

  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(root))
    .layer(Extension(Arc::new(Context::new())));

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> &'static str {
  "hello"
}
