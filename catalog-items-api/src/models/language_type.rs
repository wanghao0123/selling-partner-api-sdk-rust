/* 
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LanguageType : The language type attribute of an item.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageType {
  /// The name attribute of the item.
  #[serde(rename = "Name")]
  name: Option<String>,
  /// The type attribute of the item.
  #[serde(rename = "Type")]
  _type: Option<String>,
  /// The audio format attribute of the item.
  #[serde(rename = "AudioFormat")]
  audio_format: Option<String>
}

impl LanguageType {
  /// The language type attribute of an item.
  pub fn new() -> LanguageType {
    LanguageType {
      name: None,
      _type: None,
      audio_format: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> LanguageType {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> LanguageType {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_audio_format(&mut self, audio_format: String) {
    self.audio_format = Some(audio_format);
  }

  pub fn with_audio_format(mut self, audio_format: String) -> LanguageType {
    self.audio_format = Some(audio_format);
    self
  }

  pub fn audio_format(&self) -> Option<&String> {
    self.audio_format.as_ref()
  }

  pub fn reset_audio_format(&mut self) {
    self.audio_format = None;
  }

}



