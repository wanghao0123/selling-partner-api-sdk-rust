/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContentRecord : A content document with additional information for content management.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentRecord {
  #[serde(rename = "contentReferenceKey")]
  content_reference_key: ::models::ContentReferenceKey,
  #[serde(rename = "contentMetadata")]
  content_metadata: Option<::models::ContentMetadata>,
  #[serde(rename = "contentDocument")]
  content_document: Option<::models::ContentDocument>
}

impl ContentRecord {
  /// A content document with additional information for content management.
  pub fn new(content_reference_key: ::models::ContentReferenceKey) -> ContentRecord {
    ContentRecord {
      content_reference_key: content_reference_key,
      content_metadata: None,
      content_document: None
    }
  }

  pub fn set_content_reference_key(&mut self, content_reference_key: ::models::ContentReferenceKey) {
    self.content_reference_key = content_reference_key;
  }

  pub fn with_content_reference_key(mut self, content_reference_key: ::models::ContentReferenceKey) -> ContentRecord {
    self.content_reference_key = content_reference_key;
    self
  }

  pub fn content_reference_key(&self) -> &::models::ContentReferenceKey {
    &self.content_reference_key
  }


  pub fn set_content_metadata(&mut self, content_metadata: ::models::ContentMetadata) {
    self.content_metadata = Some(content_metadata);
  }

  pub fn with_content_metadata(mut self, content_metadata: ::models::ContentMetadata) -> ContentRecord {
    self.content_metadata = Some(content_metadata);
    self
  }

  pub fn content_metadata(&self) -> Option<&::models::ContentMetadata> {
    self.content_metadata.as_ref()
  }

  pub fn reset_content_metadata(&mut self) {
    self.content_metadata = None;
  }

  pub fn set_content_document(&mut self, content_document: ::models::ContentDocument) {
    self.content_document = Some(content_document);
  }

  pub fn with_content_document(mut self, content_document: ::models::ContentDocument) -> ContentRecord {
    self.content_document = Some(content_document);
    self
  }

  pub fn content_document(&self) -> Option<&::models::ContentDocument> {
    self.content_document.as_ref()
  }

  pub fn reset_content_document(&mut self) {
    self.content_document = None;
  }

}



