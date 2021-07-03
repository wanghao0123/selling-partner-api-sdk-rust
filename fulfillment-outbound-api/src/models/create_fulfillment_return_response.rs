/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateFulfillmentReturnResponse : The response schema for the createFulfillmentReturn operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFulfillmentReturnResponse {
  /// The payload for the createFulfillmentReturn operation.
  #[serde(rename = "payload")]
  payload: Option<::models::CreateFulfillmentReturnResult>,
  /// One or more unexpected errors occurred during the createFulfillmentReturn operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl CreateFulfillmentReturnResponse {
  /// The response schema for the createFulfillmentReturn operation.
  pub fn new() -> CreateFulfillmentReturnResponse {
    CreateFulfillmentReturnResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::CreateFulfillmentReturnResult) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::CreateFulfillmentReturnResult) -> CreateFulfillmentReturnResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::CreateFulfillmentReturnResult> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> CreateFulfillmentReturnResponse {
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



