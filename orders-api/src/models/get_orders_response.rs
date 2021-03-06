/* 
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetOrdersResponse : The response schema for the getOrders operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOrdersResponse {
  /// The payload for the getOrders operation.
  #[serde(rename = "payload")]
  payload: Option<::models::OrdersList>,
  /// One or more unexpected errors occurred during the getOrders operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl GetOrdersResponse {
  /// The response schema for the getOrders operation.
  pub fn new() -> GetOrdersResponse {
    GetOrdersResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::OrdersList) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::OrdersList) -> GetOrdersResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::OrdersList> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> GetOrdersResponse {
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



