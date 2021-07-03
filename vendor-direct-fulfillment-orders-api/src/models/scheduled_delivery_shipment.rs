/* 
 * Selling Partner API for Direct Fulfillment Orders
 *
 * The Selling Partner API for Direct Fulfillment Orders provides programmatic access to a direct fulfillment vendor's order data.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ScheduledDeliveryShipment : Dates for the scheduled delivery shipments.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledDeliveryShipment {
  /// Scheduled delivery service type.
  #[serde(rename = "scheduledDeliveryServiceType")]
  scheduled_delivery_service_type: Option<String>,
  /// Earliest nominated delivery date for the scheduled delivery.
  #[serde(rename = "earliestNominatedDeliveryDate")]
  earliest_nominated_delivery_date: Option<String>,
  /// Latest nominated delivery date for the scheduled delivery.
  #[serde(rename = "latestNominatedDeliveryDate")]
  latest_nominated_delivery_date: Option<String>
}

impl ScheduledDeliveryShipment {
  /// Dates for the scheduled delivery shipments.
  pub fn new() -> ScheduledDeliveryShipment {
    ScheduledDeliveryShipment {
      scheduled_delivery_service_type: None,
      earliest_nominated_delivery_date: None,
      latest_nominated_delivery_date: None
    }
  }

  pub fn set_scheduled_delivery_service_type(&mut self, scheduled_delivery_service_type: String) {
    self.scheduled_delivery_service_type = Some(scheduled_delivery_service_type);
  }

  pub fn with_scheduled_delivery_service_type(mut self, scheduled_delivery_service_type: String) -> ScheduledDeliveryShipment {
    self.scheduled_delivery_service_type = Some(scheduled_delivery_service_type);
    self
  }

  pub fn scheduled_delivery_service_type(&self) -> Option<&String> {
    self.scheduled_delivery_service_type.as_ref()
  }

  pub fn reset_scheduled_delivery_service_type(&mut self) {
    self.scheduled_delivery_service_type = None;
  }

  pub fn set_earliest_nominated_delivery_date(&mut self, earliest_nominated_delivery_date: String) {
    self.earliest_nominated_delivery_date = Some(earliest_nominated_delivery_date);
  }

  pub fn with_earliest_nominated_delivery_date(mut self, earliest_nominated_delivery_date: String) -> ScheduledDeliveryShipment {
    self.earliest_nominated_delivery_date = Some(earliest_nominated_delivery_date);
    self
  }

  pub fn earliest_nominated_delivery_date(&self) -> Option<&String> {
    self.earliest_nominated_delivery_date.as_ref()
  }

  pub fn reset_earliest_nominated_delivery_date(&mut self) {
    self.earliest_nominated_delivery_date = None;
  }

  pub fn set_latest_nominated_delivery_date(&mut self, latest_nominated_delivery_date: String) {
    self.latest_nominated_delivery_date = Some(latest_nominated_delivery_date);
  }

  pub fn with_latest_nominated_delivery_date(mut self, latest_nominated_delivery_date: String) -> ScheduledDeliveryShipment {
    self.latest_nominated_delivery_date = Some(latest_nominated_delivery_date);
    self
  }

  pub fn latest_nominated_delivery_date(&self) -> Option<&String> {
    self.latest_nominated_delivery_date.as_ref()
  }

  pub fn reset_latest_nominated_delivery_date(&mut self) {
    self.latest_nominated_delivery_date = None;
  }

}



