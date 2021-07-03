/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UpdateFulfillmentOrderResponse : The response schema for the updateFulfillmentOrder operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFulfillmentOrderResponse {
  /// One or more unexpected errors occurred during the updateFulfillmentOrder operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl UpdateFulfillmentOrderResponse {
  /// The response schema for the updateFulfillmentOrder operation.
  pub fn new() -> UpdateFulfillmentOrderResponse {
    UpdateFulfillmentOrderResponse {
      errors: None
    }
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> UpdateFulfillmentOrderResponse {
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



