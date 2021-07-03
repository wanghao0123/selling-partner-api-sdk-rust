/* 
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// OrderAcknowledgementItem : Details of the item being acknowledged.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAcknowledgementItem {
  /// Line item sequence number for the item.
  #[serde(rename = "itemSequenceNumber")]
  item_sequence_number: Option<String>,
  /// Amazon Standard Identification Number (ASIN) of an item.
  #[serde(rename = "amazonProductIdentifier")]
  amazon_product_identifier: Option<String>,
  /// The vendor selected product identification of the item. Should be the same as was sent in the purchase order.
  #[serde(rename = "vendorProductIdentifier")]
  vendor_product_identifier: Option<String>,
  /// The quantity of this item ordered.
  #[serde(rename = "orderedQuantity")]
  ordered_quantity: ::models::ItemQuantity,
  /// The cost to Amazon, which should match the cost on the invoice. This is a required field. If this is left blank the file will reject in Amazon systems. Price information should not be zero or negative. Indicates a net unit price.
  #[serde(rename = "netCost")]
  net_cost: ::models::Money,
  /// The list price. This is required only if a vendor sells books with a list price.
  #[serde(rename = "listPrice")]
  list_price: Option<::models::Money>,
  /// The discount multiplier that should be applied to the price if a vendor sells books with a list price. This is a multiplier factor to arrive at a final discounted price. A multiplier of .90 would be the factor if a 10% discount is given.
  #[serde(rename = "discountMultiplier")]
  discount_multiplier: Option<String>,
  /// This is used to indicate acknowledged quantity.
  #[serde(rename = "itemAcknowledgements")]
  item_acknowledgements: Vec<::models::OrderItemAcknowledgement>
}

impl OrderAcknowledgementItem {
  /// Details of the item being acknowledged.
  pub fn new(ordered_quantity: ::models::ItemQuantity, net_cost: ::models::Money, item_acknowledgements: Vec<::models::OrderItemAcknowledgement>) -> OrderAcknowledgementItem {
    OrderAcknowledgementItem {
      item_sequence_number: None,
      amazon_product_identifier: None,
      vendor_product_identifier: None,
      ordered_quantity: ordered_quantity,
      net_cost: net_cost,
      list_price: None,
      discount_multiplier: None,
      item_acknowledgements: item_acknowledgements
    }
  }

  pub fn set_item_sequence_number(&mut self, item_sequence_number: String) {
    self.item_sequence_number = Some(item_sequence_number);
  }

  pub fn with_item_sequence_number(mut self, item_sequence_number: String) -> OrderAcknowledgementItem {
    self.item_sequence_number = Some(item_sequence_number);
    self
  }

  pub fn item_sequence_number(&self) -> Option<&String> {
    self.item_sequence_number.as_ref()
  }

  pub fn reset_item_sequence_number(&mut self) {
    self.item_sequence_number = None;
  }

  pub fn set_amazon_product_identifier(&mut self, amazon_product_identifier: String) {
    self.amazon_product_identifier = Some(amazon_product_identifier);
  }

  pub fn with_amazon_product_identifier(mut self, amazon_product_identifier: String) -> OrderAcknowledgementItem {
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

  pub fn with_vendor_product_identifier(mut self, vendor_product_identifier: String) -> OrderAcknowledgementItem {
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

  pub fn with_ordered_quantity(mut self, ordered_quantity: ::models::ItemQuantity) -> OrderAcknowledgementItem {
    self.ordered_quantity = ordered_quantity;
    self
  }

  pub fn ordered_quantity(&self) -> &::models::ItemQuantity {
    &self.ordered_quantity
  }


  pub fn set_net_cost(&mut self, net_cost: ::models::Money) {
    self.net_cost = net_cost;
  }

  pub fn with_net_cost(mut self, net_cost: ::models::Money) -> OrderAcknowledgementItem {
    self.net_cost = net_cost;
    self
  }

  pub fn net_cost(&self) -> &::models::Money {
    &self.net_cost
  }


  pub fn set_list_price(&mut self, list_price: ::models::Money) {
    self.list_price = Some(list_price);
  }

  pub fn with_list_price(mut self, list_price: ::models::Money) -> OrderAcknowledgementItem {
    self.list_price = Some(list_price);
    self
  }

  pub fn list_price(&self) -> Option<&::models::Money> {
    self.list_price.as_ref()
  }

  pub fn reset_list_price(&mut self) {
    self.list_price = None;
  }

  pub fn set_discount_multiplier(&mut self, discount_multiplier: String) {
    self.discount_multiplier = Some(discount_multiplier);
  }

  pub fn with_discount_multiplier(mut self, discount_multiplier: String) -> OrderAcknowledgementItem {
    self.discount_multiplier = Some(discount_multiplier);
    self
  }

  pub fn discount_multiplier(&self) -> Option<&String> {
    self.discount_multiplier.as_ref()
  }

  pub fn reset_discount_multiplier(&mut self) {
    self.discount_multiplier = None;
  }

  pub fn set_item_acknowledgements(&mut self, item_acknowledgements: Vec<::models::OrderItemAcknowledgement>) {
    self.item_acknowledgements = item_acknowledgements;
  }

  pub fn with_item_acknowledgements(mut self, item_acknowledgements: Vec<::models::OrderItemAcknowledgement>) -> OrderAcknowledgementItem {
    self.item_acknowledgements = item_acknowledgements;
    self
  }

  pub fn item_acknowledgements(&self) -> &Vec<::models::OrderItemAcknowledgement> {
    &self.item_acknowledgements
  }


}



