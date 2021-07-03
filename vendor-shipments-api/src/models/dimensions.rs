/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Dimensions : Physical dimensional measurements of a container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimensions {
  /// The length of the container.
  #[serde(rename = "length")]
  length: ::models::Decimal,
  /// The width of the container.
  #[serde(rename = "width")]
  width: ::models::Decimal,
  /// The height of the container.
  #[serde(rename = "height")]
  height: ::models::Decimal,
  /// The unit of measure for dimensions.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: String
}

impl Dimensions {
  /// Physical dimensional measurements of a container.
  pub fn new(length: ::models::Decimal, width: ::models::Decimal, height: ::models::Decimal, unit_of_measure: String) -> Dimensions {
    Dimensions {
      length: length,
      width: width,
      height: height,
      unit_of_measure: unit_of_measure
    }
  }

  pub fn set_length(&mut self, length: ::models::Decimal) {
    self.length = length;
  }

  pub fn with_length(mut self, length: ::models::Decimal) -> Dimensions {
    self.length = length;
    self
  }

  pub fn length(&self) -> &::models::Decimal {
    &self.length
  }


  pub fn set_width(&mut self, width: ::models::Decimal) {
    self.width = width;
  }

  pub fn with_width(mut self, width: ::models::Decimal) -> Dimensions {
    self.width = width;
    self
  }

  pub fn width(&self) -> &::models::Decimal {
    &self.width
  }


  pub fn set_height(&mut self, height: ::models::Decimal) {
    self.height = height;
  }

  pub fn with_height(mut self, height: ::models::Decimal) -> Dimensions {
    self.height = height;
    self
  }

  pub fn height(&self) -> &::models::Decimal {
    &self.height
  }


  pub fn set_unit_of_measure(&mut self, unit_of_measure: String) {
    self.unit_of_measure = unit_of_measure;
  }

  pub fn with_unit_of_measure(mut self, unit_of_measure: String) -> Dimensions {
    self.unit_of_measure = unit_of_measure;
    self
  }

  pub fn unit_of_measure(&self) -> &String {
    &self.unit_of_measure
  }


}



