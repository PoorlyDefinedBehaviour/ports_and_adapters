//! Holds data, entities and ports that are used by the whole application and should be available to every adapter.

use super::training::TrainingPort;

pub struct Context {
  pub(crate) ports: Ports,
}

impl Context {
  pub fn new() -> Self {
    Self {
      ports: Ports::new(),
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
