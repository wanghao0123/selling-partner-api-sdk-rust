/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SellerInputDefinition : Specifies characteristics that apply to a seller input.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerInputDefinition {
  /// When true, the additional input field is required.
  #[serde(rename = "IsRequired")]
  is_required: bool,
  /// The data type of the additional input field.
  #[serde(rename = "DataType")]
  data_type: String,
  #[serde(rename = "Constraints")]
  constraints: ::models::Constraints,
  /// The display text for the additional input field.
  #[serde(rename = "InputDisplayText")]
  input_display_text: String,
  /// Whether the seller input applies to the item or the shipment.
  #[serde(rename = "InputTarget")]
  input_target: Option<::models::InputTargetType>,
  #[serde(rename = "StoredValue")]
  stored_value: ::models::AdditionalSellerInput,
  #[serde(rename = "RestrictedSetValues")]
  restricted_set_values: Option<::models::RestrictedSetValues>
}

impl SellerInputDefinition {
  /// Specifies characteristics that apply to a seller input.
  pub fn new(is_required: bool, data_type: String, constraints: ::models::Constraints, input_display_text: String, stored_value: ::models::AdditionalSellerInput) -> SellerInputDefinition {
    SellerInputDefinition {
      is_required: is_required,
      data_type: data_type,
      constraints: constraints,
      input_display_text: input_display_text,
      input_target: None,
      stored_value: stored_value,
      restricted_set_values: None
    }
  }

  pub fn set_is_required(&mut self, is_required: bool) {
    self.is_required = is_required;
  }

  pub fn with_is_required(mut self, is_required: bool) -> SellerInputDefinition {
    self.is_required = is_required;
    self
  }

  pub fn is_required(&self) -> &bool {
    &self.is_required
  }


  pub fn set_data_type(&mut self, data_type: String) {
    self.data_type = data_type;
  }

  pub fn with_data_type(mut self, data_type: String) -> SellerInputDefinition {
    self.data_type = data_type;
    self
  }

  pub fn data_type(&self) -> &String {
    &self.data_type
  }


  pub fn set_constraints(&mut self, constraints: ::models::Constraints) {
    self.constraints = constraints;
  }

  pub fn with_constraints(mut self, constraints: ::models::Constraints) -> SellerInputDefinition {
    self.constraints = constraints;
    self
  }

  pub fn constraints(&self) -> &::models::Constraints {
    &self.constraints
  }


  pub fn set_input_display_text(&mut self, input_display_text: String) {
    self.input_display_text = input_display_text;
  }

  pub fn with_input_display_text(mut self, input_display_text: String) -> SellerInputDefinition {
    self.input_display_text = input_display_text;
    self
  }

  pub fn input_display_text(&self) -> &String {
    &self.input_display_text
  }


  pub fn set_input_target(&mut self, input_target: ::models::InputTargetType) {
    self.input_target = Some(input_target);
  }

  pub fn with_input_target(mut self, input_target: ::models::InputTargetType) -> SellerInputDefinition {
    self.input_target = Some(input_target);
    self
  }

  pub fn input_target(&self) -> Option<&::models::InputTargetType> {
    self.input_target.as_ref()
  }

  pub fn reset_input_target(&mut self) {
    self.input_target = None;
  }

  pub fn set_stored_value(&mut self, stored_value: ::models::AdditionalSellerInput) {
    self.stored_value = stored_value;
  }

  pub fn with_stored_value(mut self, stored_value: ::models::AdditionalSellerInput) -> SellerInputDefinition {
    self.stored_value = stored_value;
    self
  }

  pub fn stored_value(&self) -> &::models::AdditionalSellerInput {
    &self.stored_value
  }


  pub fn set_restricted_set_values(&mut self, restricted_set_values: ::models::RestrictedSetValues) {
    self.restricted_set_values = Some(restricted_set_values);
  }

  pub fn with_restricted_set_values(mut self, restricted_set_values: ::models::RestrictedSetValues) -> SellerInputDefinition {
    self.restricted_set_values = Some(restricted_set_values);
    self
  }

  pub fn restricted_set_values(&self) -> Option<&::models::RestrictedSetValues> {
    self.restricted_set_values.as_ref()
  }

  pub fn reset_restricted_set_values(&mut self) {
    self.restricted_set_values = None;
  }

}



