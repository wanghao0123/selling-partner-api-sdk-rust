/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AdditionalSellerInput : Additional information required to purchase shipping.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalSellerInput {
  /// The data type of the additional information.
  #[serde(rename = "DataType")]
  data_type: Option<String>,
  /// The value when the data type is string.
  #[serde(rename = "ValueAsString")]
  value_as_string: Option<String>,
  /// The value when the data type is boolean.
  #[serde(rename = "ValueAsBoolean")]
  value_as_boolean: Option<bool>,
  /// The value when the data type is integer.
  #[serde(rename = "ValueAsInteger")]
  value_as_integer: Option<i32>,
  /// The value when the data type is a date-time formatted string.
  #[serde(rename = "ValueAsTimestamp")]
  value_as_timestamp: Option<::models::Timestamp>,
  #[serde(rename = "ValueAsAddress")]
  value_as_address: Option<::models::Address>,
  #[serde(rename = "ValueAsWeight")]
  value_as_weight: Option<::models::Weight>,
  #[serde(rename = "ValueAsDimension")]
  value_as_dimension: Option<::models::Length>,
  #[serde(rename = "ValueAsCurrency")]
  value_as_currency: Option<::models::CurrencyAmount>
}

impl AdditionalSellerInput {
  /// Additional information required to purchase shipping.
  pub fn new() -> AdditionalSellerInput {
    AdditionalSellerInput {
      data_type: None,
      value_as_string: None,
      value_as_boolean: None,
      value_as_integer: None,
      value_as_timestamp: None,
      value_as_address: None,
      value_as_weight: None,
      value_as_dimension: None,
      value_as_currency: None
    }
  }

  pub fn set_data_type(&mut self, data_type: String) {
    self.data_type = Some(data_type);
  }

  pub fn with_data_type(mut self, data_type: String) -> AdditionalSellerInput {
    self.data_type = Some(data_type);
    self
  }

  pub fn data_type(&self) -> Option<&String> {
    self.data_type.as_ref()
  }

  pub fn reset_data_type(&mut self) {
    self.data_type = None;
  }

  pub fn set_value_as_string(&mut self, value_as_string: String) {
    self.value_as_string = Some(value_as_string);
  }

  pub fn with_value_as_string(mut self, value_as_string: String) -> AdditionalSellerInput {
    self.value_as_string = Some(value_as_string);
    self
  }

  pub fn value_as_string(&self) -> Option<&String> {
    self.value_as_string.as_ref()
  }

  pub fn reset_value_as_string(&mut self) {
    self.value_as_string = None;
  }

  pub fn set_value_as_boolean(&mut self, value_as_boolean: bool) {
    self.value_as_boolean = Some(value_as_boolean);
  }

  pub fn with_value_as_boolean(mut self, value_as_boolean: bool) -> AdditionalSellerInput {
    self.value_as_boolean = Some(value_as_boolean);
    self
  }

  pub fn value_as_boolean(&self) -> Option<&bool> {
    self.value_as_boolean.as_ref()
  }

  pub fn reset_value_as_boolean(&mut self) {
    self.value_as_boolean = None;
  }

  pub fn set_value_as_integer(&mut self, value_as_integer: i32) {
    self.value_as_integer = Some(value_as_integer);
  }

  pub fn with_value_as_integer(mut self, value_as_integer: i32) -> AdditionalSellerInput {
    self.value_as_integer = Some(value_as_integer);
    self
  }

  pub fn value_as_integer(&self) -> Option<&i32> {
    self.value_as_integer.as_ref()
  }

  pub fn reset_value_as_integer(&mut self) {
    self.value_as_integer = None;
  }

  pub fn set_value_as_timestamp(&mut self, value_as_timestamp: ::models::Timestamp) {
    self.value_as_timestamp = Some(value_as_timestamp);
  }

  pub fn with_value_as_timestamp(mut self, value_as_timestamp: ::models::Timestamp) -> AdditionalSellerInput {
    self.value_as_timestamp = Some(value_as_timestamp);
    self
  }

  pub fn value_as_timestamp(&self) -> Option<&::models::Timestamp> {
    self.value_as_timestamp.as_ref()
  }

  pub fn reset_value_as_timestamp(&mut self) {
    self.value_as_timestamp = None;
  }

  pub fn set_value_as_address(&mut self, value_as_address: ::models::Address) {
    self.value_as_address = Some(value_as_address);
  }

  pub fn with_value_as_address(mut self, value_as_address: ::models::Address) -> AdditionalSellerInput {
    self.value_as_address = Some(value_as_address);
    self
  }

  pub fn value_as_address(&self) -> Option<&::models::Address> {
    self.value_as_address.as_ref()
  }

  pub fn reset_value_as_address(&mut self) {
    self.value_as_address = None;
  }

  pub fn set_value_as_weight(&mut self, value_as_weight: ::models::Weight) {
    self.value_as_weight = Some(value_as_weight);
  }

  pub fn with_value_as_weight(mut self, value_as_weight: ::models::Weight) -> AdditionalSellerInput {
    self.value_as_weight = Some(value_as_weight);
    self
  }

  pub fn value_as_weight(&self) -> Option<&::models::Weight> {
    self.value_as_weight.as_ref()
  }

  pub fn reset_value_as_weight(&mut self) {
    self.value_as_weight = None;
  }

  pub fn set_value_as_dimension(&mut self, value_as_dimension: ::models::Length) {
    self.value_as_dimension = Some(value_as_dimension);
  }

  pub fn with_value_as_dimension(mut self, value_as_dimension: ::models::Length) -> AdditionalSellerInput {
    self.value_as_dimension = Some(value_as_dimension);
    self
  }

  pub fn value_as_dimension(&self) -> Option<&::models::Length> {
    self.value_as_dimension.as_ref()
  }

  pub fn reset_value_as_dimension(&mut self) {
    self.value_as_dimension = None;
  }

  pub fn set_value_as_currency(&mut self, value_as_currency: ::models::CurrencyAmount) {
    self.value_as_currency = Some(value_as_currency);
  }

  pub fn with_value_as_currency(mut self, value_as_currency: ::models::CurrencyAmount) -> AdditionalSellerInput {
    self.value_as_currency = Some(value_as_currency);
    self
  }

  pub fn value_as_currency(&self) -> Option<&::models::CurrencyAmount> {
    self.value_as_currency.as_ref()
  }

  pub fn reset_value_as_currency(&mut self) {
    self.value_as_currency = None;
  }

}



