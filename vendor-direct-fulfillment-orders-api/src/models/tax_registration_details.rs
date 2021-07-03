/* 
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TaxRegistrationDetails : Tax registration details of the entity.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxRegistrationDetails {
  /// Tax registration type for the entity.
  #[serde(rename = "taxRegistrationType")]
  tax_registration_type: Option<String>,
  /// Tax registration number for the party. For example, VAT ID.
  #[serde(rename = "taxRegistrationNumber")]
  tax_registration_number: String,
  /// Address associated with the tax registration number.
  #[serde(rename = "taxRegistrationAddress")]
  tax_registration_address: Option<::models::Address>,
  /// Tax registration message that can be used for additional tax related details.
  #[serde(rename = "taxRegistrationMessages")]
  tax_registration_messages: Option<String>
}

impl TaxRegistrationDetails {
  /// Tax registration details of the entity.
  pub fn new(tax_registration_number: String) -> TaxRegistrationDetails {
    TaxRegistrationDetails {
      tax_registration_type: None,
      tax_registration_number: tax_registration_number,
      tax_registration_address: None,
      tax_registration_messages: None
    }
  }

  pub fn set_tax_registration_type(&mut self, tax_registration_type: String) {
    self.tax_registration_type = Some(tax_registration_type);
  }

  pub fn with_tax_registration_type(mut self, tax_registration_type: String) -> TaxRegistrationDetails {
    self.tax_registration_type = Some(tax_registration_type);
    self
  }

  pub fn tax_registration_type(&self) -> Option<&String> {
    self.tax_registration_type.as_ref()
  }

  pub fn reset_tax_registration_type(&mut self) {
    self.tax_registration_type = None;
  }

  pub fn set_tax_registration_number(&mut self, tax_registration_number: String) {
    self.tax_registration_number = tax_registration_number;
  }

  pub fn with_tax_registration_number(mut self, tax_registration_number: String) -> TaxRegistrationDetails {
    self.tax_registration_number = tax_registration_number;
    self
  }

  pub fn tax_registration_number(&self) -> &String {
    &self.tax_registration_number
  }


  pub fn set_tax_registration_address(&mut self, tax_registration_address: ::models::Address) {
    self.tax_registration_address = Some(tax_registration_address);
  }

  pub fn with_tax_registration_address(mut self, tax_registration_address: ::models::Address) -> TaxRegistrationDetails {
    self.tax_registration_address = Some(tax_registration_address);
    self
  }

  pub fn tax_registration_address(&self) -> Option<&::models::Address> {
    self.tax_registration_address.as_ref()
  }

  pub fn reset_tax_registration_address(&mut self) {
    self.tax_registration_address = None;
  }

  pub fn set_tax_registration_messages(&mut self, tax_registration_messages: String) {
    self.tax_registration_messages = Some(tax_registration_messages);
  }

  pub fn with_tax_registration_messages(mut self, tax_registration_messages: String) -> TaxRegistrationDetails {
    self.tax_registration_messages = Some(tax_registration_messages);
    self
  }

  pub fn tax_registration_messages(&self) -> Option<&String> {
    self.tax_registration_messages.as_ref()
  }

  pub fn reset_tax_registration_messages(&mut self) {
    self.tax_registration_messages = None;
  }

}



