/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FulfillmentShipmentPackage : Package information for a shipment in a fulfillment order.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentShipmentPackage {
  /// Identifies a package in a shipment.
  #[serde(rename = "packageNumber")]
  package_number: i32,
  /// Identifies the carrier who will deliver the shipment to the recipient.
  #[serde(rename = "carrierCode")]
  carrier_code: String,
  /// The tracking number, if provided, can be used to obtain tracking and delivery information.
  #[serde(rename = "trackingNumber")]
  tracking_number: Option<String>,
  /// The estimated arrival date and time of the package, in ISO 8601 date time format.
  #[serde(rename = "estimatedArrivalDate")]
  estimated_arrival_date: Option<::models::Timestamp>
}

impl FulfillmentShipmentPackage {
  /// Package information for a shipment in a fulfillment order.
  pub fn new(package_number: i32, carrier_code: String) -> FulfillmentShipmentPackage {
    FulfillmentShipmentPackage {
      package_number: package_number,
      carrier_code: carrier_code,
      tracking_number: None,
      estimated_arrival_date: None
    }
  }

  pub fn set_package_number(&mut self, package_number: i32) {
    self.package_number = package_number;
  }

  pub fn with_package_number(mut self, package_number: i32) -> FulfillmentShipmentPackage {
    self.package_number = package_number;
    self
  }

  pub fn package_number(&self) -> &i32 {
    &self.package_number
  }


  pub fn set_carrier_code(&mut self, carrier_code: String) {
    self.carrier_code = carrier_code;
  }

  pub fn with_carrier_code(mut self, carrier_code: String) -> FulfillmentShipmentPackage {
    self.carrier_code = carrier_code;
    self
  }

  pub fn carrier_code(&self) -> &String {
    &self.carrier_code
  }


  pub fn set_tracking_number(&mut self, tracking_number: String) {
    self.tracking_number = Some(tracking_number);
  }

  pub fn with_tracking_number(mut self, tracking_number: String) -> FulfillmentShipmentPackage {
    self.tracking_number = Some(tracking_number);
    self
  }

  pub fn tracking_number(&self) -> Option<&String> {
    self.tracking_number.as_ref()
  }

  pub fn reset_tracking_number(&mut self) {
    self.tracking_number = None;
  }

  pub fn set_estimated_arrival_date(&mut self, estimated_arrival_date: ::models::Timestamp) {
    self.estimated_arrival_date = Some(estimated_arrival_date);
  }

  pub fn with_estimated_arrival_date(mut self, estimated_arrival_date: ::models::Timestamp) -> FulfillmentShipmentPackage {
    self.estimated_arrival_date = Some(estimated_arrival_date);
    self
  }

  pub fn estimated_arrival_date(&self) -> Option<&::models::Timestamp> {
    self.estimated_arrival_date.as_ref()
  }

  pub fn reset_estimated_arrival_date(&mut self) {
    self.estimated_arrival_date = None;
  }

}



