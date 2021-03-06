/* 
 * Selling Partner API for FBA Inbound Eligibilty
 *
 * With the FBA Inbound Eligibility API, you can build applications that let sellers get eligibility previews for items before shipping them to Amazon's fulfillment centers. With this API you can find out if an item is eligible for inbound shipment to Amazon's fulfillment centers in a specific marketplace. You can also find out if an item is eligible for using the manufacturer barcode for FBA inventory tracking. Sellers can use this information to inform their decisions about which items to ship Amazon's fulfillment centers.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetItemEligibilityPreviewResponse : The response schema for the getItemEligibilityPreview operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetItemEligibilityPreviewResponse {
  /// The payload for the getItemEligibilityPreview operation.
  #[serde(rename = "payload")]
  payload: Option<::models::ItemEligibilityPreview>,
  /// An unexpected condition occurred during the GetItemEligibilityPreview operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl GetItemEligibilityPreviewResponse {
  /// The response schema for the getItemEligibilityPreview operation.
  pub fn new() -> GetItemEligibilityPreviewResponse {
    GetItemEligibilityPreviewResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::ItemEligibilityPreview) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::ItemEligibilityPreview) -> GetItemEligibilityPreviewResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::ItemEligibilityPreview> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> GetItemEligibilityPreviewResponse {
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



