//! Holds data, entities and ports that are used by the whole application and should be available to every adapter.

use super::training::TrainingPort;

pub struct Context {
  pub(crate) topics: Topics,
}

pub(crate) struct Topics {
  pub(crate) training_scheduled: String,
}

impl Context {
  pub fn new() -> Self {
    Self {
      topics: Topics {
        training_scheduled: String::from("topic_here"),
      },
    }
  }
}

pub(crate) struct Ports {
  pub(crate) training: TrainingPort,
}

impl Ports {
  fn new() -> Self {
    let hour_repo = todo!();
    let stream_processor = todo!();

    Self {
      training: TrainingPort {
        hour_repo,
        stream_processor,
      },
    }
  }
}
