use std::sync::Arc;

use crate::Ports;
use axum::{extract::Path, response::IntoResponse, Extension};

use super::ExtractContext;

#[tracing::instrument(name = "POST /training/:training_id/cancel", skip_all, fields(request_id = %ctx.request_id, training_id = %training_id))]
async fn cancel_training(
  ExtractContext(ctx): ExtractContext,
  Extension(ports): Extension<Arc<Ports>>,
  Path(training_id): Path<u64>,
) -> impl IntoResponse {
  ports
    .training
    .cancel(training_id)
    .await
    .tap_err(|err| tracing::error!(?err))
}

pub trait TapResult<T, E> {
  fn tap_err<F>(self, f: F) -> Result<T, E>
  where
    F: FnOnce(&E);
}

impl<T, E> TapResult<T, E> for Result<T, E> {
  fn tap_err<F>(self, f: F) -> Result<T, E>
  where
    F: FnOnce(&E),
  {
    self.map_err(|err| {
      f(&err);
      err
    })
  }
}
