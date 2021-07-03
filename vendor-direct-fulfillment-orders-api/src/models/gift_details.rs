/* 
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GiftDetails : Gift details for the item.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftDetails {
  /// Gift message to be printed in shipment.
  #[serde(rename = "giftMessage")]
  gift_message: Option<String>,
  /// Gift wrap identifier for the gift wrapping, if any.
  #[serde(rename = "giftWrapId")]
  gift_wrap_id: Option<String>
}

impl GiftDetails {
  /// Gift details for the item.
  pub fn new() -> GiftDetails {
    GiftDetails {
      gift_message: None,
      gift_wrap_id: None
    }
  }

  pub fn set_gift_message(&mut self, gift_message: String) {
    self.gift_message = Some(gift_message);
  }

  pub fn with_gift_message(mut self, gift_message: String) -> GiftDetails {
    self.gift_message = Some(gift_message);
    self
  }

  pub fn gift_message(&self) -> Option<&String> {
    self.gift_message.as_ref()
  }

  pub fn reset_gift_message(&mut self) {
    self.gift_message = None;
  }

  pub fn set_gift_wrap_id(&mut self, gift_wrap_id: String) {
    self.gift_wrap_id = Some(gift_wrap_id);
  }

  pub fn with_gift_wrap_id(mut self, gift_wrap_id: String) -> GiftDetails {
    self.gift_wrap_id = Some(gift_wrap_id);
    self
  }

  pub fn gift_wrap_id(&self) -> Option<&String> {
    self.gift_wrap_id.as_ref()
  }

  pub fn reset_gift_wrap_id(&mut self) {
    self.gift_wrap_id = None;
  }

}



