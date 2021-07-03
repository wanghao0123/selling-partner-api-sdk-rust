/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Label : The label details of the container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
  #[serde(rename = "labelStream")]
  label_stream: Option<::models::LabelStream>,
  #[serde(rename = "labelSpecification")]
  label_specification: Option<::models::LabelSpecification>
}

impl Label {
  /// The label details of the container.
  pub fn new() -> Label {
    Label {
      label_stream: None,
      label_specification: None
    }
  }

  pub fn set_label_stream(&mut self, label_stream: ::models::LabelStream) {
    self.label_stream = Some(label_stream);
  }

  pub fn with_label_stream(mut self, label_stream: ::models::LabelStream) -> Label {
    self.label_stream = Some(label_stream);
    self
  }

  pub fn label_stream(&self) -> Option<&::models::LabelStream> {
    self.label_stream.as_ref()
  }

  pub fn reset_label_stream(&mut self) {
    self.label_stream = None;
  }

  pub fn set_label_specification(&mut self, label_specification: ::models::LabelSpecification) {
    self.label_specification = Some(label_specification);
  }

  pub fn with_label_specification(mut self, label_specification: ::models::LabelSpecification) -> Label {
    self.label_specification = Some(label_specification);
    self
  }

  pub fn label_specification(&self) -> Option<&::models::LabelSpecification> {
    self.label_specification.as_ref()
  }

  pub fn reset_label_specification(&mut self) {
    self.label_specification = None;
  }

}



