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
pub struct CustomerInvoice {
  /// The purchase order number for this order.
  #[serde(rename = "purchaseOrderNumber")]
  purchase_order_number: String,
  /// The Base64encoded customer invoice.
  #[serde(rename = "content")]
  content: String
}

impl CustomerInvoice {
  pub fn new(purchase_order_number: String, content: String) -> CustomerInvoice {
    CustomerInvoice {
      purchase_order_number: purchase_order_number,
      content: content
    }
  }

  pub fn set_purchase_order_number(&mut self, purchase_order_number: String) {
    self.purchase_order_number = purchase_order_number;
  }

  pub fn with_purchase_order_number(mut self, purchase_order_number: String) -> CustomerInvoice {
    self.purchase_order_number = purchase_order_number;
    self
  }

  pub fn purchase_order_number(&self) -> &String {
    &self.purchase_order_number
  }


  pub fn set_content(&mut self, content: String) {
    self.content = content;
  }

  pub fn with_content(mut self, content: String) -> CustomerInvoice {
    self.content = content;
    self
  }

  pub fn content(&self) -> &String {
    &self.content
  }


}



