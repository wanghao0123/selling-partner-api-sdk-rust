/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LabelResult : Label details including label stream, format, size.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelResult {
  #[serde(rename = "containerReferenceId")]
  container_reference_id: Option<::models::ContainerReferenceId>,
  /// The tracking identifier assigned to the container.
  #[serde(rename = "trackingId")]
  tracking_id: Option<String>,
  #[serde(rename = "label")]
  label: Option<::models::Label>
}

impl LabelResult {
  /// Label details including label stream, format, size.
  pub fn new() -> LabelResult {
    LabelResult {
      container_reference_id: None,
      tracking_id: None,
      label: None
    }
  }

  pub fn set_container_reference_id(&mut self, container_reference_id: ::models::ContainerReferenceId) {
    self.container_reference_id = Some(container_reference_id);
  }

  pub fn with_container_reference_id(mut self, container_reference_id: ::models::ContainerReferenceId) -> LabelResult {
    self.container_reference_id = Some(container_reference_id);
    self
  }

  pub fn container_reference_id(&self) -> Option<&::models::ContainerReferenceId> {
    self.container_reference_id.as_ref()
  }

  pub fn reset_container_reference_id(&mut self) {
    self.container_reference_id = None;
  }

  pub fn set_tracking_id(&mut self, tracking_id: String) {
    self.tracking_id = Some(tracking_id);
  }

  pub fn with_tracking_id(mut self, tracking_id: String) -> LabelResult {
    self.tracking_id = Some(tracking_id);
    self
  }

  pub fn tracking_id(&self) -> Option<&String> {
    self.tracking_id.as_ref()
  }

  pub fn reset_tracking_id(&mut self) {
    self.tracking_id = None;
  }

  pub fn set_label(&mut self, label: ::models::Label) {
    self.label = Some(label);
  }

  pub fn with_label(mut self, label: ::models::Label) -> LabelResult {
    self.label = Some(label);
    self
  }

  pub fn label(&self) -> Option<&::models::Label> {
    self.label.as_ref()
  }

  pub fn reset_label(&mut self) {
    self.label = None;
  }

}



