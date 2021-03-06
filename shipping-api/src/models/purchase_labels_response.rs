/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PurchaseLabelsResponse : The response schema for the purchaseLabels operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseLabelsResponse {
  /// The payload for purchaseLabels operation
  #[serde(rename = "payload")]
  payload: Option<::models::PurchaseLabelsResult>,
  /// Encountered errors for the operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl PurchaseLabelsResponse {
  /// The response schema for the purchaseLabels operation.
  pub fn new() -> PurchaseLabelsResponse {
    PurchaseLabelsResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::PurchaseLabelsResult) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::PurchaseLabelsResult) -> PurchaseLabelsResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::PurchaseLabelsResult> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> PurchaseLabelsResponse {
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



