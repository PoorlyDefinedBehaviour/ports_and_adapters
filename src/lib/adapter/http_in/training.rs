use std::sync::Arc;

use crate::port::context::{Context, Ports};
use axum::{extract::Path, response::IntoResponse, Extension};

#[tracing::instrument(name = "POST /training/:training_id/cancel", skip_all, fields(training_id = %training_id))]
async fn cancel_training(
  Extension(_ctx): Extension<Arc<Context>>,
  Extension(ports): Extension<Arc<Ports>>,
  Path(training_id): Path<u64>,
) -> impl IntoResponse {
  match ports.training.cancel(training_id).await {
    Ok(()) => Ok(()),
    Err(err) => {
      tracing::error!(?err);
      Err(err)
    }
  }
}
