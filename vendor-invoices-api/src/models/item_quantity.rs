/* 
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ItemQuantity : Details of quantity.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemQuantity {
  /// Quantity of an item. This value should not be zero.
  #[serde(rename = "amount")]
  amount: i32,
  /// Unit of measure for the quantity.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: String,
  /// The case size, if the unit of measure value is Cases.
  #[serde(rename = "unitSize")]
  unit_size: Option<i32>
}

impl ItemQuantity {
  /// Details of quantity.
  pub fn new(amount: i32, unit_of_measure: String) -> ItemQuantity {
    ItemQuantity {
      amount: amount,
      unit_of_measure: unit_of_measure,
      unit_size: None
    }
  }

  pub fn set_amount(&mut self, amount: i32) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i32) -> ItemQuantity {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i32 {
    &self.amount
  }


  pub fn set_unit_of_measure(&mut self, unit_of_measure: String) {
    self.unit_of_measure = unit_of_measure;
  }

  pub fn with_unit_of_measure(mut self, unit_of_measure: String) -> ItemQuantity {
    self.unit_of_measure = unit_of_measure;
    self
  }

  pub fn unit_of_measure(&self) -> &String {
    &self.unit_of_measure
  }


  pub fn set_unit_size(&mut self, unit_size: i32) {
    self.unit_size = Some(unit_size);
  }

  pub fn with_unit_size(mut self, unit_size: i32) -> ItemQuantity {
    self.unit_size = Some(unit_size);
    self
  }

  pub fn unit_size(&self) -> Option<&i32> {
    self.unit_size.as_ref()
  }

  pub fn reset_unit_size(&mut self) {
    self.unit_size = None;
  }

}



