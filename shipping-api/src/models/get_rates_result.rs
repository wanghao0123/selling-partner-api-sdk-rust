/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetRatesResult : The payload schema for the getRates operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRatesResult {
  #[serde(rename = "serviceRates")]
  service_rates: ::models::ServiceRateList
}

impl GetRatesResult {
  /// The payload schema for the getRates operation.
  pub fn new(service_rates: ::models::ServiceRateList) -> GetRatesResult {
    GetRatesResult {
      service_rates: service_rates
    }
  }

  pub fn set_service_rates(&mut self, service_rates: ::models::ServiceRateList) {
    self.service_rates = service_rates;
  }

  pub fn with_service_rates(mut self, service_rates: ::models::ServiceRateList) -> GetRatesResult {
    self.service_rates = service_rates;
    self
  }

  pub fn service_rates(&self) -> &::models::ServiceRateList {
    &self.service_rates
  }


}



