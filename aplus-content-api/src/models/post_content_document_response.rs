/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostContentDocumentResponse {
  #[serde(rename = "warnings")]
  warnings: Option<::models::MessageSet>,
  #[serde(rename = "contentReferenceKey")]
  content_reference_key: ::models::ContentReferenceKey
}

impl PostContentDocumentResponse {
  pub fn new(content_reference_key: ::models::ContentReferenceKey) -> PostContentDocumentResponse {
    PostContentDocumentResponse {
      warnings: None,
      content_reference_key: content_reference_key
    }
  }

  pub fn set_warnings(&mut self, warnings: ::models::MessageSet) {
    self.warnings = Some(warnings);
  }

  pub fn with_warnings(mut self, warnings: ::models::MessageSet) -> PostContentDocumentResponse {
    self.warnings = Some(warnings);
    self
  }

  pub fn warnings(&self) -> Option<&::models::MessageSet> {
    self.warnings.as_ref()
  }

  pub fn reset_warnings(&mut self) {
    self.warnings = None;
  }

  pub fn set_content_reference_key(&mut self, content_reference_key: ::models::ContentReferenceKey) {
    self.content_reference_key = content_reference_key;
  }

  pub fn with_content_reference_key(mut self, content_reference_key: ::models::ContentReferenceKey) -> PostContentDocumentResponse {
    self.content_reference_key = content_reference_key;
    self
  }

  pub fn content_reference_key(&self) -> &::models::ContentReferenceKey {
    &self.content_reference_key
  }


}



