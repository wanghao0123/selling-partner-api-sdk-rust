/* 
 * Selling Partner API for Orders
 *
 * The Selling Partner API for Orders helps you programmatically retrieve order information. These APIs let you develop fast, flexible, custom applications in areas like order synchronization, order research, and demand-based decision support tools.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// OrderItemsBuyerInfoList : A single order item's buyer information list with the order ID.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemsBuyerInfoList {
  #[serde(rename = "OrderItems")]
  order_items: ::models::OrderItemBuyerInfoList,
  /// When present and not empty, pass this string token in the next request to return the next response page.
  #[serde(rename = "NextToken")]
  next_token: Option<String>,
  /// An Amazon-defined order identifier, in 3-7-7 format.
  #[serde(rename = "AmazonOrderId")]
  amazon_order_id: String
}

impl OrderItemsBuyerInfoList {
  /// A single order item's buyer information list with the order ID.
  pub fn new(order_items: ::models::OrderItemBuyerInfoList, amazon_order_id: String) -> OrderItemsBuyerInfoList {
    OrderItemsBuyerInfoList {
      order_items: order_items,
      next_token: None,
      amazon_order_id: amazon_order_id
    }
  }

  pub fn set_order_items(&mut self, order_items: ::models::OrderItemBuyerInfoList) {
    self.order_items = order_items;
  }

  pub fn with_order_items(mut self, order_items: ::models::OrderItemBuyerInfoList) -> OrderItemsBuyerInfoList {
    self.order_items = order_items;
    self
  }

  pub fn order_items(&self) -> &::models::OrderItemBuyerInfoList {
    &self.order_items
  }


  pub fn set_next_token(&mut self, next_token: String) {
    self.next_token = Some(next_token);
  }

  pub fn with_next_token(mut self, next_token: String) -> OrderItemsBuyerInfoList {
    self.next_token = Some(next_token);
    self
  }

  pub fn next_token(&self) -> Option<&String> {
    self.next_token.as_ref()
  }

  pub fn reset_next_token(&mut self) {
    self.next_token = None;
  }

  pub fn set_amazon_order_id(&mut self, amazon_order_id: String) {
    self.amazon_order_id = amazon_order_id;
  }

  pub fn with_amazon_order_id(mut self, amazon_order_id: String) -> OrderItemsBuyerInfoList {
    self.amazon_order_id = amazon_order_id;
    self
  }

  pub fn amazon_order_id(&self) -> &String {
    &self.amazon_order_id
  }


}



