/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TrackingSummary : The tracking summary.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingSummary {
  /// The derived status based on the events in the eventHistory.
  #[serde(rename = "status")]
  status: Option<String>
}

impl TrackingSummary {
  /// The tracking summary.
  pub fn new() -> TrackingSummary {
    TrackingSummary {
      status: None
    }
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> TrackingSummary {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



