/* 
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SubmitShipmentConfirmationsResponse : The response schema for the submitShipmentConfirmations operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitShipmentConfirmationsResponse {
  /// The response payload for the submitShipmentConfirmations operation.
  #[serde(rename = "payload")]
  payload: Option<::models::TransactionReference>,
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl SubmitShipmentConfirmationsResponse {
  /// The response schema for the submitShipmentConfirmations operation.
  pub fn new() -> SubmitShipmentConfirmationsResponse {
    SubmitShipmentConfirmationsResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::TransactionReference) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::TransactionReference) -> SubmitShipmentConfirmationsResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::TransactionReference> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> SubmitShipmentConfirmationsResponse {
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



