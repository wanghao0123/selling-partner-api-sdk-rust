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
pub struct Order {
  /// The purchase order number for this order. Formatting Notes: alpha-numeric code.
  #[serde(rename = "purchaseOrderNumber")]
  purchase_order_number: String,
  /// Purchase order details.
  #[serde(rename = "orderDetails")]
  order_details: Option<::models::OrderDetails>
}

impl Order {
  pub fn new(purchase_order_number: String) -> Order {
    Order {
      purchase_order_number: purchase_order_number,
      order_details: None
    }
  }

  pub fn set_purchase_order_number(&mut self, purchase_order_number: String) {
    self.purchase_order_number = purchase_order_number;
  }

  pub fn with_purchase_order_number(mut self, purchase_order_number: String) -> Order {
    self.purchase_order_number = purchase_order_number;
    self
  }

  pub fn purchase_order_number(&self) -> &String {
    &self.purchase_order_number
  }


  pub fn set_order_details(&mut self, order_details: ::models::OrderDetails) {
    self.order_details = Some(order_details);
  }

  pub fn with_order_details(mut self, order_details: ::models::OrderDetails) -> Order {
    self.order_details = Some(order_details);
    self
  }

  pub fn order_details(&self) -> Option<&::models::OrderDetails> {
    self.order_details.as_ref()
  }

  pub fn reset_order_details(&mut self) {
    self.order_details = None;
  }

}



