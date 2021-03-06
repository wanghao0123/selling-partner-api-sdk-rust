/* 
 * Selling Partner API for Retail Procurement Shipments
 *
 * The Selling Partner API for Retail Procurement Shipments provides programmatic access to retail shipping data for vendors.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartyIdentification {
  /// Identification of the party by address.
  #[serde(rename = "address")]
  address: Option<::models::Address>,
  /// Assigned identification for the party.
  #[serde(rename = "partyId")]
  party_id: String,
  /// Tax registration details of the entity.
  #[serde(rename = "taxRegistrationDetails")]
  tax_registration_details: Option<Vec<::models::TaxRegistrationDetails>>
}

impl PartyIdentification {
  pub fn new(party_id: String) -> PartyIdentification {
    PartyIdentification {
      address: None,
      party_id: party_id,
      tax_registration_details: None
    }
  }

  pub fn set_address(&mut self, address: ::models::Address) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: ::models::Address) -> PartyIdentification {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&::models::Address> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

  pub fn set_party_id(&mut self, party_id: String) {
    self.party_id = party_id;
  }

  pub fn with_party_id(mut self, party_id: String) -> PartyIdentification {
    self.party_id = party_id;
    self
  }

  pub fn party_id(&self) -> &String {
    &self.party_id
  }


  pub fn set_tax_registration_details(&mut self, tax_registration_details: Vec<::models::TaxRegistrationDetails>) {
    self.tax_registration_details = Some(tax_registration_details);
  }

  pub fn with_tax_registration_details(mut self, tax_registration_details: Vec<::models::TaxRegistrationDetails>) -> PartyIdentification {
    self.tax_registration_details = Some(tax_registration_details);
    self
  }

  pub fn tax_registration_details(&self) -> Option<&Vec<::models::TaxRegistrationDetails>> {
    self.tax_registration_details.as_ref()
  }

  pub fn reset_tax_registration_details(&mut self) {
    self.tax_registration_details = None;
  }

}



