use anyhow::Result;

pub struct Config {
  pub topics: Topics,
}

pub struct Topics {
  pub training_scheduled: String,
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
      topics: Topics {
        training_scheduled: std::env::var("TRAINING_SCHEDULED_TOPIC_NAME")?,
      },
    })
  }
}
