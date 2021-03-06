/* 
 * Selling Partner API for Finances
 *
 * The Selling Partner API for Finances helps you obtain financial information relevant to a seller's business. You can obtain financial events for a given order, financial event group, or date range without having to wait until a statement period closes. You can also obtain financial event groups for a given date range.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ServiceFeeEvent : A service fee on the seller's account.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceFeeEvent {
  /// An Amazon-defined identifier for an order.
  #[serde(rename = "AmazonOrderId")]
  amazon_order_id: Option<String>,
  /// A short description of the service fee reason.
  #[serde(rename = "FeeReason")]
  fee_reason: Option<String>,
  /// A list of fee components associated with the service fee.
  #[serde(rename = "FeeList")]
  fee_list: Option<::models::FeeComponentList>,
  /// The seller SKU of the item. The seller SKU is qualified by the seller's seller ID, which is included with every call to the Selling Partner API.
  #[serde(rename = "SellerSKU")]
  seller_sku: Option<String>,
  /// A unique identifier assigned by Amazon to products stored in and fulfilled from an Amazon fulfillment center.
  #[serde(rename = "FnSKU")]
  fn_sku: Option<String>,
  /// A short description of the service fee event.
  #[serde(rename = "FeeDescription")]
  fee_description: Option<String>,
  /// The Amazon Standard Identification Number (ASIN) of the item.
  #[serde(rename = "ASIN")]
  ASIN: Option<String>
}

impl ServiceFeeEvent {
  /// A service fee on the seller's account.
  pub fn new() -> ServiceFeeEvent {
    ServiceFeeEvent {
      amazon_order_id: None,
      fee_reason: None,
      fee_list: None,
      seller_sku: None,
      fn_sku: None,
      fee_description: None,
      ASIN: None
    }
  }

  pub fn set_amazon_order_id(&mut self, amazon_order_id: String) {
    self.amazon_order_id = Some(amazon_order_id);
  }

  pub fn with_amazon_order_id(mut self, amazon_order_id: String) -> ServiceFeeEvent {
    self.amazon_order_id = Some(amazon_order_id);
    self
  }

  pub fn amazon_order_id(&self) -> Option<&String> {
    self.amazon_order_id.as_ref()
  }

  pub fn reset_amazon_order_id(&mut self) {
    self.amazon_order_id = None;
  }

  pub fn set_fee_reason(&mut self, fee_reason: String) {
    self.fee_reason = Some(fee_reason);
  }

  pub fn with_fee_reason(mut self, fee_reason: String) -> ServiceFeeEvent {
    self.fee_reason = Some(fee_reason);
    self
  }

  pub fn fee_reason(&self) -> Option<&String> {
    self.fee_reason.as_ref()
  }

  pub fn reset_fee_reason(&mut self) {
    self.fee_reason = None;
  }

  pub fn set_fee_list(&mut self, fee_list: ::models::FeeComponentList) {
    self.fee_list = Some(fee_list);
  }

  pub fn with_fee_list(mut self, fee_list: ::models::FeeComponentList) -> ServiceFeeEvent {
    self.fee_list = Some(fee_list);
    self
  }

  pub fn fee_list(&self) -> Option<&::models::FeeComponentList> {
    self.fee_list.as_ref()
  }

  pub fn reset_fee_list(&mut self) {
    self.fee_list = None;
  }

  pub fn set_seller_sku(&mut self, seller_sku: String) {
    self.seller_sku = Some(seller_sku);
  }

  pub fn with_seller_sku(mut self, seller_sku: String) -> ServiceFeeEvent {
    self.seller_sku = Some(seller_sku);
    self
  }

  pub fn seller_sku(&self) -> Option<&String> {
    self.seller_sku.as_ref()
  }

  pub fn reset_seller_sku(&mut self) {
    self.seller_sku = None;
  }

  pub fn set_fn_sku(&mut self, fn_sku: String) {
    self.fn_sku = Some(fn_sku);
  }

  pub fn with_fn_sku(mut self, fn_sku: String) -> ServiceFeeEvent {
    self.fn_sku = Some(fn_sku);
    self
  }

  pub fn fn_sku(&self) -> Option<&String> {
    self.fn_sku.as_ref()
  }

  pub fn reset_fn_sku(&mut self) {
    self.fn_sku = None;
  }

  pub fn set_fee_description(&mut self, fee_description: String) {
    self.fee_description = Some(fee_description);
  }

  pub fn with_fee_description(mut self, fee_description: String) -> ServiceFeeEvent {
    self.fee_description = Some(fee_description);
    self
  }

  pub fn fee_description(&self) -> Option<&String> {
    self.fee_description.as_ref()
  }

  pub fn reset_fee_description(&mut self) {
    self.fee_description = None;
  }

  pub fn set_ASIN(&mut self, ASIN: String) {
    self.ASIN = Some(ASIN);
  }

  pub fn with_ASIN(mut self, ASIN: String) -> ServiceFeeEvent {
    self.ASIN = Some(ASIN);
    self
  }

  pub fn ASIN(&self) -> Option<&String> {
    self.ASIN.as_ref()
  }

  pub fn reset_ASIN(&mut self) {
    self.ASIN = None;
  }

}



