/* 
 * Selling Partner API for Notifications
 *
 * The Selling Partner API for Notifications lets you subscribe to notifications that are relevant to a selling partner's business. Using this API you can create a destination to receive notifications, subscribe to notifications, delete notification subscriptions, and more.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CreateDestinationRequest : The request schema for the createDestination operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDestinationRequest {
  /// The information required to create a destination resource. Applications should use one resource type (sqs or eventBridge) per destination.
  #[serde(rename = "resourceSpecification")]
  resource_specification: ::models::DestinationResourceSpecification,
  /// A developer-defined name to help identify this destination.
  #[serde(rename = "name")]
  name: String
}

impl CreateDestinationRequest {
  /// The request schema for the createDestination operation.
  pub fn new(resource_specification: ::models::DestinationResourceSpecification, name: String) -> CreateDestinationRequest {
    CreateDestinationRequest {
      resource_specification: resource_specification,
      name: name
    }
  }

  pub fn set_resource_specification(&mut self, resource_specification: ::models::DestinationResourceSpecification) {
    self.resource_specification = resource_specification;
  }

  pub fn with_resource_specification(mut self, resource_specification: ::models::DestinationResourceSpecification) -> CreateDestinationRequest {
    self.resource_specification = resource_specification;
    self
  }

  pub fn resource_specification(&self) -> &::models::DestinationResourceSpecification {
    &self.resource_specification
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> CreateDestinationRequest {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



