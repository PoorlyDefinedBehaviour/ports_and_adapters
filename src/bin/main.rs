use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{routing::get, Extension, Router};
use boilerplate::{adapter, Context};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let builder = PrometheusBuilder::new();

  let prometheus = builder
    .idle_timeout(
      MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
      Some(Duration::from_secs(10)),
    )
    .install_recorder()
    .expect("failed to install Prometheus recorder");

  let app = Router::new()
    .route("/metrics", get(adapter::http_in::metrics::handler))
    .layer(Extension(Arc::new(Context::new())))
    .layer(Extension(Arc::new(prometheus)));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
