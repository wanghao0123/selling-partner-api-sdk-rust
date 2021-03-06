/* 
 * Selling Partner API for Direct Fulfillment Payments
 *
 * The Selling Partner API for Direct Fulfillment Payments provides programmatic access to a direct fulfillment vendor's invoice data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AdditionalDetails : A field where selling party can provide additional information for tax related or any other purposes.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalDetails {
  /// The type of the additional information provided by the selling party.
  #[serde(rename = "type")]
  _type: String,
  /// The detail of the additional information provided by the selling party.
  #[serde(rename = "detail")]
  detail: String,
  /// The language code of the additional information detail.
  #[serde(rename = "languageCode")]
  language_code: Option<String>
}

impl AdditionalDetails {
  /// A field where selling party can provide additional information for tax related or any other purposes.
  pub fn new(_type: String, detail: String) -> AdditionalDetails {
    AdditionalDetails {
      _type: _type,
      detail: detail,
      language_code: None
    }
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> AdditionalDetails {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_detail(&mut self, detail: String) {
    self.detail = detail;
  }

  pub fn with_detail(mut self, detail: String) -> AdditionalDetails {
    self.detail = detail;
    self
  }

  pub fn detail(&self) -> &String {
    &self.detail
  }


  pub fn set_language_code(&mut self, language_code: String) {
    self.language_code = Some(language_code);
  }

  pub fn with_language_code(mut self, language_code: String) -> AdditionalDetails {
    self.language_code = Some(language_code);
    self
  }

  pub fn language_code(&self) -> Option<&String> {
    self.language_code.as_ref()
  }

  pub fn reset_language_code(&mut self) {
    self.language_code = None;
  }

}



