/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageTrackingDetails {
  /// The package identifier.
  #[serde(rename = "packageNumber")]
  package_number: i32,
  /// The tracking number for the package.
  #[serde(rename = "trackingNumber")]
  tracking_number: Option<String>,
  /// Link on swiship.com that allows customers to track the package.
  #[serde(rename = "customerTrackingLink")]
  customer_tracking_link: Option<String>,
  /// The name of the carrier.
  #[serde(rename = "carrierCode")]
  carrier_code: Option<String>,
  /// The phone number of the carrier.
  #[serde(rename = "carrierPhoneNumber")]
  carrier_phone_number: Option<String>,
  /// The URL of the carrier’s website.
  #[serde(rename = "carrierURL")]
  carrier_url: Option<String>,
  /// The shipping date for the package.
  #[serde(rename = "shipDate")]
  ship_date: Option<::models::Timestamp>,
  /// The estimated arrival date.
  #[serde(rename = "estimatedArrivalDate")]
  estimated_arrival_date: Option<::models::Timestamp>,
  /// The destination city for the package.
  #[serde(rename = "shipToAddress")]
  ship_to_address: Option<::models::TrackingAddress>,
  #[serde(rename = "currentStatus")]
  current_status: Option<::models::CurrentStatus>,
  /// Description corresponding to the CurrentStatus value.
  #[serde(rename = "currentStatusDescription")]
  current_status_description: Option<String>,
  /// The name of the person who signed for the package.
  #[serde(rename = "signedForBy")]
  signed_for_by: Option<String>,
  #[serde(rename = "additionalLocationInfo")]
  additional_location_info: Option<::models::AdditionalLocationInfo>,
  #[serde(rename = "trackingEvents")]
  tracking_events: Option<::models::TrackingEventList>
}

impl PackageTrackingDetails {
  pub fn new(package_number: i32) -> PackageTrackingDetails {
    PackageTrackingDetails {
      package_number: package_number,
      tracking_number: None,
      customer_tracking_link: None,
      carrier_code: None,
      carrier_phone_number: None,
      carrier_url: None,
      ship_date: None,
      estimated_arrival_date: None,
      ship_to_address: None,
      current_status: None,
      current_status_description: None,
      signed_for_by: None,
      additional_location_info: None,
      tracking_events: None
    }
  }

  pub fn set_package_number(&mut self, package_number: i32) {
    self.package_number = package_number;
  }

  pub fn with_package_number(mut self, package_number: i32) -> PackageTrackingDetails {
    self.package_number = package_number;
    self
  }

  pub fn package_number(&self) -> &i32 {
    &self.package_number
  }


  pub fn set_tracking_number(&mut self, tracking_number: String) {
    self.tracking_number = Some(tracking_number);
  }

  pub fn with_tracking_number(mut self, tracking_number: String) -> PackageTrackingDetails {
    self.tracking_number = Some(tracking_number);
    self
  }

  pub fn tracking_number(&self) -> Option<&String> {
    self.tracking_number.as_ref()
  }

  pub fn reset_tracking_number(&mut self) {
    self.tracking_number = None;
  }

  pub fn set_customer_tracking_link(&mut self, customer_tracking_link: String) {
    self.customer_tracking_link = Some(customer_tracking_link);
  }

  pub fn with_customer_tracking_link(mut self, customer_tracking_link: String) -> PackageTrackingDetails {
    self.customer_tracking_link = Some(customer_tracking_link);
    self
  }

  pub fn customer_tracking_link(&self) -> Option<&String> {
    self.customer_tracking_link.as_ref()
  }

  pub fn reset_customer_tracking_link(&mut self) {
    self.customer_tracking_link = None;
  }

  pub fn set_carrier_code(&mut self, carrier_code: String) {
    self.carrier_code = Some(carrier_code);
  }

  pub fn with_carrier_code(mut self, carrier_code: String) -> PackageTrackingDetails {
    self.carrier_code = Some(carrier_code);
    self
  }

  pub fn carrier_code(&self) -> Option<&String> {
    self.carrier_code.as_ref()
  }

  pub fn reset_carrier_code(&mut self) {
    self.carrier_code = None;
  }

  pub fn set_carrier_phone_number(&mut self, carrier_phone_number: String) {
    self.carrier_phone_number = Some(carrier_phone_number);
  }

  pub fn with_carrier_phone_number(mut self, carrier_phone_number: String) -> PackageTrackingDetails {
    self.carrier_phone_number = Some(carrier_phone_number);
    self
  }

  pub fn carrier_phone_number(&self) -> Option<&String> {
    self.carrier_phone_number.as_ref()
  }

  pub fn reset_carrier_phone_number(&mut self) {
    self.carrier_phone_number = None;
  }

  pub fn set_carrier_url(&mut self, carrier_url: String) {
    self.carrier_url = Some(carrier_url);
  }

  pub fn with_carrier_url(mut self, carrier_url: String) -> PackageTrackingDetails {
    self.carrier_url = Some(carrier_url);
    self
  }

  pub fn carrier_url(&self) -> Option<&String> {
    self.carrier_url.as_ref()
  }

  pub fn reset_carrier_url(&mut self) {
    self.carrier_url = None;
  }

  pub fn set_ship_date(&mut self, ship_date: ::models::Timestamp) {
    self.ship_date = Some(ship_date);
  }

  pub fn with_ship_date(mut self, ship_date: ::models::Timestamp) -> PackageTrackingDetails {
    self.ship_date = Some(ship_date);
    self
  }

  pub fn ship_date(&self) -> Option<&::models::Timestamp> {
    self.ship_date.as_ref()
  }

  pub fn reset_ship_date(&mut self) {
    self.ship_date = None;
  }

  pub fn set_estimated_arrival_date(&mut self, estimated_arrival_date: ::models::Timestamp) {
    self.estimated_arrival_date = Some(estimated_arrival_date);
  }

  pub fn with_estimated_arrival_date(mut self, estimated_arrival_date: ::models::Timestamp) -> PackageTrackingDetails {
    self.estimated_arrival_date = Some(estimated_arrival_date);
    self
  }

  pub fn estimated_arrival_date(&self) -> Option<&::models::Timestamp> {
    self.estimated_arrival_date.as_ref()
  }

  pub fn reset_estimated_arrival_date(&mut self) {
    self.estimated_arrival_date = None;
  }

  pub fn set_ship_to_address(&mut self, ship_to_address: ::models::TrackingAddress) {
    self.ship_to_address = Some(ship_to_address);
  }

  pub fn with_ship_to_address(mut self, ship_to_address: ::models::TrackingAddress) -> PackageTrackingDetails {
    self.ship_to_address = Some(ship_to_address);
    self
  }

  pub fn ship_to_address(&self) -> Option<&::models::TrackingAddress> {
    self.ship_to_address.as_ref()
  }

  pub fn reset_ship_to_address(&mut self) {
    self.ship_to_address = None;
  }

  pub fn set_current_status(&mut self, current_status: ::models::CurrentStatus) {
    self.current_status = Some(current_status);
  }

  pub fn with_current_status(mut self, current_status: ::models::CurrentStatus) -> PackageTrackingDetails {
    self.current_status = Some(current_status);
    self
  }

  pub fn current_status(&self) -> Option<&::models::CurrentStatus> {
    self.current_status.as_ref()
  }

  pub fn reset_current_status(&mut self) {
    self.current_status = None;
  }

  pub fn set_current_status_description(&mut self, current_status_description: String) {
    self.current_status_description = Some(current_status_description);
  }

  pub fn with_current_status_description(mut self, current_status_description: String) -> PackageTrackingDetails {
    self.current_status_description = Some(current_status_description);
    self
  }

  pub fn current_status_description(&self) -> Option<&String> {
    self.current_status_description.as_ref()
  }

  pub fn reset_current_status_description(&mut self) {
    self.current_status_description = None;
  }

  pub fn set_signed_for_by(&mut self, signed_for_by: String) {
    self.signed_for_by = Some(signed_for_by);
  }

  pub fn with_signed_for_by(mut self, signed_for_by: String) -> PackageTrackingDetails {
    self.signed_for_by = Some(signed_for_by);
    self
  }

  pub fn signed_for_by(&self) -> Option<&String> {
    self.signed_for_by.as_ref()
  }

  pub fn reset_signed_for_by(&mut self) {
    self.signed_for_by = None;
  }

  pub fn set_additional_location_info(&mut self, additional_location_info: ::models::AdditionalLocationInfo) {
    self.additional_location_info = Some(additional_location_info);
  }

  pub fn with_additional_location_info(mut self, additional_location_info: ::models::AdditionalLocationInfo) -> PackageTrackingDetails {
    self.additional_location_info = Some(additional_location_info);
    self
  }

  pub fn additional_location_info(&self) -> Option<&::models::AdditionalLocationInfo> {
    self.additional_location_info.as_ref()
  }

  pub fn reset_additional_location_info(&mut self) {
    self.additional_location_info = None;
  }

  pub fn set_tracking_events(&mut self, tracking_events: ::models::TrackingEventList) {
    self.tracking_events = Some(tracking_events);
  }

  pub fn with_tracking_events(mut self, tracking_events: ::models::TrackingEventList) -> PackageTrackingDetails {
    self.tracking_events = Some(tracking_events);
    self
  }

  pub fn tracking_events(&self) -> Option<&::models::TrackingEventList> {
    self.tracking_events.as_ref()
  }

  pub fn reset_tracking_events(&mut self) {
    self.tracking_events = None;
  }

}



