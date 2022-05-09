use std::{net::SocketAddr, sync::Arc};

use boilerplate::{adapter, App};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = Arc::new(App::new());

  let router = adapter::http_in::router(app);

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  tracing::info!("listening on {}", addr);

  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}
