/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DeliveryWindow : The time range within which a Scheduled Delivery fulfillment order should be delivered.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryWindow {
  /// The date and time of the start of the Scheduled Delivery window, in ISO 8601 date time format.
  #[serde(rename = "startDate")]
  start_date: ::models::Timestamp,
  /// The date and time of the end of the Scheduled Delivery window, in ISO 8601 date time format.
  #[serde(rename = "endDate")]
  end_date: ::models::Timestamp
}

impl DeliveryWindow {
  /// The time range within which a Scheduled Delivery fulfillment order should be delivered.
  pub fn new(start_date: ::models::Timestamp, end_date: ::models::Timestamp) -> DeliveryWindow {
    DeliveryWindow {
      start_date: start_date,
      end_date: end_date
    }
  }

  pub fn set_start_date(&mut self, start_date: ::models::Timestamp) {
    self.start_date = start_date;
  }

  pub fn with_start_date(mut self, start_date: ::models::Timestamp) -> DeliveryWindow {
    self.start_date = start_date;
    self
  }

  pub fn start_date(&self) -> &::models::Timestamp {
    &self.start_date
  }


  pub fn set_end_date(&mut self, end_date: ::models::Timestamp) {
    self.end_date = end_date;
  }

  pub fn with_end_date(mut self, end_date: ::models::Timestamp) -> DeliveryWindow {
    self.end_date = end_date;
    self
  }

  pub fn end_date(&self) -> &::models::Timestamp {
    &self.end_date
  }


}



