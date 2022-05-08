use axum::{response::IntoResponse, Extension};
use metrics_exporter_prometheus::PrometheusHandle;
use std::sync::Arc;

#[tracing::instrument(name = "GET /metrics", skip_all)]
pub async fn handler(Extension(prometheus): Extension<Arc<PrometheusHandle>>) -> impl IntoResponse {
  prometheus.render()
}
