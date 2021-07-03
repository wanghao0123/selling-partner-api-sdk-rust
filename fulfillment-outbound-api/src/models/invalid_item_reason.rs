/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// InvalidItemReason : The reason that the item is invalid for return.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidItemReason {
  #[serde(rename = "invalidItemReasonCode")]
  invalid_item_reason_code: ::models::InvalidItemReasonCode,
  /// A human readable description of the invalid item reason code.
  #[serde(rename = "description")]
  description: String
}

impl InvalidItemReason {
  /// The reason that the item is invalid for return.
  pub fn new(invalid_item_reason_code: ::models::InvalidItemReasonCode, description: String) -> InvalidItemReason {
    InvalidItemReason {
      invalid_item_reason_code: invalid_item_reason_code,
      description: description
    }
  }

  pub fn set_invalid_item_reason_code(&mut self, invalid_item_reason_code: ::models::InvalidItemReasonCode) {
    self.invalid_item_reason_code = invalid_item_reason_code;
  }

  pub fn with_invalid_item_reason_code(mut self, invalid_item_reason_code: ::models::InvalidItemReasonCode) -> InvalidItemReason {
    self.invalid_item_reason_code = invalid_item_reason_code;
    self
  }

  pub fn invalid_item_reason_code(&self) -> &::models::InvalidItemReasonCode {
    &self.invalid_item_reason_code
  }


  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> InvalidItemReason {
    self.description = description;
    self
  }

  pub fn description(&self) -> &String {
    &self.description
  }


}



