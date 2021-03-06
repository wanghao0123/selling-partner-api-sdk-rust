/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CancelFulfillmentOrderResponse : The response schema for the cancelFulfillmentOrder operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelFulfillmentOrderResponse {
  /// One or more unexpected errors occurred during the cancelFulfillmentOrder operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl CancelFulfillmentOrderResponse {
  /// The response schema for the cancelFulfillmentOrder operation.
  pub fn new() -> CancelFulfillmentOrderResponse {
    CancelFulfillmentOrderResponse {
      errors: None
    }
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> CancelFulfillmentOrderResponse {
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



