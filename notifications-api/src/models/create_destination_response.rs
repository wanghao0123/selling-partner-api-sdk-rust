/* 
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateDestinationResponse : The response schema for the createDestination operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDestinationResponse {
  /// The payload for the createDestination operation.
  #[serde(rename = "payload")]
  payload: Option<::models::Destination>,
  /// One or more unexpected errors occurred during the createDestination operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl CreateDestinationResponse {
  /// The response schema for the createDestination operation.
  pub fn new() -> CreateDestinationResponse {
    CreateDestinationResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::Destination) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::Destination) -> CreateDestinationResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::Destination> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> CreateDestinationResponse {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&::models::ErrorList> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

}



