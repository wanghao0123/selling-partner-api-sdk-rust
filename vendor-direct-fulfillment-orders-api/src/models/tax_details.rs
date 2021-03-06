/* 
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxDetails {
  #[serde(rename = "taxRate")]
  tax_rate: Option<::models::Decimal>,
  #[serde(rename = "taxAmount")]
  tax_amount: ::models::Money,
  #[serde(rename = "taxableAmount")]
  taxable_amount: Option<::models::Money>,
  /// Tax type.
  #[serde(rename = "type")]
  _type: Option<String>
}

impl TaxDetails {
  pub fn new(tax_amount: ::models::Money) -> TaxDetails {
    TaxDetails {
      tax_rate: None,
      tax_amount: tax_amount,
      taxable_amount: None,
      _type: None
    }
  }

  pub fn set_tax_rate(&mut self, tax_rate: ::models::Decimal) {
    self.tax_rate = Some(tax_rate);
  }

  pub fn with_tax_rate(mut self, tax_rate: ::models::Decimal) -> TaxDetails {
    self.tax_rate = Some(tax_rate);
    self
  }

  pub fn tax_rate(&self) -> Option<&::models::Decimal> {
    self.tax_rate.as_ref()
  }

  pub fn reset_tax_rate(&mut self) {
    self.tax_rate = None;
  }

  pub fn set_tax_amount(&mut self, tax_amount: ::models::Money) {
    self.tax_amount = tax_amount;
  }

  pub fn with_tax_amount(mut self, tax_amount: ::models::Money) -> TaxDetails {
    self.tax_amount = tax_amount;
    self
  }

  pub fn tax_amount(&self) -> &::models::Money {
    &self.tax_amount
  }


  pub fn set_taxable_amount(&mut self, taxable_amount: ::models::Money) {
    self.taxable_amount = Some(taxable_amount);
  }

  pub fn with_taxable_amount(mut self, taxable_amount: ::models::Money) -> TaxDetails {
    self.taxable_amount = Some(taxable_amount);
    self
  }

  pub fn taxable_amount(&self) -> Option<&::models::Money> {
    self.taxable_amount.as_ref()
  }

  pub fn reset_taxable_amount(&mut self) {
    self.taxable_amount = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> TaxDetails {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



