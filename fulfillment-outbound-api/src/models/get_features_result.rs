/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFeaturesResult : The payload for the getFeatures operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFeaturesResult {
  #[serde(rename = "features")]
  features: ::models::Features
}

impl GetFeaturesResult {
  /// The payload for the getFeatures operation.
  pub fn new(features: ::models::Features) -> GetFeaturesResult {
    GetFeaturesResult {
      features: features
    }
  }

  pub fn set_features(&mut self, features: ::models::Features) {
    self.features = features;
  }

  pub fn with_features(mut self, features: ::models::Features) -> GetFeaturesResult {
    self.features = features;
    self
  }

  pub fn features(&self) -> &::models::Features {
    &self.features
  }


}



