/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AcceptedRate : The specific rate purchased for the shipment, or null if unpurchased.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptedRate {
  /// The total charge that will be billed for the rate.
  #[serde(rename = "totalCharge")]
  total_charge: Option<::models::Currency>,
  /// The weight that was used to calculate the totalCharge.
  #[serde(rename = "billedWeight")]
  billed_weight: Option<::models::Weight>,
  #[serde(rename = "serviceType")]
  service_type: Option<::models::ServiceType>,
  #[serde(rename = "promise")]
  promise: Option<::models::ShippingPromiseSet>
}

impl AcceptedRate {
  /// The specific rate purchased for the shipment, or null if unpurchased.
  pub fn new() -> AcceptedRate {
    AcceptedRate {
      total_charge: None,
      billed_weight: None,
      service_type: None,
      promise: None
    }
  }

  pub fn set_total_charge(&mut self, total_charge: ::models::Currency) {
    self.total_charge = Some(total_charge);
  }

  pub fn with_total_charge(mut self, total_charge: ::models::Currency) -> AcceptedRate {
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

  pub fn with_billed_weight(mut self, billed_weight: ::models::Weight) -> AcceptedRate {
    self.billed_weight = Some(billed_weight);
    self
  }

  pub fn billed_weight(&self) -> Option<&::models::Weight> {
    self.billed_weight.as_ref()
  }

  pub fn reset_billed_weight(&mut self) {
    self.billed_weight = None;
  }

  pub fn set_service_type(&mut self, service_type: ::models::ServiceType) {
    self.service_type = Some(service_type);
  }

  pub fn with_service_type(mut self, service_type: ::models::ServiceType) -> AcceptedRate {
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

  pub fn with_promise(mut self, promise: ::models::ShippingPromiseSet) -> AcceptedRate {
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



