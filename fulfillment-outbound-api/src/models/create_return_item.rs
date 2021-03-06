/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateReturnItem : An item that Amazon accepted for return.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReturnItem {
  /// An identifier assigned by the seller to the return item.
  #[serde(rename = "sellerReturnItemId")]
  seller_return_item_id: String,
  /// The identifier assigned to the item by the seller when the fulfillment order was created.
  #[serde(rename = "sellerFulfillmentOrderItemId")]
  seller_fulfillment_order_item_id: String,
  /// The identifier for the shipment that is associated with the return item.
  #[serde(rename = "amazonShipmentId")]
  amazon_shipment_id: String,
  /// The return reason code assigned to the return item by the seller.
  #[serde(rename = "returnReasonCode")]
  return_reason_code: String,
  /// An optional comment about the return item.
  #[serde(rename = "returnComment")]
  return_comment: Option<String>
}

impl CreateReturnItem {
  /// An item that Amazon accepted for return.
  pub fn new(seller_return_item_id: String, seller_fulfillment_order_item_id: String, amazon_shipment_id: String, return_reason_code: String) -> CreateReturnItem {
    CreateReturnItem {
      seller_return_item_id: seller_return_item_id,
      seller_fulfillment_order_item_id: seller_fulfillment_order_item_id,
      amazon_shipment_id: amazon_shipment_id,
      return_reason_code: return_reason_code,
      return_comment: None
    }
  }

  pub fn set_seller_return_item_id(&mut self, seller_return_item_id: String) {
    self.seller_return_item_id = seller_return_item_id;
  }

  pub fn with_seller_return_item_id(mut self, seller_return_item_id: String) -> CreateReturnItem {
    self.seller_return_item_id = seller_return_item_id;
    self
  }

  pub fn seller_return_item_id(&self) -> &String {
    &self.seller_return_item_id
  }


  pub fn set_seller_fulfillment_order_item_id(&mut self, seller_fulfillment_order_item_id: String) {
    self.seller_fulfillment_order_item_id = seller_fulfillment_order_item_id;
  }

  pub fn with_seller_fulfillment_order_item_id(mut self, seller_fulfillment_order_item_id: String) -> CreateReturnItem {
    self.seller_fulfillment_order_item_id = seller_fulfillment_order_item_id;
    self
  }

  pub fn seller_fulfillment_order_item_id(&self) -> &String {
    &self.seller_fulfillment_order_item_id
  }


  pub fn set_amazon_shipment_id(&mut self, amazon_shipment_id: String) {
    self.amazon_shipment_id = amazon_shipment_id;
  }

  pub fn with_amazon_shipment_id(mut self, amazon_shipment_id: String) -> CreateReturnItem {
    self.amazon_shipment_id = amazon_shipment_id;
    self
  }

  pub fn amazon_shipment_id(&self) -> &String {
    &self.amazon_shipment_id
  }


  pub fn set_return_reason_code(&mut self, return_reason_code: String) {
    self.return_reason_code = return_reason_code;
  }

  pub fn with_return_reason_code(mut self, return_reason_code: String) -> CreateReturnItem {
    self.return_reason_code = return_reason_code;
    self
  }

  pub fn return_reason_code(&self) -> &String {
    &self.return_reason_code
  }


  pub fn set_return_comment(&mut self, return_comment: String) {
    self.return_comment = Some(return_comment);
  }

  pub fn with_return_comment(mut self, return_comment: String) -> CreateReturnItem {
    self.return_comment = Some(return_comment);
    self
  }

  pub fn return_comment(&self) -> Option<&String> {
    self.return_comment.as_ref()
  }

  pub fn reset_return_comment(&mut self) {
    self.return_comment = None;
  }

}



