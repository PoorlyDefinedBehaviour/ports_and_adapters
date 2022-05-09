use uuid::Uuid;

/// Carries request-dependent data.
pub struct Context {
  pub request_id: String,
}

impl Context {
  pub fn new() -> Self {
    Self {
      request_id: Uuid::new_v4().to_string(),
    }
  }
}
