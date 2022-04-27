use anyhow::Result;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use thiserror::Error;

use crate::model::domain::hour::Hour;

use super::contract::repository::HourRepository;

pub(crate) struct TrainingPort {
  hour_repo: Arc<dyn HourRepository>,
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum TrainingError {
  #[error("hour not found: {0}")]
  HourNotFound(DateTime<Utc>),
}

impl TrainingPort {
  pub async fn schedule(&self, time: DateTime<Utc>) -> Result<()> {
    match self.hour_repo.get_by_time(time).await? {
      None => Err(TrainingError::HourNotFound(time).into()),
      Some(hour) => self.hour_repo.save(hour.schedule_traning()?).await,
    }
  }

  pub async fn cancel(&self, training_id: u64) -> Result<(), ()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::{model::domain::hour::HourError, port::contract::repository::MockHourRepository};

  use super::*;
  use mockall::predicate::*;

  #[tokio::test]
  async fn schedule_hour_doesnt_exist() {
    // Given
    let mut hour_repo = MockHourRepository::new();

    let time = Utc::now();

    hour_repo
      .expect_get_by_time()
      .once()
      .with(eq(time))
      .return_once(|_| Box::pin(async { Ok(None) }));

    // When
    let port = TrainingPort {
      hour_repo: Arc::new(hour_repo),
    };

    let actual = port
      .schedule(time)
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

    let time = Utc::now();

    let scheduled_hour = Hour::new(time).schedule_traning()?;

    hour_repo
      .expect_get_by_time()
      .once()
      .with(eq(time))
      .return_once(|_| Box::pin(async { Ok(Some(scheduled_hour)) }));

    // When
    let port = TrainingPort {
      hour_repo: Arc::new(hour_repo),
    };

    let actual = port
      .schedule(time)
      .await
      .unwrap_err()
      .downcast::<HourError>()
      .unwrap();

    // Then
    assert_eq!(HourError::HourAlreadyScheduled, actual);

    Ok(())
  }
}
