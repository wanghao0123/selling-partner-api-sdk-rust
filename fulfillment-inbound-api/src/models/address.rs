/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  /// Name of the individual or business.
  #[serde(rename = "Name")]
  name: String,
  /// The street address information.
  #[serde(rename = "AddressLine1")]
  address_line1: String,
  /// Additional street address information, if required.
  #[serde(rename = "AddressLine2")]
  address_line2: Option<String>,
  /// The district or county.
  #[serde(rename = "DistrictOrCounty")]
  district_or_county: Option<String>,
  /// The city.
  #[serde(rename = "City")]
  city: String,
  /// The state or province code.  If state or province codes are used in your marketplace, it is recommended that you include one with your request. This helps Amazon to select the most appropriate Amazon fulfillment center for your inbound shipment plan.
  #[serde(rename = "StateOrProvinceCode")]
  state_or_province_code: String,
  /// The country code in two-character ISO 3166-1 alpha-2 format.
  #[serde(rename = "CountryCode")]
  country_code: String,
  /// The postal code.  If postal codes are used in your marketplace, we recommended that you include one with your request. This helps Amazon select the most appropriate Amazon fulfillment center for the inbound shipment plan.
  #[serde(rename = "PostalCode")]
  postal_code: String
}

impl Address {
  pub fn new(name: String, address_line1: String, city: String, state_or_province_code: String, country_code: String, postal_code: String) -> Address {
    Address {
      name: name,
      address_line1: address_line1,
      address_line2: None,
      district_or_county: None,
      city: city,
      state_or_province_code: state_or_province_code,
      country_code: country_code,
      postal_code: postal_code
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

  pub fn set_district_or_county(&mut self, district_or_county: String) {
    self.district_or_county = Some(district_or_county);
  }

  pub fn with_district_or_county(mut self, district_or_county: String) -> Address {
    self.district_or_county = Some(district_or_county);
    self
  }

  pub fn district_or_county(&self) -> Option<&String> {
    self.district_or_county.as_ref()
  }

  pub fn reset_district_or_county(&mut self) {
    self.district_or_county = None;
  }

  pub fn set_city(&mut self, city: String) {
    self.city = city;
  }

  pub fn with_city(mut self, city: String) -> Address {
    self.city = city;
    self
  }

  pub fn city(&self) -> &String {
    &self.city
  }


  pub fn set_state_or_province_code(&mut self, state_or_province_code: String) {
    self.state_or_province_code = state_or_province_code;
  }

  pub fn with_state_or_province_code(mut self, state_or_province_code: String) -> Address {
    self.state_or_province_code = state_or_province_code;
    self
  }

  pub fn state_or_province_code(&self) -> &String {
    &self.state_or_province_code
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


  pub fn set_postal_code(&mut self, postal_code: String) {
    self.postal_code = postal_code;
  }

  pub fn with_postal_code(mut self, postal_code: String) -> Address {
    self.postal_code = postal_code;
    self
  }

  pub fn postal_code(&self) -> &String {
    &self.postal_code
  }


}



