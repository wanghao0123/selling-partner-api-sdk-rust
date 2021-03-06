/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidSku {
  /// The seller SKU of the item.
  #[serde(rename = "SellerSKU")]
  seller_sku: Option<String>,
  /// The reason why the seller SKU is invalid.
  #[serde(rename = "ErrorReason")]
  error_reason: Option<::models::ErrorReason>
}

impl InvalidSku {
  pub fn new() -> InvalidSku {
    InvalidSku {
      seller_sku: None,
      error_reason: None
    }
  }

  pub fn set_seller_sku(&mut self, seller_sku: String) {
    self.seller_sku = Some(seller_sku);
  }

  pub fn with_seller_sku(mut self, seller_sku: String) -> InvalidSku {
    self.seller_sku = Some(seller_sku);
    self
  }

  pub fn seller_sku(&self) -> Option<&String> {
    self.seller_sku.as_ref()
  }

  pub fn reset_seller_sku(&mut self) {
    self.seller_sku = None;
  }

  pub fn set_error_reason(&mut self, error_reason: ::models::ErrorReason) {
    self.error_reason = Some(error_reason);
  }

  pub fn with_error_reason(mut self, error_reason: ::models::ErrorReason) -> InvalidSku {
    self.error_reason = Some(error_reason);
    self
  }

  pub fn error_reason(&self) -> Option<&::models::ErrorReason> {
    self.error_reason.as_ref()
  }

  pub fn reset_error_reason(&mut self) {
    self.error_reason = None;
  }

}



