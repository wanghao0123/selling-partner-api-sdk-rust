/* 
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SubmitAcknowledgementRequest : The request schema for the submitAcknowledgment operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitAcknowledgementRequest {
  #[serde(rename = "acknowledgements")]
  acknowledgements: Option<Vec<::models::OrderAcknowledgement>>
}

impl SubmitAcknowledgementRequest {
  /// The request schema for the submitAcknowledgment operation.
  pub fn new() -> SubmitAcknowledgementRequest {
    SubmitAcknowledgementRequest {
      acknowledgements: None
    }
  }

  pub fn set_acknowledgements(&mut self, acknowledgements: Vec<::models::OrderAcknowledgement>) {
    self.acknowledgements = Some(acknowledgements);
  }

  pub fn with_acknowledgements(mut self, acknowledgements: Vec<::models::OrderAcknowledgement>) -> SubmitAcknowledgementRequest {
    self.acknowledgements = Some(acknowledgements);
    self
  }

  pub fn acknowledgements(&self) -> Option<&Vec<::models::OrderAcknowledgement>> {
    self.acknowledgements.as_ref()
  }

  pub fn reset_acknowledgements(&mut self) {
    self.acknowledgements = None;
  }

}



