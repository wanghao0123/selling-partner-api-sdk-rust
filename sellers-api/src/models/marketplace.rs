/* 
 * Selling Partner API for Sellers
 *
 * The Selling Partner API for Sellers lets you retrieve information on behalf of sellers about their seller account, such as the marketplaces they participate in. Along with listing the marketplaces that a seller can sell in, the API also provides additional information about the marketplace such as the default language and the default currency. The API also provides seller-specific information such as whether the seller has suspended listings in that marketplace.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Marketplace : Detailed information about an Amazon market where a seller can list items for sale and customers can view and purchase items.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Marketplace {
  /// The encrypted marketplace value.
  #[serde(rename = "id")]
  id: String,
  /// Marketplace name.
  #[serde(rename = "name")]
  name: String,
  /// The ISO 3166-1 alpha-2 format country code of the marketplace.
  #[serde(rename = "countryCode")]
  country_code: String,
  /// The ISO 4217 format currency code of the marketplace.
  #[serde(rename = "defaultCurrencyCode")]
  default_currency_code: String,
  /// The ISO 639-1 format language code of the marketplace.
  #[serde(rename = "defaultLanguageCode")]
  default_language_code: String,
  /// The domain name of the marketplace.
  #[serde(rename = "domainName")]
  domain_name: String
}

impl Marketplace {
  /// Detailed information about an Amazon market where a seller can list items for sale and customers can view and purchase items.
  pub fn new(id: String, name: String, country_code: String, default_currency_code: String, default_language_code: String, domain_name: String) -> Marketplace {
    Marketplace {
      id: id,
      name: name,
      country_code: country_code,
      default_currency_code: default_currency_code,
      default_language_code: default_language_code,
      domain_name: domain_name
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> Marketplace {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Marketplace {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_country_code(&mut self, country_code: String) {
    self.country_code = country_code;
  }

  pub fn with_country_code(mut self, country_code: String) -> Marketplace {
    self.country_code = country_code;
    self
  }

  pub fn country_code(&self) -> &String {
    &self.country_code
  }


  pub fn set_default_currency_code(&mut self, default_currency_code: String) {
    self.default_currency_code = default_currency_code;
  }

  pub fn with_default_currency_code(mut self, default_currency_code: String) -> Marketplace {
    self.default_currency_code = default_currency_code;
    self
  }

  pub fn default_currency_code(&self) -> &String {
    &self.default_currency_code
  }


  pub fn set_default_language_code(&mut self, default_language_code: String) {
    self.default_language_code = default_language_code;
  }

  pub fn with_default_language_code(mut self, default_language_code: String) -> Marketplace {
    self.default_language_code = default_language_code;
    self
  }

  pub fn default_language_code(&self) -> &String {
    &self.default_language_code
  }


  pub fn set_domain_name(&mut self, domain_name: String) {
    self.domain_name = domain_name;
  }

  pub fn with_domain_name(mut self, domain_name: String) -> Marketplace {
    self.domain_name = domain_name;
    self
  }

  pub fn domain_name(&self) -> &String {
    &self.domain_name
  }


}



