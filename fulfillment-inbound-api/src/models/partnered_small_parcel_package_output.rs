/* 
 * Selling Partner API for Fulfillment Inbound
 *
 * The Selling Partner API for Fulfillment Inbound lets you create applications that create and update inbound shipments of inventory to Amazon's fulfillment network.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PartneredSmallParcelPackageOutput : Dimension, weight, and shipping information for the package.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartneredSmallParcelPackageOutput {
  #[serde(rename = "Dimensions")]
  dimensions: ::models::Dimensions,
  #[serde(rename = "Weight")]
  weight: ::models::Weight,
  /// The carrier specified with a previous call to putTransportDetails.
  #[serde(rename = "CarrierName")]
  carrier_name: String,
  #[serde(rename = "TrackingId")]
  tracking_id: ::models::TrackingId,
  #[serde(rename = "PackageStatus")]
  package_status: ::models::PackageStatus
}

impl PartneredSmallParcelPackageOutput {
  /// Dimension, weight, and shipping information for the package.
  pub fn new(dimensions: ::models::Dimensions, weight: ::models::Weight, carrier_name: String, tracking_id: ::models::TrackingId, package_status: ::models::PackageStatus) -> PartneredSmallParcelPackageOutput {
    PartneredSmallParcelPackageOutput {
      dimensions: dimensions,
      weight: weight,
      carrier_name: carrier_name,
      tracking_id: tracking_id,
      package_status: package_status
    }
  }

  pub fn set_dimensions(&mut self, dimensions: ::models::Dimensions) {
    self.dimensions = dimensions;
  }

  pub fn with_dimensions(mut self, dimensions: ::models::Dimensions) -> PartneredSmallParcelPackageOutput {
    self.dimensions = dimensions;
    self
  }

  pub fn dimensions(&self) -> &::models::Dimensions {
    &self.dimensions
  }


  pub fn set_weight(&mut self, weight: ::models::Weight) {
    self.weight = weight;
  }

  pub fn with_weight(mut self, weight: ::models::Weight) -> PartneredSmallParcelPackageOutput {
    self.weight = weight;
    self
  }

  pub fn weight(&self) -> &::models::Weight {
    &self.weight
  }


  pub fn set_carrier_name(&mut self, carrier_name: String) {
    self.carrier_name = carrier_name;
  }

  pub fn with_carrier_name(mut self, carrier_name: String) -> PartneredSmallParcelPackageOutput {
    self.carrier_name = carrier_name;
    self
  }

  pub fn carrier_name(&self) -> &String {
    &self.carrier_name
  }


  pub fn set_tracking_id(&mut self, tracking_id: ::models::TrackingId) {
    self.tracking_id = tracking_id;
  }

  pub fn with_tracking_id(mut self, tracking_id: ::models::TrackingId) -> PartneredSmallParcelPackageOutput {
    self.tracking_id = tracking_id;
    self
  }

  pub fn tracking_id(&self) -> &::models::TrackingId {
    &self.tracking_id
  }


  pub fn set_package_status(&mut self, package_status: ::models::PackageStatus) {
    self.package_status = package_status;
  }

  pub fn with_package_status(mut self, package_status: ::models::PackageStatus) -> PartneredSmallParcelPackageOutput {
    self.package_status = package_status;
    self
  }

  pub fn package_status(&self) -> &::models::PackageStatus {
    &self.package_status
  }


}



