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
pub struct PackedItem {
  /// Item Sequence Number for the item. This must be the same value as sent in the order for a given item.
  #[serde(rename = "itemSequenceNumber")]
  item_sequence_number: i32,
  /// Buyer's Standard Identification Number (ASIN) of an item. Either buyerProductIdentifier or vendorProductIdentifier is required.
  #[serde(rename = "buyerProductIdentifier")]
  buyer_product_identifier: Option<String>,
  /// The vendor selected product identification of the item. Should be the same as was sent in the Purchase Order, like SKU Number.
  #[serde(rename = "vendorProductIdentifier")]
  vendor_product_identifier: Option<String>,
  /// Total item quantity packed in the container.
  #[serde(rename = "packedQuantity")]
  packed_quantity: ::models::ItemQuantity
}

impl PackedItem {
  pub fn new(item_sequence_number: i32, packed_quantity: ::models::ItemQuantity) -> PackedItem {
    PackedItem {
      item_sequence_number: item_sequence_number,
      buyer_product_identifier: None,
      vendor_product_identifier: None,
      packed_quantity: packed_quantity
    }
  }

  pub fn set_item_sequence_number(&mut self, item_sequence_number: i32) {
    self.item_sequence_number = item_sequence_number;
  }

  pub fn with_item_sequence_number(mut self, item_sequence_number: i32) -> PackedItem {
    self.item_sequence_number = item_sequence_number;
    self
  }

  pub fn item_sequence_number(&self) -> &i32 {
    &self.item_sequence_number
  }


  pub fn set_buyer_product_identifier(&mut self, buyer_product_identifier: String) {
    self.buyer_product_identifier = Some(buyer_product_identifier);
  }

  pub fn with_buyer_product_identifier(mut self, buyer_product_identifier: String) -> PackedItem {
    self.buyer_product_identifier = Some(buyer_product_identifier);
    self
  }

  pub fn buyer_product_identifier(&self) -> Option<&String> {
    self.buyer_product_identifier.as_ref()
  }

  pub fn reset_buyer_product_identifier(&mut self) {
    self.buyer_product_identifier = None;
  }

  pub fn set_vendor_product_identifier(&mut self, vendor_product_identifier: String) {
    self.vendor_product_identifier = Some(vendor_product_identifier);
  }

  pub fn with_vendor_product_identifier(mut self, vendor_product_identifier: String) -> PackedItem {
    self.vendor_product_identifier = Some(vendor_product_identifier);
    self
  }

  pub fn vendor_product_identifier(&self) -> Option<&String> {
    self.vendor_product_identifier.as_ref()
  }

  pub fn reset_vendor_product_identifier(&mut self) {
    self.vendor_product_identifier = None;
  }

  pub fn set_packed_quantity(&mut self, packed_quantity: ::models::ItemQuantity) {
    self.packed_quantity = packed_quantity;
  }

  pub fn with_packed_quantity(mut self, packed_quantity: ::models::ItemQuantity) -> PackedItem {
    self.packed_quantity = packed_quantity;
    self
  }

  pub fn packed_quantity(&self) -> &::models::ItemQuantity {
    &self.packed_quantity
  }


}



