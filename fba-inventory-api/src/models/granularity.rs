/* 
 * Selling Partner API for FBA Inventory
 *
 * The Selling Partner API for FBA Inventory lets you programmatically retrieve information about inventory in Amazon's fulfillment network. Today this API is available only in the North America region. In 2021 we plan to release this API in the Europe and Far East regions.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Granularity : Describes a granularity at which inventory data can be aggregated. For example, if you use Marketplace granularity, the fulfillable quantity will reflect inventory that could be fulfilled in the given marketplace.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Granularity {
  /// The granularity type for the inventory aggregation level.
  #[serde(rename = "granularityType")]
  granularity_type: Option<String>,
  /// The granularity ID for the specified granularity type. When granularityType is Marketplace, specify the marketplaceId.
  #[serde(rename = "granularityId")]
  granularity_id: Option<String>
}

impl Granularity {
  /// Describes a granularity at which inventory data can be aggregated. For example, if you use Marketplace granularity, the fulfillable quantity will reflect inventory that could be fulfilled in the given marketplace.
  pub fn new() -> Granularity {
    Granularity {
      granularity_type: None,
      granularity_id: None
    }
  }

  pub fn set_granularity_type(&mut self, granularity_type: String) {
    self.granularity_type = Some(granularity_type);
  }

  pub fn with_granularity_type(mut self, granularity_type: String) -> Granularity {
    self.granularity_type = Some(granularity_type);
    self
  }

  pub fn granularity_type(&self) -> Option<&String> {
    self.granularity_type.as_ref()
  }

  pub fn reset_granularity_type(&mut self) {
    self.granularity_type = None;
  }

  pub fn set_granularity_id(&mut self, granularity_id: String) {
    self.granularity_id = Some(granularity_id);
  }

  pub fn with_granularity_id(mut self, granularity_id: String) -> Granularity {
    self.granularity_id = Some(granularity_id);
    self
  }

  pub fn granularity_id(&self) -> Option<&String> {
    self.granularity_id.as_ref()
  }

  pub fn reset_granularity_id(&mut self) {
    self.granularity_id = None;
  }

}



