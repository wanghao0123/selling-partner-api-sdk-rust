/* 
 * Selling Partner API for Feeds
 *
 * The Selling Partner API for Feeds lets you upload data to Amazon on behalf of a selling partner.
 *
 * OpenAPI spec version: 2021-06-30
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateFeedDocumentResponse : Information required to upload a feed document's contents.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFeedDocumentResponse {
  /// The identifier of the feed document.
  #[serde(rename = "feedDocumentId")]
  feed_document_id: String,
  /// The presigned URL for uploading the feed contents. This URL expires after 5 minutes.
  #[serde(rename = "url")]
  url: String
}

impl CreateFeedDocumentResponse {
  /// Information required to upload a feed document's contents.
  pub fn new(feed_document_id: String, url: String) -> CreateFeedDocumentResponse {
    CreateFeedDocumentResponse {
      feed_document_id: feed_document_id,
      url: url
    }
  }

  pub fn set_feed_document_id(&mut self, feed_document_id: String) {
    self.feed_document_id = feed_document_id;
  }

  pub fn with_feed_document_id(mut self, feed_document_id: String) -> CreateFeedDocumentResponse {
    self.feed_document_id = feed_document_id;
    self
  }

  pub fn feed_document_id(&self) -> &String {
    &self.feed_document_id
  }


  pub fn set_url(&mut self, url: String) {
    self.url = url;
  }

  pub fn with_url(mut self, url: String) -> CreateFeedDocumentResponse {
    self.url = url;
    self
  }

  pub fn url(&self) -> &String {
    &self.url
  }


}



