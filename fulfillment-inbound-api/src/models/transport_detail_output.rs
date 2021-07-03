/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TransportDetailOutput : Inbound shipment information, including carrier details and shipment status.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportDetailOutput {
  #[serde(rename = "PartneredSmallParcelData")]
  partnered_small_parcel_data: Option<::models::PartneredSmallParcelDataOutput>,
  #[serde(rename = "NonPartneredSmallParcelData")]
  non_partnered_small_parcel_data: Option<::models::NonPartneredSmallParcelDataOutput>,
  #[serde(rename = "PartneredLtlData")]
  partnered_ltl_data: Option<::models::PartneredLtlDataOutput>,
  #[serde(rename = "NonPartneredLtlData")]
  non_partnered_ltl_data: Option<::models::NonPartneredLtlDataOutput>
}

impl TransportDetailOutput {
  /// Inbound shipment information, including carrier details and shipment status.
  pub fn new() -> TransportDetailOutput {
    TransportDetailOutput {
      partnered_small_parcel_data: None,
      non_partnered_small_parcel_data: None,
      partnered_ltl_data: None,
      non_partnered_ltl_data: None
    }
  }

  pub fn set_partnered_small_parcel_data(&mut self, partnered_small_parcel_data: ::models::PartneredSmallParcelDataOutput) {
    self.partnered_small_parcel_data = Some(partnered_small_parcel_data);
  }

  pub fn with_partnered_small_parcel_data(mut self, partnered_small_parcel_data: ::models::PartneredSmallParcelDataOutput) -> TransportDetailOutput {
    self.partnered_small_parcel_data = Some(partnered_small_parcel_data);
    self
  }

  pub fn partnered_small_parcel_data(&self) -> Option<&::models::PartneredSmallParcelDataOutput> {
    self.partnered_small_parcel_data.as_ref()
  }

  pub fn reset_partnered_small_parcel_data(&mut self) {
    self.partnered_small_parcel_data = None;
  }

  pub fn set_non_partnered_small_parcel_data(&mut self, non_partnered_small_parcel_data: ::models::NonPartneredSmallParcelDataOutput) {
    self.non_partnered_small_parcel_data = Some(non_partnered_small_parcel_data);
  }

  pub fn with_non_partnered_small_parcel_data(mut self, non_partnered_small_parcel_data: ::models::NonPartneredSmallParcelDataOutput) -> TransportDetailOutput {
    self.non_partnered_small_parcel_data = Some(non_partnered_small_parcel_data);
    self
  }

  pub fn non_partnered_small_parcel_data(&self) -> Option<&::models::NonPartneredSmallParcelDataOutput> {
    self.non_partnered_small_parcel_data.as_ref()
  }

  pub fn reset_non_partnered_small_parcel_data(&mut self) {
    self.non_partnered_small_parcel_data = None;
  }

  pub fn set_partnered_ltl_data(&mut self, partnered_ltl_data: ::models::PartneredLtlDataOutput) {
    self.partnered_ltl_data = Some(partnered_ltl_data);
  }

  pub fn with_partnered_ltl_data(mut self, partnered_ltl_data: ::models::PartneredLtlDataOutput) -> TransportDetailOutput {
    self.partnered_ltl_data = Some(partnered_ltl_data);
    self
  }

  pub fn partnered_ltl_data(&self) -> Option<&::models::PartneredLtlDataOutput> {
    self.partnered_ltl_data.as_ref()
  }

  pub fn reset_partnered_ltl_data(&mut self) {
    self.partnered_ltl_data = None;
  }

  pub fn set_non_partnered_ltl_data(&mut self, non_partnered_ltl_data: ::models::NonPartneredLtlDataOutput) {
    self.non_partnered_ltl_data = Some(non_partnered_ltl_data);
  }

  pub fn with_non_partnered_ltl_data(mut self, non_partnered_ltl_data: ::models::NonPartneredLtlDataOutput) -> TransportDetailOutput {
    self.non_partnered_ltl_data = Some(non_partnered_ltl_data);
    self
  }

  pub fn non_partnered_ltl_data(&self) -> Option<&::models::NonPartneredLtlDataOutput> {
    self.non_partnered_ltl_data.as_ref()
  }

  pub fn reset_non_partnered_ltl_data(&mut self) {
    self.non_partnered_ltl_data = None;
  }

}



