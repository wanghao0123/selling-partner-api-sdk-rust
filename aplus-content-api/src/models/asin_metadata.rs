/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AsinMetadata : The A+ Content ASIN with additional metadata for content management. If you don't include the `includedDataSet` parameter in a call to the listContentDocumentAsinRelations operation, the related ASINs are returned without metadata.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsinMetadata {
  #[serde(rename = "asin")]
  asin: ::models::Asin,
  #[serde(rename = "badgeSet")]
  badge_set: Option<::models::AsinBadgeSet>,
  #[serde(rename = "parent")]
  parent: Option<::models::Asin>,
  /// The title for the ASIN in the Amazon catalog.
  #[serde(rename = "title")]
  title: Option<String>,
  /// The default image for the ASIN in the Amazon catalog.
  #[serde(rename = "imageUrl")]
  image_url: Option<String>,
  #[serde(rename = "contentReferenceKeySet")]
  content_reference_key_set: Option<::models::ContentReferenceKeySet>
}

impl AsinMetadata {
  /// The A+ Content ASIN with additional metadata for content management. If you don't include the `includedDataSet` parameter in a call to the listContentDocumentAsinRelations operation, the related ASINs are returned without metadata.
  pub fn new(asin: ::models::Asin) -> AsinMetadata {
    AsinMetadata {
      asin: asin,
      badge_set: None,
      parent: None,
      title: None,
      image_url: None,
      content_reference_key_set: None
    }
  }

  pub fn set_asin(&mut self, asin: ::models::Asin) {
    self.asin = asin;
  }

  pub fn with_asin(mut self, asin: ::models::Asin) -> AsinMetadata {
    self.asin = asin;
    self
  }

  pub fn asin(&self) -> &::models::Asin {
    &self.asin
  }


  pub fn set_badge_set(&mut self, badge_set: ::models::AsinBadgeSet) {
    self.badge_set = Some(badge_set);
  }

  pub fn with_badge_set(mut self, badge_set: ::models::AsinBadgeSet) -> AsinMetadata {
    self.badge_set = Some(badge_set);
    self
  }

  pub fn badge_set(&self) -> Option<&::models::AsinBadgeSet> {
    self.badge_set.as_ref()
  }

  pub fn reset_badge_set(&mut self) {
    self.badge_set = None;
  }

  pub fn set_parent(&mut self, parent: ::models::Asin) {
    self.parent = Some(parent);
  }

  pub fn with_parent(mut self, parent: ::models::Asin) -> AsinMetadata {
    self.parent = Some(parent);
    self
  }

  pub fn parent(&self) -> Option<&::models::Asin> {
    self.parent.as_ref()
  }

  pub fn reset_parent(&mut self) {
    self.parent = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> AsinMetadata {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_image_url(&mut self, image_url: String) {
    self.image_url = Some(image_url);
  }

  pub fn with_image_url(mut self, image_url: String) -> AsinMetadata {
    self.image_url = Some(image_url);
    self
  }

  pub fn image_url(&self) -> Option<&String> {
    self.image_url.as_ref()
  }

  pub fn reset_image_url(&mut self) {
    self.image_url = None;
  }

  pub fn set_content_reference_key_set(&mut self, content_reference_key_set: ::models::ContentReferenceKeySet) {
    self.content_reference_key_set = Some(content_reference_key_set);
  }

  pub fn with_content_reference_key_set(mut self, content_reference_key_set: ::models::ContentReferenceKeySet) -> AsinMetadata {
    self.content_reference_key_set = Some(content_reference_key_set);
    self
  }

  pub fn content_reference_key_set(&self) -> Option<&::models::ContentReferenceKeySet> {
    self.content_reference_key_set.as_ref()
  }

  pub fn reset_content_reference_key_set(&mut self) {
    self.content_reference_key_set = None;
  }

}



