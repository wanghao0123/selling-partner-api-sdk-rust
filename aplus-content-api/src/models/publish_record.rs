/* 
 * Selling Partner API for A+ Content Management
 *
 * With the A+ Content API, you can build applications that help selling partners add rich marketing content to their Amazon product detail pages. A+ content helps selling partners share their brand and product story, which helps buyers make informed purchasing decisions. Selling partners assemble content by choosing from content modules and adding images and text.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PublishRecord : The full context for an A+ Content publishing event.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishRecord {
  #[serde(rename = "marketplaceId")]
  marketplace_id: ::models::MarketplaceId,
  #[serde(rename = "locale")]
  locale: ::models::LanguageTag,
  #[serde(rename = "asin")]
  asin: ::models::Asin,
  #[serde(rename = "contentType")]
  content_type: ::models::ContentType,
  #[serde(rename = "contentSubType")]
  content_sub_type: Option<::models::ContentSubType>,
  #[serde(rename = "contentReferenceKey")]
  content_reference_key: ::models::ContentReferenceKey
}

impl PublishRecord {
  /// The full context for an A+ Content publishing event.
  pub fn new(marketplace_id: ::models::MarketplaceId, locale: ::models::LanguageTag, asin: ::models::Asin, content_type: ::models::ContentType, content_reference_key: ::models::ContentReferenceKey) -> PublishRecord {
    PublishRecord {
      marketplace_id: marketplace_id,
      locale: locale,
      asin: asin,
      content_type: content_type,
      content_sub_type: None,
      content_reference_key: content_reference_key
    }
  }

  pub fn set_marketplace_id(&mut self, marketplace_id: ::models::MarketplaceId) {
    self.marketplace_id = marketplace_id;
  }

  pub fn with_marketplace_id(mut self, marketplace_id: ::models::MarketplaceId) -> PublishRecord {
    self.marketplace_id = marketplace_id;
    self
  }

  pub fn marketplace_id(&self) -> &::models::MarketplaceId {
    &self.marketplace_id
  }


  pub fn set_locale(&mut self, locale: ::models::LanguageTag) {
    self.locale = locale;
  }

  pub fn with_locale(mut self, locale: ::models::LanguageTag) -> PublishRecord {
    self.locale = locale;
    self
  }

  pub fn locale(&self) -> &::models::LanguageTag {
    &self.locale
  }


  pub fn set_asin(&mut self, asin: ::models::Asin) {
    self.asin = asin;
  }

  pub fn with_asin(mut self, asin: ::models::Asin) -> PublishRecord {
    self.asin = asin;
    self
  }

  pub fn asin(&self) -> &::models::Asin {
    &self.asin
  }


  pub fn set_content_type(&mut self, content_type: ::models::ContentType) {
    self.content_type = content_type;
  }

  pub fn with_content_type(mut self, content_type: ::models::ContentType) -> PublishRecord {
    self.content_type = content_type;
    self
  }

  pub fn content_type(&self) -> &::models::ContentType {
    &self.content_type
  }


  pub fn set_content_sub_type(&mut self, content_sub_type: ::models::ContentSubType) {
    self.content_sub_type = Some(content_sub_type);
  }

  pub fn with_content_sub_type(mut self, content_sub_type: ::models::ContentSubType) -> PublishRecord {
    self.content_sub_type = Some(content_sub_type);
    self
  }

  pub fn content_sub_type(&self) -> Option<&::models::ContentSubType> {
    self.content_sub_type.as_ref()
  }

  pub fn reset_content_sub_type(&mut self) {
    self.content_sub_type = None;
  }

  pub fn set_content_reference_key(&mut self, content_reference_key: ::models::ContentReferenceKey) {
    self.content_reference_key = content_reference_key;
  }

  pub fn with_content_reference_key(mut self, content_reference_key: ::models::ContentReferenceKey) -> PublishRecord {
    self.content_reference_key = content_reference_key;
    self
  }

  pub fn content_reference_key(&self) -> &::models::ContentReferenceKey {
    &self.content_reference_key
  }


}



