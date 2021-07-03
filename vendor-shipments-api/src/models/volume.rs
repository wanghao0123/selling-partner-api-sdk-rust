/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Volume : The volume of the container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
  /// The unit of measurement.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: String,
  /// The measurement value.
  #[serde(rename = "value")]
  value: ::models::Decimal
}

impl Volume {
  /// The volume of the container.
  pub fn new(unit_of_measure: String, value: ::models::Decimal) -> Volume {
    Volume {
      unit_of_measure: unit_of_measure,
      value: value
    }
  }

  pub fn set_unit_of_measure(&mut self, unit_of_measure: String) {
    self.unit_of_measure = unit_of_measure;
  }

  pub fn with_unit_of_measure(mut self, unit_of_measure: String) -> Volume {
    self.unit_of_measure = unit_of_measure;
    self
  }

  pub fn unit_of_measure(&self) -> &String {
    &self.unit_of_measure
  }


  pub fn set_value(&mut self, value: ::models::Decimal) {
    self.value = value;
  }

  pub fn with_value(mut self, value: ::models::Decimal) -> Volume {
    self.value = value;
    self
  }

  pub fn value(&self) -> &::models::Decimal {
    &self.value
  }


}



