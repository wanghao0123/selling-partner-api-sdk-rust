/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
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
  /// Three digit currency code in ISO 4217 format.
  #[serde(rename = "currencyCode")]
  currency_code: String,
  #[serde(rename = "amount")]
  amount: ::models::Decimal
}

impl Money {
  /// An amount of money, including units in the form of currency.
  pub fn new(currency_code: String, amount: ::models::Decimal) -> Money {
    Money {
      currency_code: currency_code,
      amount: amount
    }
  }

  pub fn set_currency_code(&mut self, currency_code: String) {
    self.currency_code = currency_code;
  }

  pub fn with_currency_code(mut self, currency_code: String) -> Money {
    self.currency_code = currency_code;
    self
  }

  pub fn currency_code(&self) -> &String {
    &self.currency_code
  }


  pub fn set_amount(&mut self, amount: ::models::Decimal) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: ::models::Decimal) -> Money {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &::models::Decimal {
    &self.amount
  }


}



