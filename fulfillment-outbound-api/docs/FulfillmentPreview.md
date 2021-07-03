# FulfillmentPreview

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipping_speed_category** | [***::models::ShippingSpeedCategory**](ShippingSpeedCategory.md) |  | [default to null]
**scheduled_delivery_info** | [***::models::ScheduledDeliveryInfo**](ScheduledDeliveryInfo.md) |  | [optional] [default to null]
**is_fulfillable** | **bool** | When true, this fulfillment order preview is fulfillable. | [default to null]
**is_cod_capable** | **bool** | When true, this fulfillment order preview is for COD (Cash On Delivery). | [default to null]
**estimated_shipping_weight** | [***::models::Weight**](Weight.md) | Estimated shipping weight for this fulfillment order preview. | [optional] [default to null]
**estimated_fees** | [***::models::FeeList**](FeeList.md) | The estimated fulfillment fees for this fulfillment order preview, if applicable. | [optional] [default to null]
**fulfillment_preview_shipments** | [***::models::FulfillmentPreviewShipmentList**](FulfillmentPreviewShipmentList.md) |  | [optional] [default to null]
**unfulfillable_preview_items** | [***::models::UnfulfillablePreviewItemList**](UnfulfillablePreviewItemList.md) |  | [optional] [default to null]
**order_unfulfillable_reasons** | [***::models::StringList**](StringList.md) | Error codes associated with the fulfillment order preview that indicate why the order is not fulfillable.  Error code examples:  DeliverySLAUnavailable InvalidDestinationAddress | [optional] [default to null]
**marketplace_id** | **String** | The marketplace the fulfillment order is placed against. | [default to null]
**feature_constraints** | [**Vec<::models::FeatureSettings>**](FeatureSettings.md) | A list of features and their fulfillment policies to apply to the order. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


