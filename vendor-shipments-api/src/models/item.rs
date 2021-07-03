/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Item : Details of the item being shipped.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
  /// Item sequence number for the item. The first item will be 001, the second 002, and so on. This number is used as a reference to refer to this item from the carton or pallet level.
  #[serde(rename = "itemSequenceNumber")]
  item_sequence_number: String,
  /// Amazon Standard Identification Number (ASIN) of an item.
  #[serde(rename = "amazonProductIdentifier")]
  amazon_product_identifier: Option<String>,
  /// The vendor selected product identification of the item. Should be the same as was sent in the purchase order.
  #[serde(rename = "vendorProductIdentifier")]
  vendor_product_identifier: Option<String>,
  /// Total item quantity shipped in this shipment.
  #[serde(rename = "shippedQuantity")]
  shipped_quantity: ::models::ItemQuantity,
  #[serde(rename = "itemDetails")]
  item_details: Option<::models::ItemDetails>
}

impl Item {
  /// Details of the item being shipped.
  pub fn new(item_sequence_number: String, shipped_quantity: ::models::ItemQuantity) -> Item {
    Item {
      item_sequence_number: item_sequence_number,
      amazon_product_identifier: None,
      vendor_product_identifier: None,
      shipped_quantity: shipped_quantity,
      item_details: None
    }
  }

  pub fn set_item_sequence_number(&mut self, item_sequence_number: String) {
    self.item_sequence_number = item_sequence_number;
  }

  pub fn with_item_sequence_number(mut self, item_sequence_number: String) -> Item {
    self.item_sequence_number = item_sequence_number;
    self
  }

  pub fn item_sequence_number(&self) -> &String {
    &self.item_sequence_number
  }


  pub fn set_amazon_product_identifier(&mut self, amazon_product_identifier: String) {
    self.amazon_product_identifier = Some(amazon_product_identifier);
  }

  pub fn with_amazon_product_identifier(mut self, amazon_product_identifier: String) -> Item {
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

  pub fn with_vendor_product_identifier(mut self, vendor_product_identifier: String) -> Item {
    self.vendor_product_identifier = Some(vendor_product_identifier);
    self
  }

  pub fn vendor_product_identifier(&self) -> Option<&String> {
    self.vendor_product_identifier.as_ref()
  }

  pub fn reset_vendor_product_identifier(&mut self) {
    self.vendor_product_identifier = None;
  }

  pub fn set_shipped_quantity(&mut self, shipped_quantity: ::models::ItemQuantity) {
    self.shipped_quantity = shipped_quantity;
  }

  pub fn with_shipped_quantity(mut self, shipped_quantity: ::models::ItemQuantity) -> Item {
    self.shipped_quantity = shipped_quantity;
    self
  }

  pub fn shipped_quantity(&self) -> &::models::ItemQuantity {
    &self.shipped_quantity
  }


  pub fn set_item_details(&mut self, item_details: ::models::ItemDetails) {
    self.item_details = Some(item_details);
  }

  pub fn with_item_details(mut self, item_details: ::models::ItemDetails) -> Item {
    self.item_details = Some(item_details);
    self
  }

  pub fn item_details(&self) -> Option<&::models::ItemDetails> {
    self.item_details.as_ref()
  }

  pub fn reset_item_details(&mut self) {
    self.item_details = None;
  }

}



