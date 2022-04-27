use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::model::domain::hour::Hour;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub(crate) trait HourRepository {
  async fn get_by_time(&self, time: DateTime<Utc>) -> Result<Option<Hour>>;
  async fn save(&self, hour: Hour) -> Result<()>;
}
