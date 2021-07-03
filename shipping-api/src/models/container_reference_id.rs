/* 
 * Selling Partner API for Shipping
 *
 * Provides programmatic access to Amazon Shipping APIs.
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ContainerReferenceId : An identifier for the container. This must be unique within all the containers in the same shipment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerReferenceId {
}

impl ContainerReferenceId {
  /// An identifier for the container. This must be unique within all the containers in the same shipment.
  pub fn new() -> ContainerReferenceId {
    ContainerReferenceId {
    }
  }

}



