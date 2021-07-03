/* 
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Money : The monetary value of the order.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
  /// The three-digit currency code. In ISO 4217 format.
  #[serde(rename = "CurrencyCode")]
  currency_code: Option<String>,
  /// The currency amount.
  #[serde(rename = "Amount")]
  amount: Option<String>
}

impl Money {
  /// The monetary value of the order.
  pub fn new() -> Money {
    Money {
      currency_code: None,
      amount: None
    }
  }

  pub fn set_currency_code(&mut self, currency_code: String) {
    self.currency_code = Some(currency_code);
  }

  pub fn with_currency_code(mut self, currency_code: String) -> Money {
    self.currency_code = Some(currency_code);
    self
  }

  pub fn currency_code(&self) -> Option<&String> {
    self.currency_code.as_ref()
  }

  pub fn reset_currency_code(&mut self) {
    self.currency_code = None;
  }

  pub fn set_amount(&mut self, amount: String) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: String) -> Money {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&String> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

}



