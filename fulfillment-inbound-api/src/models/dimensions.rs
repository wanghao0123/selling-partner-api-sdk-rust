/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Dimensions : The dimension values and unit of measurement.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimensions {
  /// The length dimension.
  #[serde(rename = "Length")]
  length: ::models::BigDecimalType,
  /// The width dimension.
  #[serde(rename = "Width")]
  width: ::models::BigDecimalType,
  /// The height dimension.
  #[serde(rename = "Height")]
  height: ::models::BigDecimalType,
  /// The unit of measurement for the dimensions.
  #[serde(rename = "Unit")]
  unit: ::models::UnitOfMeasurement
}

impl Dimensions {
  /// The dimension values and unit of measurement.
  pub fn new(length: ::models::BigDecimalType, width: ::models::BigDecimalType, height: ::models::BigDecimalType, unit: ::models::UnitOfMeasurement) -> Dimensions {
    Dimensions {
      length: length,
      width: width,
      height: height,
      unit: unit
    }
  }

  pub fn set_length(&mut self, length: ::models::BigDecimalType) {
    self.length = length;
  }

  pub fn with_length(mut self, length: ::models::BigDecimalType) -> Dimensions {
    self.length = length;
    self
  }

  pub fn length(&self) -> &::models::BigDecimalType {
    &self.length
  }


  pub fn set_width(&mut self, width: ::models::BigDecimalType) {
    self.width = width;
  }

  pub fn with_width(mut self, width: ::models::BigDecimalType) -> Dimensions {
    self.width = width;
    self
  }

  pub fn width(&self) -> &::models::BigDecimalType {
    &self.width
  }


  pub fn set_height(&mut self, height: ::models::BigDecimalType) {
    self.height = height;
  }

  pub fn with_height(mut self, height: ::models::BigDecimalType) -> Dimensions {
    self.height = height;
    self
  }

  pub fn height(&self) -> &::models::BigDecimalType {
    &self.height
  }


  pub fn set_unit(&mut self, unit: ::models::UnitOfMeasurement) {
    self.unit = unit;
  }

  pub fn with_unit(mut self, unit: ::models::UnitOfMeasurement) -> Dimensions {
    self.unit = unit;
    self
  }

  pub fn unit(&self) -> &::models::UnitOfMeasurement {
    &self.unit
  }


}



