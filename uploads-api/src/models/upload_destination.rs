/* 
 * Selling Partner API for Uploads
 *
 * The Uploads API lets you upload files that you can programmatically access using other Selling Partner APIs, such as the A+ Content API and the Messaging API.
 *
 * OpenAPI spec version: 2020-11-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// UploadDestination : Information about an upload destination.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadDestination {
  /// The unique identifier for the upload destination.
  #[serde(rename = "uploadDestinationId")]
  upload_destination_id: Option<String>,
  /// The URL for the upload destination.
  #[serde(rename = "url")]
  url: Option<String>,
  /// The headers to include in the upload request.
  #[serde(rename = "headers")]
  headers: Option<Value>
}

impl UploadDestination {
  /// Information about an upload destination.
  pub fn new() -> UploadDestination {
    UploadDestination {
      upload_destination_id: None,
      url: None,
      headers: None
    }
  }

  pub fn set_upload_destination_id(&mut self, upload_destination_id: String) {
    self.upload_destination_id = Some(upload_destination_id);
  }

  pub fn with_upload_destination_id(mut self, upload_destination_id: String) -> UploadDestination {
    self.upload_destination_id = Some(upload_destination_id);
    self
  }

  pub fn upload_destination_id(&self) -> Option<&String> {
    self.upload_destination_id.as_ref()
  }

  pub fn reset_upload_destination_id(&mut self) {
    self.upload_destination_id = None;
  }

  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> UploadDestination {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_headers(&mut self, headers: Value) {
    self.headers = Some(headers);
  }

  pub fn with_headers(mut self, headers: Value) -> UploadDestination {
    self.headers = Some(headers);
    self
  }

  pub fn headers(&self) -> Option<&Value> {
    self.headers.as_ref()
  }

  pub fn reset_headers(&mut self) {
    self.headers = None;
  }

}



