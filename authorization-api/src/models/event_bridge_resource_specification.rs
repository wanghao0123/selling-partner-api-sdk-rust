/* 
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EventBridgeResourceSpecification : The information required to create an Amazon EventBridge destination.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventBridgeResourceSpecification {
  /// The AWS region in which you will be receiving the notifications.
  #[serde(rename = "region")]
  region: String,
  /// The identifier for the AWS account that is responsible for charges related to receiving notifications.
  #[serde(rename = "accountId")]
  account_id: String
}

impl EventBridgeResourceSpecification {
  /// The information required to create an Amazon EventBridge destination.
  pub fn new(region: String, account_id: String) -> EventBridgeResourceSpecification {
    EventBridgeResourceSpecification {
      region: region,
      account_id: account_id
    }
  }

  pub fn set_region(&mut self, region: String) {
    self.region = region;
  }

  pub fn with_region(mut self, region: String) -> EventBridgeResourceSpecification {
    self.region = region;
    self
  }

  pub fn region(&self) -> &String {
    &self.region
  }


  pub fn set_account_id(&mut self, account_id: String) {
    self.account_id = account_id;
  }

  pub fn with_account_id(mut self, account_id: String) -> EventBridgeResourceSpecification {
    self.account_id = account_id;
    self
  }

  pub fn account_id(&self) -> &String {
    &self.account_id
  }


}



