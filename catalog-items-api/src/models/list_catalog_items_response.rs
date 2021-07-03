/* 
 * Selling Partner API for Catalog Items
 *
 * The Selling Partner API for Catalog Items helps you programmatically retrieve item details for items in the catalog.
 *
 * OpenAPI spec version: v0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCatalogItemsResponse {
  /// The payload for the listCatalogItems operation.
  #[serde(rename = "payload")]
  payload: Option<::models::ListMatchingItemsResponse>,
  /// One or more unexpected errors occurred during the listCatalogItems operation.
  #[serde(rename = "errors")]
  errors: Option<::models::ErrorList>
}

impl ListCatalogItemsResponse {
  pub fn new() -> ListCatalogItemsResponse {
    ListCatalogItemsResponse {
      payload: None,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: ::models::ListMatchingItemsResponse) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: ::models::ListMatchingItemsResponse) -> ListCatalogItemsResponse {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&::models::ListMatchingItemsResponse> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_errors(&mut self, errors: ::models::ErrorList) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: ::models::ErrorList) -> ListCatalogItemsResponse {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&::models::ErrorList> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

}



