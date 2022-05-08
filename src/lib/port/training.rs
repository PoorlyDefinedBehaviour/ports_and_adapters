use anyhow::Result;
use chrono::{DateTime, Utc};
use metrics::{decrement_gauge, increment_counter, increment_gauge};
use std::sync::Arc;
use thiserror::Error;

use crate::Context;

use super::contract::{
  repository::HourRepository,
  stream_processor::{SendInput, StreamProcessor},
};

pub(crate) struct TrainingPort {
  pub(crate) hour_repo: Arc<dyn HourRepository + Send + Sync>,
  pub(crate) stream_processor: Arc<dyn StreamProcessor + Send + Sync>,
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum TrainingError {
  #[error("hour not found: {0}")]
  HourNotFound(DateTime<Utc>),
}

impl TrainingPort {
  #[tracing::instrument(name = "Schedule training", skip_all, fields(time = ?time))]
  pub async fn schedule(&self, ctx: &Context, time: DateTime<Utc>) -> Result<()> {
    match self.hour_repo.get_by_time(time).await? {
      None => Err(TrainingError::HourNotFound(time).into()),
      Some(hour) => {
        self.hour_repo.save(hour.schedule_traning()?).await?;

        match self
          .stream_processor
          .send(SendInput {
            topic: ctx.topics.training_scheduled.clone(),
            key: None,
            payload: "hello world".as_bytes().to_vec(),
          })
          .await
        {
          Err(err) => Err(err.into()),
          Ok(_) => {
            increment_gauge!("training_scheduled", 1.0);
            Ok(())
          }
        }
      }
    }
  }

  #[tracing::instrument(name = "Cancelling training", skip_all, fields(training_id = training_id))]
  pub async fn cancel(&self, training_id: u64) -> Result<(), ()> {
    decrement_gauge!("training_scheduled", 1.0);
    increment_counter!("training_cancelled");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    model::domain::hour::{Hour, HourError},
    port::contract::{repository::MockHourRepository, stream_processor::MockStreamProcessor},
  };

  use super::*;
  use mockall::predicate::*;

  #[tokio::test]
  async fn schedule_hour_doesnt_exist() {
    // Given
    let mut hour_repo = MockHourRepository::new();
    let stream_processor = MockStreamProcessor::new();

    let time = Utc::now();

    hour_repo
      .expect_get_by_time()
      .once()
      .with(eq(time))
      .return_once(|_| Ok(None));

    // When
    let port = TrainingPort {
      hour_repo: Arc::new(hour_repo),
      stream_processor: Arc::new(stream_processor),
    };

    let actual = port
      .schedule(&Context::new(), time)
      .await
      .unwrap_err()
      .downcast::<TrainingError>()
      .unwrap();

    // Then
    assert_eq!(TrainingError::HourNotFound(time), actual);
  }

  #[tokio::test]
  async fn schedule_hour_already_has_training_scheduled() -> Result<(), Box<dyn std::error::Error>>
  {
    // Given
    let mut hour_repo = MockHourRepository::new();
    let stream_processor = MockStreamProcessor::new();

    let time = Utc::now();

    let scheduled_hour = Hour::new(time).schedule_traning()?;

    hour_repo
      .expect_get_by_time()
      .once()
      .with(eq(time))
      .return_once(|_| Ok(Some(scheduled_hour)));

    // When
    let port = TrainingPort {
      hour_repo: Arc::new(hour_repo),
      stream_processor: Arc::new(stream_processor),
    };

    let actual = port
      .schedule(&Context::new(), time)
      .await
      .unwrap_err()
      .downcast::<HourError>()
      .unwrap();

    // Then
    assert_eq!(HourError::HourAlreadyScheduled, actual);

    Ok(())
  }

  #[tokio::test]
  async fn schedules_hour_and_publishes_message() -> Result<(), Box<dyn std::error::Error>> {
    let time = Utc::now();

    let hour = Hour::new(time);

    let mut hour_repo = MockHourRepository::new();

    // Hour is available
    hour_repo
      .expect_get_by_time()
      .once()
      .with(eq(time))
      .return_once(|_| Ok(Some(hour)));

    // Should store Hour in the database
    hour_repo.expect_save().once().return_once(|_| Ok(()));

    let mut stream_processor = MockStreamProcessor::new();

    // Should publish message
    let ctx = Context::new();

    stream_processor
      .expect_send()
      .once()
      .with(eq(SendInput {
        topic: ctx.topics.training_scheduled.clone(),
        key: None,
        payload: "hello world".as_bytes().to_vec(),
      }))
      .return_once(|_| Ok(()));

    let port = TrainingPort {
      hour_repo: Arc::new(hour_repo),
      stream_processor: Arc::new(stream_processor),
    };

    let result = port.schedule(&ctx, time).await;

    assert!(result.is_ok());

    Ok(())
  }
}
