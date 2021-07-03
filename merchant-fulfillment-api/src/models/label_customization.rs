/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LabelCustomization : Custom text for shipping labels.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelCustomization {
  #[serde(rename = "CustomTextForLabel")]
  custom_text_for_label: Option<::models::CustomTextForLabel>,
  #[serde(rename = "StandardIdForLabel")]
  standard_id_for_label: Option<::models::StandardIdForLabel>
}

impl LabelCustomization {
  /// Custom text for shipping labels.
  pub fn new() -> LabelCustomization {
    LabelCustomization {
      custom_text_for_label: None,
      standard_id_for_label: None
    }
  }

  pub fn set_custom_text_for_label(&mut self, custom_text_for_label: ::models::CustomTextForLabel) {
    self.custom_text_for_label = Some(custom_text_for_label);
  }

  pub fn with_custom_text_for_label(mut self, custom_text_for_label: ::models::CustomTextForLabel) -> LabelCustomization {
    self.custom_text_for_label = Some(custom_text_for_label);
    self
  }

  pub fn custom_text_for_label(&self) -> Option<&::models::CustomTextForLabel> {
    self.custom_text_for_label.as_ref()
  }

  pub fn reset_custom_text_for_label(&mut self) {
    self.custom_text_for_label = None;
  }

  pub fn set_standard_id_for_label(&mut self, standard_id_for_label: ::models::StandardIdForLabel) {
    self.standard_id_for_label = Some(standard_id_for_label);
  }

  pub fn with_standard_id_for_label(mut self, standard_id_for_label: ::models::StandardIdForLabel) -> LabelCustomization {
    self.standard_id_for_label = Some(standard_id_for_label);
    self
  }

  pub fn standard_id_for_label(&self) -> Option<&::models::StandardIdForLabel> {
    self.standard_id_for_label.as_ref()
  }

  pub fn reset_standard_id_for_label(&mut self) {
    self.standard_id_for_label = None;
  }

}



