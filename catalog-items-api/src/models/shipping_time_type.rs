/* 
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingTimeType {
  /// (0-2 days, 3-7 days, 8-13 days, or 14 or more days) – Indicates the maximum time within which the item will likely be shipped once an order has been placed.
  #[serde(rename = "Max")]
  max: Option<String>
}

impl ShippingTimeType {
  pub fn new() -> ShippingTimeType {
    ShippingTimeType {
      max: None
    }
  }

  pub fn set_max(&mut self, max: String) {
    self.max = Some(max);
  }

  pub fn with_max(mut self, max: String) -> ShippingTimeType {
    self.max = Some(max);
    self
  }

  pub fn max(&self) -> Option<&String> {
    self.max.as_ref()
  }

  pub fn reset_max(&mut self) {
    self.max = None;
  }

}



