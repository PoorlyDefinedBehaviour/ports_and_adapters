use std::sync::Arc;

use crate::port::context::Context;
use axum::{extract::Path, response::IntoResponse, Extension};

#[tracing::instrument(name = "POST /training/:training_id/cancel", skip_all, fields(training_id = %training_id))]
async fn cancel_training(
  Extension(ctx): Extension<Arc<Context>>,
  Path(training_id): Path<u64>,
) -> impl IntoResponse {
  match ctx.ports.training.cancel(training_id).await {
    Ok(()) => Ok(()),
    Err(err) => {
      tracing::error!(?err);
      Err(err)
    }
  }
}
