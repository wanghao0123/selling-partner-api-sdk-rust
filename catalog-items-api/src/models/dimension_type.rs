/* 
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DimensionType : The dimension type attribute of an item.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionType {
  /// The height attribute of the dimension.
  #[serde(rename = "Height")]
  height: Option<::models::DecimalWithUnits>,
  /// The length attribute of the dimension.
  #[serde(rename = "Length")]
  length: Option<::models::DecimalWithUnits>,
  /// The width attribute of the dimension.
  #[serde(rename = "Width")]
  width: Option<::models::DecimalWithUnits>,
  /// The weight attribute of the dimension.
  #[serde(rename = "Weight")]
  weight: Option<::models::DecimalWithUnits>
}

impl DimensionType {
  /// The dimension type attribute of an item.
  pub fn new() -> DimensionType {
    DimensionType {
      height: None,
      length: None,
      width: None,
      weight: None
    }
  }

  pub fn set_height(&mut self, height: ::models::DecimalWithUnits) {
    self.height = Some(height);
  }

  pub fn with_height(mut self, height: ::models::DecimalWithUnits) -> DimensionType {
    self.height = Some(height);
    self
  }

  pub fn height(&self) -> Option<&::models::DecimalWithUnits> {
    self.height.as_ref()
  }

  pub fn reset_height(&mut self) {
    self.height = None;
  }

  pub fn set_length(&mut self, length: ::models::DecimalWithUnits) {
    self.length = Some(length);
  }

  pub fn with_length(mut self, length: ::models::DecimalWithUnits) -> DimensionType {
    self.length = Some(length);
    self
  }

  pub fn length(&self) -> Option<&::models::DecimalWithUnits> {
    self.length.as_ref()
  }

  pub fn reset_length(&mut self) {
    self.length = None;
  }

  pub fn set_width(&mut self, width: ::models::DecimalWithUnits) {
    self.width = Some(width);
  }

  pub fn with_width(mut self, width: ::models::DecimalWithUnits) -> DimensionType {
    self.width = Some(width);
    self
  }

  pub fn width(&self) -> Option<&::models::DecimalWithUnits> {
    self.width.as_ref()
  }

  pub fn reset_width(&mut self) {
    self.width = None;
  }

  pub fn set_weight(&mut self, weight: ::models::DecimalWithUnits) {
    self.weight = Some(weight);
  }

  pub fn with_weight(mut self, weight: ::models::DecimalWithUnits) -> DimensionType {
    self.weight = Some(weight);
    self
  }

  pub fn weight(&self) -> Option<&::models::DecimalWithUnits> {
    self.weight.as_ref()
  }

  pub fn reset_weight(&mut self) {
    self.weight = None;
  }

}



