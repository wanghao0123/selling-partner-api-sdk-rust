/* 
 * Selling Partner API for Listings Items
 *
 * The Selling Partner API for Listings Items (Listings Items API) provides programmatic access to selling partner listings on Amazon. Use this API in collaboration with the Selling Partner API for Product Type Definitions, which you use to retrieve the information about Amazon product types needed to use the Listings Items API.
 *
 * OpenAPI spec version: 2020-09-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ListingsItemPutRequest : The request body schema for the putListingsItem operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingsItemPutRequest {
  /// The Amazon product type of the listings item.
  #[serde(rename = "productType")]
  product_type: String,
  /// The name of the requirements set for the provided data.
  #[serde(rename = "requirements")]
  requirements: Option<String>,
  /// JSON object containing structured listings item attribute data keyed by attribute name.
  #[serde(rename = "attributes")]
  attributes: Value
}

impl ListingsItemPutRequest {
  /// The request body schema for the putListingsItem operation.
  pub fn new(product_type: String, attributes: Value) -> ListingsItemPutRequest {
    ListingsItemPutRequest {
      product_type: product_type,
      requirements: None,
      attributes: attributes
    }
  }

  pub fn set_product_type(&mut self, product_type: String) {
    self.product_type = product_type;
  }

  pub fn with_product_type(mut self, product_type: String) -> ListingsItemPutRequest {
    self.product_type = product_type;
    self
  }

  pub fn product_type(&self) -> &String {
    &self.product_type
  }


  pub fn set_requirements(&mut self, requirements: String) {
    self.requirements = Some(requirements);
  }

  pub fn with_requirements(mut self, requirements: String) -> ListingsItemPutRequest {
    self.requirements = Some(requirements);
    self
  }

  pub fn requirements(&self) -> Option<&String> {
    self.requirements.as_ref()
  }

  pub fn reset_requirements(&mut self) {
    self.requirements = None;
  }

  pub fn set_attributes(&mut self, attributes: Value) {
    self.attributes = attributes;
  }

  pub fn with_attributes(mut self, attributes: Value) -> ListingsItemPutRequest {
    self.attributes = attributes;
    self
  }

  pub fn attributes(&self) -> &Value {
    &self.attributes
  }


}



