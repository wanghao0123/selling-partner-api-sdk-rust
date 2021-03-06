/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Rate : The available rate that can be used to send the shipment

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rate {
  /// An identifier for the rate.
  #[serde(rename = "rateId")]
  rate_id: Option<String>,
  /// The total charge that will be billed for the rate.
  #[serde(rename = "totalCharge")]
  total_charge: Option<::models::Currency>,
  /// The weight that was used to calculate the totalCharge.
  #[serde(rename = "billedWeight")]
  billed_weight: Option<::models::Weight>,
  /// The time after which the offering will expire.
  #[serde(rename = "expirationTime")]
  expiration_time: Option<String>,
  #[serde(rename = "serviceType")]
  service_type: Option<::models::ServiceType>,
  #[serde(rename = "promise")]
  promise: Option<::models::ShippingPromiseSet>
}

impl Rate {
  /// The available rate that can be used to send the shipment
  pub fn new() -> Rate {
    Rate {
      rate_id: None,
      total_charge: None,
      billed_weight: None,
      expiration_time: None,
      service_type: None,
      promise: None
    }
  }

  pub fn set_rate_id(&mut self, rate_id: String) {
    self.rate_id = Some(rate_id);
  }

  pub fn with_rate_id(mut self, rate_id: String) -> Rate {
    self.rate_id = Some(rate_id);
    self
  }

  pub fn rate_id(&self) -> Option<&String> {
    self.rate_id.as_ref()
  }

  pub fn reset_rate_id(&mut self) {
    self.rate_id = None;
  }

  pub fn set_total_charge(&mut self, total_charge: ::models::Currency) {
    self.total_charge = Some(total_charge);
  }

  pub fn with_total_charge(mut self, total_charge: ::models::Currency) -> Rate {
    self.total_charge = Some(total_charge);
    self
  }

  pub fn total_charge(&self) -> Option<&::models::Currency> {
    self.total_charge.as_ref()
  }

  pub fn reset_total_charge(&mut self) {
    self.total_charge = None;
  }

  pub fn set_billed_weight(&mut self, billed_weight: ::models::Weight) {
    self.billed_weight = Some(billed_weight);
  }

  pub fn with_billed_weight(mut self, billed_weight: ::models::Weight) -> Rate {
    self.billed_weight = Some(billed_weight);
    self
  }

  pub fn billed_weight(&self) -> Option<&::models::Weight> {
    self.billed_weight.as_ref()
  }

  pub fn reset_billed_weight(&mut self) {
    self.billed_weight = None;
  }

  pub fn set_expiration_time(&mut self, expiration_time: String) {
    self.expiration_time = Some(expiration_time);
  }

  pub fn with_expiration_time(mut self, expiration_time: String) -> Rate {
    self.expiration_time = Some(expiration_time);
    self
  }

  pub fn expiration_time(&self) -> Option<&String> {
    self.expiration_time.as_ref()
  }

  pub fn reset_expiration_time(&mut self) {
    self.expiration_time = None;
  }

  pub fn set_service_type(&mut self, service_type: ::models::ServiceType) {
    self.service_type = Some(service_type);
  }

  pub fn with_service_type(mut self, service_type: ::models::ServiceType) -> Rate {
    self.service_type = Some(service_type);
    self
  }

  pub fn service_type(&self) -> Option<&::models::ServiceType> {
    self.service_type.as_ref()
  }

  pub fn reset_service_type(&mut self) {
    self.service_type = None;
  }

  pub fn set_promise(&mut self, promise: ::models::ShippingPromiseSet) {
    self.promise = Some(promise);
  }

  pub fn with_promise(mut self, promise: ::models::ShippingPromiseSet) -> Rate {
    self.promise = Some(promise);
    self
  }

  pub fn promise(&self) -> Option<&::models::ShippingPromiseSet> {
    self.promise.as_ref()
  }

  pub fn reset_promise(&mut self) {
    self.promise = None;
  }

}



