/* 
 * Selling Partner API for Retail Procurement Payments
 *
 * The Selling Partner API for Retail Procurement Payments provides programmatic access to vendors payments data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Money : An amount of money, including units in the form of currency.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
  /// Three-digit currency code in ISO 4217 format.
  #[serde(rename = "currencyCode")]
  currency_code: Option<String>,
  #[serde(rename = "amount")]
  amount: Option<::models::Decimal>
}

impl Money {
  /// An amount of money, including units in the form of currency.
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

  pub fn set_amount(&mut self, amount: ::models::Decimal) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: ::models::Decimal) -> Money {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&::models::Decimal> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

}



