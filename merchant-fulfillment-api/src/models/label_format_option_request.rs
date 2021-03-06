/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LabelFormatOptionRequest : Whether to include a packing slip.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelFormatOptionRequest {
  /// When true, include a packing slip with the label.
  #[serde(rename = "IncludePackingSlipWithLabel")]
  include_packing_slip_with_label: Option<bool>
}

impl LabelFormatOptionRequest {
  /// Whether to include a packing slip.
  pub fn new() -> LabelFormatOptionRequest {
    LabelFormatOptionRequest {
      include_packing_slip_with_label: None
    }
  }

  pub fn set_include_packing_slip_with_label(&mut self, include_packing_slip_with_label: bool) {
    self.include_packing_slip_with_label = Some(include_packing_slip_with_label);
  }

  pub fn with_include_packing_slip_with_label(mut self, include_packing_slip_with_label: bool) -> LabelFormatOptionRequest {
    self.include_packing_slip_with_label = Some(include_packing_slip_with_label);
    self
  }

  pub fn include_packing_slip_with_label(&self) -> Option<&bool> {
    self.include_packing_slip_with_label.as_ref()
  }

  pub fn reset_include_packing_slip_with_label(&mut self) {
    self.include_packing_slip_with_label = None;
  }

}



