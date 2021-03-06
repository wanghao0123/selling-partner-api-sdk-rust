/* 
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateSubscriptionRequest : The request schema for the createSubscription operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
  /// The version of the payload object to be used in the notification.
  #[serde(rename = "payloadVersion")]
  payload_version: Option<String>,
  /// The identifier for the destination where notifications will be delivered.
  #[serde(rename = "destinationId")]
  destination_id: Option<String>
}

impl CreateSubscriptionRequest {
  /// The request schema for the createSubscription operation.
  pub fn new() -> CreateSubscriptionRequest {
    CreateSubscriptionRequest {
      payload_version: None,
      destination_id: None
    }
  }

  pub fn set_payload_version(&mut self, payload_version: String) {
    self.payload_version = Some(payload_version);
  }

  pub fn with_payload_version(mut self, payload_version: String) -> CreateSubscriptionRequest {
    self.payload_version = Some(payload_version);
    self
  }

  pub fn payload_version(&self) -> Option<&String> {
    self.payload_version.as_ref()
  }

  pub fn reset_payload_version(&mut self) {
    self.payload_version = None;
  }

  pub fn set_destination_id(&mut self, destination_id: String) {
    self.destination_id = Some(destination_id);
  }

  pub fn with_destination_id(mut self, destination_id: String) -> CreateSubscriptionRequest {
    self.destination_id = Some(destination_id);
    self
  }

  pub fn destination_id(&self) -> Option<&String> {
    self.destination_id.as_ref()
  }

  pub fn reset_destination_id(&mut self) {
    self.destination_id = None;
  }

}



