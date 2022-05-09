use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use axum::{
  extract::{FromRequest, RequestParts},
  http::StatusCode,
  routing::get,
  Extension, Router,
};

use crate::{context::Context, App};

mod metrics;
mod training;

pub fn router(app: Arc<App>) -> Router {
  Router::new()
    .route("/metrics", get(metrics::handler))
    .layer(Extension(Arc::clone(&app.prometheus)))
}

pub struct ExtractContext(Context);

#[async_trait]
impl<B> FromRequest<B> for ExtractContext
where
  B: Send,
{
  type Rejection = (StatusCode, &'static str);

  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    let request_id = match req.headers().get("x-request-id") {
      None => Uuid::new_v4().to_string(),
      Some(value) => value
        .to_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|_| Uuid::new_v4().to_string()),
    };

    Ok(ExtractContext(Context { request_id }))
  }
}
