/* 
 * Selling Partner APIs for Fulfillment Outbound
 *
 * The Selling Partner API for Fulfillment Outbound lets you create applications that help a seller fulfill Multi-Channel Fulfillment orders using their inventory in Amazon's fulfillment network. You can get information on both potential and existing fulfillment orders.
 *
 * OpenAPI spec version: 2020-07-01
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFulfillmentPreviewRequest : The request body schema for the getFulfillmentPreview operation.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFulfillmentPreviewRequest {
  /// The marketplace the fulfillment order is placed against.
  #[serde(rename = "marketplaceId")]
  marketplace_id: Option<String>,
  /// The destination address for the fulfillment order preview.
  #[serde(rename = "address")]
  address: ::models::Address,
  /// Identifying information and quantity information for the items in the fulfillment order preview.
  #[serde(rename = "items")]
  items: ::models::GetFulfillmentPreviewItemList,
  /// A list of shipping methods used for creating fulfillment order previews.  Possible values:  * Standard - Standard shipping method. * Expedited - Expedited shipping method. * Priority - Priority shipping method. * ScheduledDelivery - Scheduled Delivery shipping method. Note: Shipping method service level agreements vary by marketplace. Sellers should see the Seller Central website in their marketplace for shipping method service level agreements and fulfillment fees.
  #[serde(rename = "shippingSpeedCategories")]
  shipping_speed_categories: Option<::models::ShippingSpeedCategoryList>,
  /// Specifies whether to return fulfillment order previews that are for COD (Cash On Delivery).  Possible values:  * true - Returns all fulfillment order previews (both for COD and not for COD). * false - Returns only fulfillment order previews that are not for COD.
  #[serde(rename = "includeCODFulfillmentPreview")]
  include_cod_fulfillment_preview: Option<bool>,
  /// Specifies whether to return the ScheduledDeliveryInfo response object, which contains the available delivery windows for a Scheduled Delivery. The ScheduledDeliveryInfo response object can only be returned for fulfillment order previews with ShippingSpeedCategories = ScheduledDelivery.
  #[serde(rename = "includeDeliveryWindows")]
  include_delivery_windows: Option<bool>,
  /// A list of features and their fulfillment policies to apply to the order.
  #[serde(rename = "featureConstraints")]
  feature_constraints: Option<Vec<::models::FeatureSettings>>
}

impl GetFulfillmentPreviewRequest {
  /// The request body schema for the getFulfillmentPreview operation.
  pub fn new(address: ::models::Address, items: ::models::GetFulfillmentPreviewItemList) -> GetFulfillmentPreviewRequest {
    GetFulfillmentPreviewRequest {
      marketplace_id: None,
      address: address,
      items: items,
      shipping_speed_categories: None,
      include_cod_fulfillment_preview: None,
      include_delivery_windows: None,
      feature_constraints: None
    }
  }

  pub fn set_marketplace_id(&mut self, marketplace_id: String) {
    self.marketplace_id = Some(marketplace_id);
  }

  pub fn with_marketplace_id(mut self, marketplace_id: String) -> GetFulfillmentPreviewRequest {
    self.marketplace_id = Some(marketplace_id);
    self
  }

  pub fn marketplace_id(&self) -> Option<&String> {
    self.marketplace_id.as_ref()
  }

  pub fn reset_marketplace_id(&mut self) {
    self.marketplace_id = None;
  }

  pub fn set_address(&mut self, address: ::models::Address) {
    self.address = address;
  }

  pub fn with_address(mut self, address: ::models::Address) -> GetFulfillmentPreviewRequest {
    self.address = address;
    self
  }

  pub fn address(&self) -> &::models::Address {
    &self.address
  }


  pub fn set_items(&mut self, items: ::models::GetFulfillmentPreviewItemList) {
    self.items = items;
  }

  pub fn with_items(mut self, items: ::models::GetFulfillmentPreviewItemList) -> GetFulfillmentPreviewRequest {
    self.items = items;
    self
  }

  pub fn items(&self) -> &::models::GetFulfillmentPreviewItemList {
    &self.items
  }


  pub fn set_shipping_speed_categories(&mut self, shipping_speed_categories: ::models::ShippingSpeedCategoryList) {
    self.shipping_speed_categories = Some(shipping_speed_categories);
  }

  pub fn with_shipping_speed_categories(mut self, shipping_speed_categories: ::models::ShippingSpeedCategoryList) -> GetFulfillmentPreviewRequest {
    self.shipping_speed_categories = Some(shipping_speed_categories);
    self
  }

  pub fn shipping_speed_categories(&self) -> Option<&::models::ShippingSpeedCategoryList> {
    self.shipping_speed_categories.as_ref()
  }

  pub fn reset_shipping_speed_categories(&mut self) {
    self.shipping_speed_categories = None;
  }

  pub fn set_include_cod_fulfillment_preview(&mut self, include_cod_fulfillment_preview: bool) {
    self.include_cod_fulfillment_preview = Some(include_cod_fulfillment_preview);
  }

  pub fn with_include_cod_fulfillment_preview(mut self, include_cod_fulfillment_preview: bool) -> GetFulfillmentPreviewRequest {
    self.include_cod_fulfillment_preview = Some(include_cod_fulfillment_preview);
    self
  }

  pub fn include_cod_fulfillment_preview(&self) -> Option<&bool> {
    self.include_cod_fulfillment_preview.as_ref()
  }

  pub fn reset_include_cod_fulfillment_preview(&mut self) {
    self.include_cod_fulfillment_preview = None;
  }

  pub fn set_include_delivery_windows(&mut self, include_delivery_windows: bool) {
    self.include_delivery_windows = Some(include_delivery_windows);
  }

  pub fn with_include_delivery_windows(mut self, include_delivery_windows: bool) -> GetFulfillmentPreviewRequest {
    self.include_delivery_windows = Some(include_delivery_windows);
    self
  }

  pub fn include_delivery_windows(&self) -> Option<&bool> {
    self.include_delivery_windows.as_ref()
  }

  pub fn reset_include_delivery_windows(&mut self) {
    self.include_delivery_windows = None;
  }

  pub fn set_feature_constraints(&mut self, feature_constraints: Vec<::models::FeatureSettings>) {
    self.feature_constraints = Some(feature_constraints);
  }

  pub fn with_feature_constraints(mut self, feature_constraints: Vec<::models::FeatureSettings>) -> GetFulfillmentPreviewRequest {
    self.feature_constraints = Some(feature_constraints);
    self
  }

  pub fn feature_constraints(&self) -> Option<&Vec<::models::FeatureSettings>> {
    self.feature_constraints.as_ref()
  }

  pub fn reset_feature_constraints(&mut self) {
    self.feature_constraints = None;
  }

}



