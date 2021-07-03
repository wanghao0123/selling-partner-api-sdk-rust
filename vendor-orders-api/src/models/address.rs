/* 
 * Selling Partner API for Retail Procurement Orders
 *
 * The Selling Partner API for Retail Procurement Orders provides programmatic access to vendor orders data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Address : Address of the party.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  /// The name of the person, business or institution at that address.
  #[serde(rename = "name")]
  name: String,
  /// First line of the address.
  #[serde(rename = "addressLine1")]
  address_line1: String,
  /// Additional address information, if required.
  #[serde(rename = "addressLine2")]
  address_line2: Option<String>,
  /// Additional address information, if required.
  #[serde(rename = "addressLine3")]
  address_line3: Option<String>,
  /// The city where the person, business or institution is located.
  #[serde(rename = "city")]
  city: Option<String>,
  /// The county where person, business or institution is located.
  #[serde(rename = "county")]
  county: Option<String>,
  /// The district where person, business or institution is located.
  #[serde(rename = "district")]
  district: Option<String>,
  /// The state or region where person, business or institution is located.
  #[serde(rename = "stateOrRegion")]
  state_or_region: Option<String>,
  /// The postal code of that address. It conatins a series of letters or digits or both, sometimes including spaces or punctuation.
  #[serde(rename = "postalCode")]
  postal_code: Option<String>,
  /// The two digit country code. In ISO 3166-1 alpha-2 format.
  #[serde(rename = "countryCode")]
  country_code: String,
  /// The phone number of the person, business or institution located at that address.
  #[serde(rename = "phone")]
  phone: Option<String>
}

impl Address {
  /// Address of the party.
  pub fn new(name: String, address_line1: String, country_code: String) -> Address {
    Address {
      name: name,
      address_line1: address_line1,
      address_line2: None,
      address_line3: None,
      city: None,
      county: None,
      district: None,
      state_or_region: None,
      postal_code: None,
      country_code: country_code,
      phone: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Address {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_address_line1(&mut self, address_line1: String) {
    self.address_line1 = address_line1;
  }

  pub fn with_address_line1(mut self, address_line1: String) -> Address {
    self.address_line1 = address_line1;
    self
  }

  pub fn address_line1(&self) -> &String {
    &self.address_line1
  }


  pub fn set_address_line2(&mut self, address_line2: String) {
    self.address_line2 = Some(address_line2);
  }

  pub fn with_address_line2(mut self, address_line2: String) -> Address {
    self.address_line2 = Some(address_line2);
    self
  }

  pub fn address_line2(&self) -> Option<&String> {
    self.address_line2.as_ref()
  }

  pub fn reset_address_line2(&mut self) {
    self.address_line2 = None;
  }

  pub fn set_address_line3(&mut self, address_line3: String) {
    self.address_line3 = Some(address_line3);
  }

  pub fn with_address_line3(mut self, address_line3: String) -> Address {
    self.address_line3 = Some(address_line3);
    self
  }

  pub fn address_line3(&self) -> Option<&String> {
    self.address_line3.as_ref()
  }

  pub fn reset_address_line3(&mut self) {
    self.address_line3 = None;
  }

  pub fn set_city(&mut self, city: String) {
    self.city = Some(city);
  }

  pub fn with_city(mut self, city: String) -> Address {
    self.city = Some(city);
    self
  }

  pub fn city(&self) -> Option<&String> {
    self.city.as_ref()
  }

  pub fn reset_city(&mut self) {
    self.city = None;
  }

  pub fn set_county(&mut self, county: String) {
    self.county = Some(county);
  }

  pub fn with_county(mut self, county: String) -> Address {
    self.county = Some(county);
    self
  }

  pub fn county(&self) -> Option<&String> {
    self.county.as_ref()
  }

  pub fn reset_county(&mut self) {
    self.county = None;
  }

  pub fn set_district(&mut self, district: String) {
    self.district = Some(district);
  }

  pub fn with_district(mut self, district: String) -> Address {
    self.district = Some(district);
    self
  }

  pub fn district(&self) -> Option<&String> {
    self.district.as_ref()
  }

  pub fn reset_district(&mut self) {
    self.district = None;
  }

  pub fn set_state_or_region(&mut self, state_or_region: String) {
    self.state_or_region = Some(state_or_region);
  }

  pub fn with_state_or_region(mut self, state_or_region: String) -> Address {
    self.state_or_region = Some(state_or_region);
    self
  }

  pub fn state_or_region(&self) -> Option<&String> {
    self.state_or_region.as_ref()
  }

  pub fn reset_state_or_region(&mut self) {
    self.state_or_region = None;
  }

  pub fn set_postal_code(&mut self, postal_code: String) {
    self.postal_code = Some(postal_code);
  }

  pub fn with_postal_code(mut self, postal_code: String) -> Address {
    self.postal_code = Some(postal_code);
    self
  }

  pub fn postal_code(&self) -> Option<&String> {
    self.postal_code.as_ref()
  }

  pub fn reset_postal_code(&mut self) {
    self.postal_code = None;
  }

  pub fn set_country_code(&mut self, country_code: String) {
    self.country_code = country_code;
  }

  pub fn with_country_code(mut self, country_code: String) -> Address {
    self.country_code = country_code;
    self
  }

  pub fn country_code(&self) -> &String {
    &self.country_code
  }


  pub fn set_phone(&mut self, phone: String) {
    self.phone = Some(phone);
  }

  pub fn with_phone(mut self, phone: String) -> Address {
    self.phone = Some(phone);
    self
  }

  pub fn phone(&self) -> Option<&String> {
    self.phone.as_ref()
  }

  pub fn reset_phone(&mut self) {
    self.phone = None;
  }

}



