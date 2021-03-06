/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LabelDimensions : Dimensions for printing a shipping label.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelDimensions {
  /// The length dimension.
  #[serde(rename = "Length")]
  length: ::models::LabelDimension,
  /// The width dimension.
  #[serde(rename = "Width")]
  width: ::models::LabelDimension,
  /// The unit of measurement.
  #[serde(rename = "Unit")]
  unit: ::models::UnitOfLength
}

impl LabelDimensions {
  /// Dimensions for printing a shipping label.
  pub fn new(length: ::models::LabelDimension, width: ::models::LabelDimension, unit: ::models::UnitOfLength) -> LabelDimensions {
    LabelDimensions {
      length: length,
      width: width,
      unit: unit
    }
  }

  pub fn set_length(&mut self, length: ::models::LabelDimension) {
    self.length = length;
  }

  pub fn with_length(mut self, length: ::models::LabelDimension) -> LabelDimensions {
    self.length = length;
    self
  }

  pub fn length(&self) -> &::models::LabelDimension {
    &self.length
  }


  pub fn set_width(&mut self, width: ::models::LabelDimension) {
    self.width = width;
  }

  pub fn with_width(mut self, width: ::models::LabelDimension) -> LabelDimensions {
    self.width = width;
    self
  }

  pub fn width(&self) -> &::models::LabelDimension {
    &self.width
  }


  pub fn set_unit(&mut self, unit: ::models::UnitOfLength) {
    self.unit = unit;
  }

  pub fn with_unit(mut self, unit: ::models::UnitOfLength) -> LabelDimensions {
    self.unit = unit;
    self
  }

  pub fn unit(&self) -> &::models::UnitOfLength {
    &self.unit
  }


}



