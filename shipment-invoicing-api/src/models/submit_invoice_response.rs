/* 
 * Selling Partner API for Shipment Invoicing
 *
 * The Selling Partner API for Shipment Invoicing helps you programmatically retrieve shipment invoice information in the Brazil marketplace for a selling partner’s Fulfillment by Amazon (FBA) orders.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SubmitInvoiceResponse : The response schema for the submitInvoice operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitInvoiceResponse {
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl SubmitInvoiceResponse {
  /// The response schema for the submitInvoice operation.
  pub fn new() -> SubmitInvoiceResponse {
    SubmitInvoiceResponse {
      errors: None
    }
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> SubmitInvoiceResponse {
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



