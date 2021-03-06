/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TransportDetailInput : Information required to create an Amazon-partnered carrier shipping estimate, or to alert the Amazon fulfillment center to the arrival of an inbound shipment by a non-Amazon-partnered carrier.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDetailInput {
  #[serde(rename = "PartneredSmallParcelData")]
  partnered_small_parcel_data: Option<::models::PartneredSmallParcelDataInput>,
  #[serde(rename = "NonPartneredSmallParcelData")]
  non_partnered_small_parcel_data: Option<::models::NonPartneredSmallParcelDataInput>,
  #[serde(rename = "PartneredLtlData")]
  partnered_ltl_data: Option<::models::PartneredLtlDataInput>,
  #[serde(rename = "NonPartneredLtlData")]
  non_partnered_ltl_data: Option<::models::NonPartneredLtlDataInput>
}

impl TransportDetailInput {
  /// Information required to create an Amazon-partnered carrier shipping estimate, or to alert the Amazon fulfillment center to the arrival of an inbound shipment by a non-Amazon-partnered carrier.
  pub fn new() -> TransportDetailInput {
    TransportDetailInput {
      partnered_small_parcel_data: None,
      non_partnered_small_parcel_data: None,
      partnered_ltl_data: None,
      non_partnered_ltl_data: None
    }
  }

  pub fn set_partnered_small_parcel_data(&mut self, partnered_small_parcel_data: ::models::PartneredSmallParcelDataInput) {
    self.partnered_small_parcel_data = Some(partnered_small_parcel_data);
  }

  pub fn with_partnered_small_parcel_data(mut self, partnered_small_parcel_data: ::models::PartneredSmallParcelDataInput) -> TransportDetailInput {
    self.partnered_small_parcel_data = Some(partnered_small_parcel_data);
    self
  }

  pub fn partnered_small_parcel_data(&self) -> Option<&::models::PartneredSmallParcelDataInput> {
    self.partnered_small_parcel_data.as_ref()
  }

  pub fn reset_partnered_small_parcel_data(&mut self) {
    self.partnered_small_parcel_data = None;
  }

  pub fn set_non_partnered_small_parcel_data(&mut self, non_partnered_small_parcel_data: ::models::NonPartneredSmallParcelDataInput) {
    self.non_partnered_small_parcel_data = Some(non_partnered_small_parcel_data);
  }

  pub fn with_non_partnered_small_parcel_data(mut self, non_partnered_small_parcel_data: ::models::NonPartneredSmallParcelDataInput) -> TransportDetailInput {
    self.non_partnered_small_parcel_data = Some(non_partnered_small_parcel_data);
    self
  }

  pub fn non_partnered_small_parcel_data(&self) -> Option<&::models::NonPartneredSmallParcelDataInput> {
    self.non_partnered_small_parcel_data.as_ref()
  }

  pub fn reset_non_partnered_small_parcel_data(&mut self) {
    self.non_partnered_small_parcel_data = None;
  }

  pub fn set_partnered_ltl_data(&mut self, partnered_ltl_data: ::models::PartneredLtlDataInput) {
    self.partnered_ltl_data = Some(partnered_ltl_data);
  }

  pub fn with_partnered_ltl_data(mut self, partnered_ltl_data: ::models::PartneredLtlDataInput) -> TransportDetailInput {
    self.partnered_ltl_data = Some(partnered_ltl_data);
    self
  }

  pub fn partnered_ltl_data(&self) -> Option<&::models::PartneredLtlDataInput> {
    self.partnered_ltl_data.as_ref()
  }

  pub fn reset_partnered_ltl_data(&mut self) {
    self.partnered_ltl_data = None;
  }

  pub fn set_non_partnered_ltl_data(&mut self, non_partnered_ltl_data: ::models::NonPartneredLtlDataInput) {
    self.non_partnered_ltl_data = Some(non_partnered_ltl_data);
  }

  pub fn with_non_partnered_ltl_data(mut self, non_partnered_ltl_data: ::models::NonPartneredLtlDataInput) -> TransportDetailInput {
    self.non_partnered_ltl_data = Some(non_partnered_ltl_data);
    self
  }

  pub fn non_partnered_ltl_data(&self) -> Option<&::models::NonPartneredLtlDataInput> {
    self.non_partnered_ltl_data.as_ref()
  }

  pub fn reset_non_partnered_ltl_data(&mut self) {
    self.non_partnered_ltl_data = None;
  }

}



