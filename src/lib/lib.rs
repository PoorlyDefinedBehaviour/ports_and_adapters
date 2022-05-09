pub mod adapter;
pub mod config;
pub mod context;
mod model;
mod port;

use std::{sync::Arc, time::Duration};

use config::Config;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use metrics_util::MetricKindMask;
use port::training::TrainingPort;

pub struct App {
  pub prometheus: Arc<PrometheusHandle>,
  pub ports: Ports,
}

pub struct Ports {
  pub training: TrainingPort,
}

impl App {
  pub fn new() -> Self {
    let prometheus = Arc::new(
      PrometheusBuilder::new()
        .idle_timeout(
          MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
          Some(Duration::from_secs(10)),
        )
        .install_recorder()
        .expect("failed to install Prometheus recorder"),
    );

    let config = Arc::new(Config::new().expect("error creating Config"));

    Self {
      prometheus,
      ports: Ports {
        training: TrainingPort {
          config,
          hour_repo: todo!(),
          stream_processor: todo!(),
        },
      },
    }
  }
}
