/* 
 * Selling Partner API for Merchant Fulfillment
 *
 * The Selling Partner API for Merchant Fulfillment helps you build applications that let sellers purchase shipping for non-Prime and Prime orders using Amazon’s Buy Shipping Services.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AvailableShippingServiceOptions : The available shipping service options.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableShippingServiceOptions {
  #[serde(rename = "AvailableCarrierWillPickUpOptions")]
  available_carrier_will_pick_up_options: ::models::AvailableCarrierWillPickUpOptionsList,
  #[serde(rename = "AvailableDeliveryExperienceOptions")]
  available_delivery_experience_options: ::models::AvailableDeliveryExperienceOptionsList
}

impl AvailableShippingServiceOptions {
  /// The available shipping service options.
  pub fn new(available_carrier_will_pick_up_options: ::models::AvailableCarrierWillPickUpOptionsList, available_delivery_experience_options: ::models::AvailableDeliveryExperienceOptionsList) -> AvailableShippingServiceOptions {
    AvailableShippingServiceOptions {
      available_carrier_will_pick_up_options: available_carrier_will_pick_up_options,
      available_delivery_experience_options: available_delivery_experience_options
    }
  }

  pub fn set_available_carrier_will_pick_up_options(&mut self, available_carrier_will_pick_up_options: ::models::AvailableCarrierWillPickUpOptionsList) {
    self.available_carrier_will_pick_up_options = available_carrier_will_pick_up_options;
  }

  pub fn with_available_carrier_will_pick_up_options(mut self, available_carrier_will_pick_up_options: ::models::AvailableCarrierWillPickUpOptionsList) -> AvailableShippingServiceOptions {
    self.available_carrier_will_pick_up_options = available_carrier_will_pick_up_options;
    self
  }

  pub fn available_carrier_will_pick_up_options(&self) -> &::models::AvailableCarrierWillPickUpOptionsList {
    &self.available_carrier_will_pick_up_options
  }


  pub fn set_available_delivery_experience_options(&mut self, available_delivery_experience_options: ::models::AvailableDeliveryExperienceOptionsList) {
    self.available_delivery_experience_options = available_delivery_experience_options;
  }

  pub fn with_available_delivery_experience_options(mut self, available_delivery_experience_options: ::models::AvailableDeliveryExperienceOptionsList) -> AvailableShippingServiceOptions {
    self.available_delivery_experience_options = available_delivery_experience_options;
    self
  }

  pub fn available_delivery_experience_options(&self) -> &::models::AvailableDeliveryExperienceOptionsList {
    &self.available_delivery_experience_options
  }


}



