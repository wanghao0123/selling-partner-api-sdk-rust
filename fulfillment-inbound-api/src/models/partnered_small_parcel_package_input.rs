/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartneredSmallParcelPackageInput : Dimension and weight information for the package.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartneredSmallParcelPackageInput {
  #[serde(rename = "Dimensions")]
  dimensions: ::models::Dimensions,
  #[serde(rename = "Weight")]
  weight: ::models::Weight
}

impl PartneredSmallParcelPackageInput {
  /// Dimension and weight information for the package.
  pub fn new(dimensions: ::models::Dimensions, weight: ::models::Weight) -> PartneredSmallParcelPackageInput {
    PartneredSmallParcelPackageInput {
      dimensions: dimensions,
      weight: weight
    }
  }

  pub fn set_dimensions(&mut self, dimensions: ::models::Dimensions) {
    self.dimensions = dimensions;
  }

  pub fn with_dimensions(mut self, dimensions: ::models::Dimensions) -> PartneredSmallParcelPackageInput {
    self.dimensions = dimensions;
    self
  }

  pub fn dimensions(&self) -> &::models::Dimensions {
    &self.dimensions
  }


  pub fn set_weight(&mut self, weight: ::models::Weight) {
    self.weight = weight;
  }

  pub fn with_weight(mut self, weight: ::models::Weight) -> PartneredSmallParcelPackageInput {
    self.weight = weight;
    self
  }

  pub fn weight(&self) -> &::models::Weight {
    &self.weight
  }


}



