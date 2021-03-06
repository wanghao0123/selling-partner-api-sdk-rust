/* 
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ItemQuantity : Details of quantity ordered.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemQuantity {
  /// Acknowledged quantity. This value should not be zero.
  #[serde(rename = "amount")]
  amount: Option<i32>,
  /// Unit of measure for the acknowledged quantity.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: Option<String>
}

impl ItemQuantity {
  /// Details of quantity ordered.
  pub fn new() -> ItemQuantity {
    ItemQuantity {
      amount: None,
      unit_of_measure: None
    }
  }

  pub fn set_amount(&mut self, amount: i32) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: i32) -> ItemQuantity {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&i32> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_unit_of_measure(&mut self, unit_of_measure: String) {
    self.unit_of_measure = Some(unit_of_measure);
  }

  pub fn with_unit_of_measure(mut self, unit_of_measure: String) -> ItemQuantity {
    self.unit_of_measure = Some(unit_of_measure);
    self
  }

  pub fn unit_of_measure(&self) -> Option<&String> {
    self.unit_of_measure.as_ref()
  }

  pub fn reset_unit_of_measure(&mut self) {
    self.unit_of_measure = None;
  }

}



