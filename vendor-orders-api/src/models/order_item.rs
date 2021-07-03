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
pub struct OrderItem {
  /// Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on.
  #[serde(rename = "itemSequenceNumber")]
  item_sequence_number: String,
  /// Amazon Standard Identification Number (ASIN) of an item.
  #[serde(rename = "amazonProductIdentifier")]
  amazon_product_identifier: Option<String>,
  /// The vendor selected product identification of the item.
  #[serde(rename = "vendorProductIdentifier")]
  vendor_product_identifier: Option<String>,
  /// Item quantity ordered.
  #[serde(rename = "orderedQuantity")]
  ordered_quantity: ::models::ItemQuantity,
  /// When true, we will accept backorder confirmations for this item.
  #[serde(rename = "isBackOrderAllowed")]
  is_back_order_allowed: bool,
  /// The price to Amazon each (cost).
  #[serde(rename = "netCost")]
  net_cost: Option<::models::Money>,
  /// The price to Amazon each (list).
  #[serde(rename = "listPrice")]
  list_price: Option<::models::Money>
}

impl OrderItem {
  pub fn new(item_sequence_number: String, ordered_quantity: ::models::ItemQuantity, is_back_order_allowed: bool) -> OrderItem {
    OrderItem {
      item_sequence_number: item_sequence_number,
      amazon_product_identifier: None,
      vendor_product_identifier: None,
      ordered_quantity: ordered_quantity,
      is_back_order_allowed: is_back_order_allowed,
      net_cost: None,
      list_price: None
    }
  }

  pub fn set_item_sequence_number(&mut self, item_sequence_number: String) {
    self.item_sequence_number = item_sequence_number;
  }

  pub fn with_item_sequence_number(mut self, item_sequence_number: String) -> OrderItem {
    self.item_sequence_number = item_sequence_number;
    self
  }

  pub fn item_sequence_number(&self) -> &String {
    &self.item_sequence_number
  }


  pub fn set_amazon_product_identifier(&mut self, amazon_product_identifier: String) {
    self.amazon_product_identifier = Some(amazon_product_identifier);
  }

  pub fn with_amazon_product_identifier(mut self, amazon_product_identifier: String) -> OrderItem {
    self.amazon_product_identifier = Some(amazon_product_identifier);
    self
  }

  pub fn amazon_product_identifier(&self) -> Option<&String> {
    self.amazon_product_identifier.as_ref()
  }

  pub fn reset_amazon_product_identifier(&mut self) {
    self.amazon_product_identifier = None;
  }

  pub fn set_vendor_product_identifier(&mut self, vendor_product_identifier: String) {
    self.vendor_product_identifier = Some(vendor_product_identifier);
  }

  pub fn with_vendor_product_identifier(mut self, vendor_product_identifier: String) -> OrderItem {
    self.vendor_product_identifier = Some(vendor_product_identifier);
    self
  }

  pub fn vendor_product_identifier(&self) -> Option<&String> {
    self.vendor_product_identifier.as_ref()
  }

  pub fn reset_vendor_product_identifier(&mut self) {
    self.vendor_product_identifier = None;
  }

  pub fn set_ordered_quantity(&mut self, ordered_quantity: ::models::ItemQuantity) {
    self.ordered_quantity = ordered_quantity;
  }

  pub fn with_ordered_quantity(mut self, ordered_quantity: ::models::ItemQuantity) -> OrderItem {
    self.ordered_quantity = ordered_quantity;
    self
  }

  pub fn ordered_quantity(&self) -> &::models::ItemQuantity {
    &self.ordered_quantity
  }


  pub fn set_is_back_order_allowed(&mut self, is_back_order_allowed: bool) {
    self.is_back_order_allowed = is_back_order_allowed;
  }

  pub fn with_is_back_order_allowed(mut self, is_back_order_allowed: bool) -> OrderItem {
    self.is_back_order_allowed = is_back_order_allowed;
    self
  }

  pub fn is_back_order_allowed(&self) -> &bool {
    &self.is_back_order_allowed
  }


  pub fn set_net_cost(&mut self, net_cost: ::models::Money) {
    self.net_cost = Some(net_cost);
  }

  pub fn with_net_cost(mut self, net_cost: ::models::Money) -> OrderItem {
    self.net_cost = Some(net_cost);
    self
  }

  pub fn net_cost(&self) -> Option<&::models::Money> {
    self.net_cost.as_ref()
  }

  pub fn reset_net_cost(&mut self) {
    self.net_cost = None;
  }

  pub fn set_list_price(&mut self, list_price: ::models::Money) {
    self.list_price = Some(list_price);
  }

  pub fn with_list_price(mut self, list_price: ::models::Money) -> OrderItem {
    self.list_price = Some(list_price);
    self
  }

  pub fn list_price(&self) -> Option<&::models::Money> {
    self.list_price.as_ref()
  }

  pub fn reset_list_price(&mut self) {
    self.list_price = None;
  }

}



