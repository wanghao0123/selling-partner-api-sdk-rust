/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Container : Container in the shipment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
  /// The type of physical container being used. (always 'PACKAGE')
  #[serde(rename = "containerType")]
  container_type: Option<String>,
  #[serde(rename = "containerReferenceId")]
  container_reference_id: ::models::ContainerReferenceId,
  /// The total value of all items in the container.
  #[serde(rename = "value")]
  value: ::models::Currency,
  /// The length, width, height, and weight of the container.
  #[serde(rename = "dimensions")]
  dimensions: ::models::Dimensions,
  /// A list of the items in the container.
  #[serde(rename = "items")]
  items: Vec<::models::ContainerItem>,
  /// The weight of the container.
  #[serde(rename = "weight")]
  weight: ::models::Weight
}

impl Container {
  /// Container in the shipment.
  pub fn new(container_reference_id: ::models::ContainerReferenceId, value: ::models::Currency, dimensions: ::models::Dimensions, items: Vec<::models::ContainerItem>, weight: ::models::Weight) -> Container {
    Container {
      container_type: None,
      container_reference_id: container_reference_id,
      value: value,
      dimensions: dimensions,
      items: items,
      weight: weight
    }
  }

  pub fn set_container_type(&mut self, container_type: String) {
    self.container_type = Some(container_type);
  }

  pub fn with_container_type(mut self, container_type: String) -> Container {
    self.container_type = Some(container_type);
    self
  }

  pub fn container_type(&self) -> Option<&String> {
    self.container_type.as_ref()
  }

  pub fn reset_container_type(&mut self) {
    self.container_type = None;
  }

  pub fn set_container_reference_id(&mut self, container_reference_id: ::models::ContainerReferenceId) {
    self.container_reference_id = container_reference_id;
  }

  pub fn with_container_reference_id(mut self, container_reference_id: ::models::ContainerReferenceId) -> Container {
    self.container_reference_id = container_reference_id;
    self
  }

  pub fn container_reference_id(&self) -> &::models::ContainerReferenceId {
    &self.container_reference_id
  }


  pub fn set_value(&mut self, value: ::models::Currency) {
    self.value = value;
  }

  pub fn with_value(mut self, value: ::models::Currency) -> Container {
    self.value = value;
    self
  }

  pub fn value(&self) -> &::models::Currency {
    &self.value
  }


  pub fn set_dimensions(&mut self, dimensions: ::models::Dimensions) {
    self.dimensions = dimensions;
  }

  pub fn with_dimensions(mut self, dimensions: ::models::Dimensions) -> Container {
    self.dimensions = dimensions;
    self
  }

  pub fn dimensions(&self) -> &::models::Dimensions {
    &self.dimensions
  }


  pub fn set_items(&mut self, items: Vec<::models::ContainerItem>) {
    self.items = items;
  }

  pub fn with_items(mut self, items: Vec<::models::ContainerItem>) -> Container {
    self.items = items;
    self
  }

  pub fn items(&self) -> &Vec<::models::ContainerItem> {
    &self.items
  }


  pub fn set_weight(&mut self, weight: ::models::Weight) {
    self.weight = weight;
  }

  pub fn with_weight(mut self, weight: ::models::Weight) -> Container {
    self.weight = weight;
    self
  }

  pub fn weight(&self) -> &::models::Weight {
    &self.weight
  }


}



