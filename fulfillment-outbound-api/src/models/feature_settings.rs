/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FeatureSettings : FeatureSettings allows users to apply fulfillment features to an order. To block an order from being shipped using Amazon Logistics (AMZL) and an AMZL tracking number, use featureName as BLOCK_AMZL and featureFulfillmentPolicy as Required. Blocking AMZL will incur an additional fee surcharge on your MCF orders and increase the risk of some of your orders being unfulfilled or delivered late if there are no alternative carriers available. Using BLOCK_AMZL in an order request will take precedence over your Seller Central account setting.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSettings {
  /// The name of the feature.
  #[serde(rename = "featureName")]
  feature_name: Option<String>,
  /// Specifies the policy to use when fulfilling an order.
  #[serde(rename = "featureFulfillmentPolicy")]
  feature_fulfillment_policy: Option<String>
}

impl FeatureSettings {
  /// FeatureSettings allows users to apply fulfillment features to an order. To block an order from being shipped using Amazon Logistics (AMZL) and an AMZL tracking number, use featureName as BLOCK_AMZL and featureFulfillmentPolicy as Required. Blocking AMZL will incur an additional fee surcharge on your MCF orders and increase the risk of some of your orders being unfulfilled or delivered late if there are no alternative carriers available. Using BLOCK_AMZL in an order request will take precedence over your Seller Central account setting.
  pub fn new() -> FeatureSettings {
    FeatureSettings {
      feature_name: None,
      feature_fulfillment_policy: None
    }
  }

  pub fn set_feature_name(&mut self, feature_name: String) {
    self.feature_name = Some(feature_name);
  }

  pub fn with_feature_name(mut self, feature_name: String) -> FeatureSettings {
    self.feature_name = Some(feature_name);
    self
  }

  pub fn feature_name(&self) -> Option<&String> {
    self.feature_name.as_ref()
  }

  pub fn reset_feature_name(&mut self) {
    self.feature_name = None;
  }

  pub fn set_feature_fulfillment_policy(&mut self, feature_fulfillment_policy: String) {
    self.feature_fulfillment_policy = Some(feature_fulfillment_policy);
  }

  pub fn with_feature_fulfillment_policy(mut self, feature_fulfillment_policy: String) -> FeatureSettings {
    self.feature_fulfillment_policy = Some(feature_fulfillment_policy);
    self
  }

  pub fn feature_fulfillment_policy(&self) -> Option<&String> {
    self.feature_fulfillment_policy.as_ref()
  }

  pub fn reset_feature_fulfillment_policy(&mut self) {
    self.feature_fulfillment_policy = None;
  }

}



