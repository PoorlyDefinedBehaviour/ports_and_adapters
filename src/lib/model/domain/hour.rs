use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, PartialEq)]
enum Availability {
  /// The hour is available to be scheduled
  Available,
  /// The hour is not available to be scheduled
  Unavailable,
  /// The hour already has a training scheduled
  TrainingScheduled,
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum HourError {
  #[error("operation required hour to have a training scheduled")]
  NoTrainingScheduled,
  #[error("hour already scheduled")]
  HourAlreadyScheduled,
}

// Typestate pattern to enforce invariants at compile time
#[derive(Debug, PartialEq)]
pub(crate) struct Hour {
  hour: DateTime<Utc>,
  availability: Availability,
}

impl Hour {
  /// Creates a new Hour with the available status
  pub fn new(hour: DateTime<Utc>) -> Self {
    Self {
      hour,
      availability: Availability::Available,
    }
  }

  /// Returns true when the hour already has a training scheduled for it
  pub fn has_training_scheduled(&self) -> bool {
    match self.availability {
      Availability::Available | Availability::Unavailable => false,
      Availability::TrainingScheduled => true,
    }
  }

  /// Returns true when hour is available
  pub fn is_available(&self) -> bool {
    match self.availability {
      Availability::Available => true,
      Availability::Unavailable | Availability::TrainingScheduled => false,
    }
  }

  /// Schedules training for this hour
  pub fn schedule_traning(self) -> Result<Hour, HourError> {
    if self.has_training_scheduled() {
      Err(HourError::HourAlreadyScheduled)
    } else {
      Ok(Hour {
        hour: self.hour,
        availability: Availability::TrainingScheduled,
      })
    }
  }

  // The hour becomes unavailable for scheduling
  pub fn make_unavailable(self) -> Hour {
    Hour {
      hour: self.hour,
      availability: Availability::Unavailable,
    }
  }

  // The hour becomes available for scheduling
  pub fn make_available(self) -> Hour {
    Hour {
      hour: self.hour,
      availability: Availability::Available,
    }
  }

  pub fn cancel_training(self) -> Result<Hour, HourError> {
    if self.has_training_scheduled() {
      Ok(Hour {
        hour: self.hour,
        availability: Availability::Available,
      })
    } else {
      Err(HourError::NoTrainingScheduled)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Utc;

  #[test]
  fn available_hour_doesnt_have_training_scheduled() {
    // Given
    let hour = Hour::new(Utc::now());

    // Then
    assert!(!hour.has_training_scheduled());
  }

  #[test]
  fn available_hour_can_be_made_unavailable() {
    // Given
    let hour = Hour::new(Utc::now());

    // When
    let hour = hour.make_unavailable();

    // Then
    assert!(!hour.is_available());
  }

  #[test]
  fn available_hour_can_schedule_training() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let hour = Hour::new(Utc::now());

    // When
    let hour = hour.schedule_traning()?;

    // Then
    assert!(hour.has_training_scheduled());
    assert!(!hour.is_available());

    Ok(())
  }

  #[test]
  fn cannot_schedule_training_if_hour_already_has_training_scheduled(
  ) -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let hour = Hour::new(Utc::now()).schedule_traning()?;

    // When
    let actual = hour.schedule_traning();

    // Then
    assert_eq!(Err(HourError::HourAlreadyScheduled), actual);

    Ok(())
  }

  #[test]
  fn unavailable_hour_doesnt_have_training_scheduled() {
    // Given
    let hour = Hour::new(Utc::now());

    // When
    let hour = hour.make_unavailable();

    // Then
    assert!(!hour.has_training_scheduled());
    assert!(!hour.is_available());
  }

  #[test]
  fn unavailable_hour_can_be_made_available() {
    // Given
    let hour = Hour::new(Utc::now()).make_unavailable();

    // When
    let hour = hour.make_available();

    // Then
    assert!(hour.is_available());
  }

  #[test]
  fn hour_has_training_scheduled() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let hour = Hour::new(Utc::now());

    // When
    let hour = hour.schedule_traning()?;

    // Then
    assert!(hour.has_training_scheduled());

    Ok(())
  }

  #[test]
  fn hour_with_training_scheduled_can_have_training_canceled(
  ) -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let hour = Hour::new(Utc::now()).schedule_traning()?;

    // When
    let hour = hour.cancel_training()?;

    // Then
    assert!(!hour.has_training_scheduled());

    Ok(())
  }

  #[test]
  fn cant_cancel_training_if_hour_does_not_have_training_scheduled() {
    // Given
    let hour = Hour::new(Utc::now());

    // Then
    assert_eq!(Err(HourError::NoTrainingScheduled), hour.cancel_training());

    // Given
    let hour = Hour::new(Utc::now()).make_unavailable();

    // Then
    assert_eq!(Err(HourError::NoTrainingScheduled), hour.cancel_training());
  }
}
