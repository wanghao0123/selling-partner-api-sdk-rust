/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Decorator : A decorator applied to a content string value in order to create rich text.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Decorator {
  #[serde(rename = "type")]
  _type: Option<::models::DecoratorType>,
  /// The starting character of this decorator within the content string. Use zero for the first character.
  #[serde(rename = "offset")]
  offset: Option<i32>,
  /// The number of content characters to alter with this decorator. Decorators such as line breaks can have zero length and fit between characters.
  #[serde(rename = "length")]
  length: Option<i32>,
  /// The relative intensity or variation of this decorator. Decorators such as bullet-points, for example, can have multiple indentation depths.
  #[serde(rename = "depth")]
  depth: Option<i32>
}

impl Decorator {
  /// A decorator applied to a content string value in order to create rich text.
  pub fn new() -> Decorator {
    Decorator {
      _type: None,
      offset: None,
      length: None,
      depth: None
    }
  }

  pub fn set__type(&mut self, _type: ::models::DecoratorType) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: ::models::DecoratorType) -> Decorator {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&::models::DecoratorType> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_offset(&mut self, offset: i32) {
    self.offset = Some(offset);
  }

  pub fn with_offset(mut self, offset: i32) -> Decorator {
    self.offset = Some(offset);
    self
  }

  pub fn offset(&self) -> Option<&i32> {
    self.offset.as_ref()
  }

  pub fn reset_offset(&mut self) {
    self.offset = None;
  }

  pub fn set_length(&mut self, length: i32) {
    self.length = Some(length);
  }

  pub fn with_length(mut self, length: i32) -> Decorator {
    self.length = Some(length);
    self
  }

  pub fn length(&self) -> Option<&i32> {
    self.length.as_ref()
  }

  pub fn reset_length(&mut self) {
    self.length = None;
  }

  pub fn set_depth(&mut self, depth: i32) {
    self.depth = Some(depth);
  }

  pub fn with_depth(mut self, depth: i32) -> Decorator {
    self.depth = Some(depth);
    self
  }

  pub fn depth(&self) -> Option<&i32> {
    self.depth.as_ref()
  }

  pub fn reset_depth(&mut self) {
    self.depth = None;
  }

}



