/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CartonReferenceDetails {
  /// Pallet level carton count is mandatory for single item pallet and optional for mixed item pallet.
  #[serde(rename = "cartonCount")]
  carton_count: Option<i32>,
  /// Array of reference numbers for the carton that are part of this pallet/shipment. Please provide the cartonSequenceNumber from the 'cartons' segment to refer to that carton's details here.
  #[serde(rename = "cartonReferenceNumbers")]
  carton_reference_numbers: Vec<String>
}

impl CartonReferenceDetails {
  pub fn new(carton_reference_numbers: Vec<String>) -> CartonReferenceDetails {
    CartonReferenceDetails {
      carton_count: None,
      carton_reference_numbers: carton_reference_numbers
    }
  }

  pub fn set_carton_count(&mut self, carton_count: i32) {
    self.carton_count = Some(carton_count);
  }

  pub fn with_carton_count(mut self, carton_count: i32) -> CartonReferenceDetails {
    self.carton_count = Some(carton_count);
    self
  }

  pub fn carton_count(&self) -> Option<&i32> {
    self.carton_count.as_ref()
  }

  pub fn reset_carton_count(&mut self) {
    self.carton_count = None;
  }

  pub fn set_carton_reference_numbers(&mut self, carton_reference_numbers: Vec<String>) {
    self.carton_reference_numbers = carton_reference_numbers;
  }

  pub fn with_carton_reference_numbers(mut self, carton_reference_numbers: Vec<String>) -> CartonReferenceDetails {
    self.carton_reference_numbers = carton_reference_numbers;
    self
  }

  pub fn carton_reference_numbers(&self) -> &Vec<String> {
    &self.carton_reference_numbers
  }


}



