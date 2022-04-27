//! Holds data, entities and ports that are used by the whole application and should be available to every adapter.

use super::training::TrainingPort;

pub(crate) struct Context {
  pub(crate) ports: Ports,
}

impl Context {
  pub fn new() -> Self {
    Self{
      ports: Ports::new()
    }
  }
}

pub(crate) struct Ports {
  pub(crate) training: TrainingPort,
}

impl Ports {
  training: TrainingPort{}
}

