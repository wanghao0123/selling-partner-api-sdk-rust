/* 
 * Selling Partner API for Direct Fulfillment Shipping
 *
 * The Selling Partner API for Direct Fulfillment Shipping provides programmatic access to a direct fulfillment vendor's shipping data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInvoiceList {
  #[serde(rename = "pagination")]
  pagination: Option<::models::Pagination>,
  #[serde(rename = "customerInvoices")]
  customer_invoices: Option<Vec<::models::CustomerInvoice>>
}

impl CustomerInvoiceList {
  pub fn new() -> CustomerInvoiceList {
    CustomerInvoiceList {
      pagination: None,
      customer_invoices: None
    }
  }

  pub fn set_pagination(&mut self, pagination: ::models::Pagination) {
    self.pagination = Some(pagination);
  }

  pub fn with_pagination(mut self, pagination: ::models::Pagination) -> CustomerInvoiceList {
    self.pagination = Some(pagination);
    self
  }

  pub fn pagination(&self) -> Option<&::models::Pagination> {
    self.pagination.as_ref()
  }

  pub fn reset_pagination(&mut self) {
    self.pagination = None;
  }

  pub fn set_customer_invoices(&mut self, customer_invoices: Vec<::models::CustomerInvoice>) {
    self.customer_invoices = Some(customer_invoices);
  }

  pub fn with_customer_invoices(mut self, customer_invoices: Vec<::models::CustomerInvoice>) -> CustomerInvoiceList {
    self.customer_invoices = Some(customer_invoices);
    self
  }

  pub fn customer_invoices(&self) -> Option<&Vec<::models::CustomerInvoice>> {
    self.customer_invoices.as_ref()
  }

  pub fn reset_customer_invoices(&mut self) {
    self.customer_invoices = None;
  }

}



