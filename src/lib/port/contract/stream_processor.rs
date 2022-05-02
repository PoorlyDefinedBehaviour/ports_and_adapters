use async_trait::async_trait;
use thiserror::Error;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub(crate) trait StreamProcessor {
  async fn recv(&self) -> Result<Message, ReceiveError>;
  async fn send(&self, message: SendInput) -> Result<(), SendError>;
}

#[derive(Debug, Error)]
pub(crate) enum SendError {}

#[derive(Debug, PartialEq)]
pub struct SendInput {
  pub(crate) topic: String,
  pub(crate) key: Option<String>,
  pub(crate) payload: Vec<u8>,
}

pub(crate) type ReceiveError = String;

#[derive(Debug)]
pub(crate) struct Message {
  pub(crate) key: Option<String>,
  pub(crate) topic: String,
  pub(crate) partition: i32,
  pub(crate) offset: i64,
  pub(crate) timestamp: Option<i64>,
}
