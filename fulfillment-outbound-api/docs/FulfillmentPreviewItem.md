# FulfillmentPreviewItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU of the item. | [default to null]
**quantity** | [***::models::Quantity**](Quantity.md) | The item quantity. | [default to null]
**seller_fulfillment_order_item_id** | **String** | A fulfillment order item identifier that the seller created with a call to the createFulfillmentOrder operation. | [default to null]
**estimated_shipping_weight** | [***::models::Weight**](Weight.md) | The estimated shipping weight of the item quantity for a single item, as identified by sellerSku, in a shipment. | [optional] [default to null]
**shipping_weight_calculation_method** | **String** | The method used to calculate the estimated shipping weight. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


