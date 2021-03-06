/* 
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  /// The purchase order number for this order. Formatting Notes: 8-character alpha-numeric code.
  #[serde(rename = "purchaseOrderNumber")]
  purchase_order_number: String,
  /// This field will contain the current state of the purchase order.
  #[serde(rename = "purchaseOrderState")]
  purchase_order_state: String,
  /// Details of an order.
  #[serde(rename = "orderDetails")]
  order_details: Option<::models::OrderDetails>
}

impl Order {
  pub fn new(purchase_order_number: String, purchase_order_state: String) -> Order {
    Order {
      purchase_order_number: purchase_order_number,
      purchase_order_state: purchase_order_state,
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


  pub fn set_purchase_order_state(&mut self, purchase_order_state: String) {
    self.purchase_order_state = purchase_order_state;
  }

  pub fn with_purchase_order_state(mut self, purchase_order_state: String) -> Order {
    self.purchase_order_state = purchase_order_state;
    self
  }

  pub fn purchase_order_state(&self) -> &String {
    &self.purchase_order_state
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



