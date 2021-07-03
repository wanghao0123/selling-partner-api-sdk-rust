# FulfillmentOrderItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU of the item. | [default to null]
**seller_fulfillment_order_item_id** | **String** | A fulfillment order item identifier submitted with a call to the createFulfillmentOrder operation. | [default to null]
**quantity** | [***::models::Quantity**](Quantity.md) |  | [default to null]
**gift_message** | **String** | A message to the gift recipient, if applicable. | [optional] [default to null]
**displayable_comment** | **String** | Item-specific text that displays in recipient-facing materials such as the outbound shipment packing slip. | [optional] [default to null]
**fulfillment_network_sku** | **String** | Amazon&#39;s fulfillment network SKU of the item. | [optional] [default to null]
**order_item_disposition** | **String** | Indicates whether the item is sellable or unsellable. | [optional] [default to null]
**cancelled_quantity** | [***::models::Quantity**](Quantity.md) | The item quantity that was cancelled by the seller. | [default to null]
**unfulfillable_quantity** | [***::models::Quantity**](Quantity.md) | The item quantity that is unfulfillable. | [default to null]
**estimated_ship_date** | [***::models::Timestamp**](Timestamp.md) | The estimated date and time that the item quantity is scheduled to ship from the fulfillment center. Note that this value can change over time. If the shipment that contains the item quantity has been cancelled, estimatedShipDate is not returned. | [optional] [default to null]
**estimated_arrival_date** | [***::models::Timestamp**](Timestamp.md) | The estimated arrival date and time of the item quantity. Note that this value can change over time. If the shipment that contains the item quantity has been cancelled, estimatedArrivalDate is not returned. | [optional] [default to null]
**per_unit_price** | [***::models::Money**](Money.md) | The amount to be collected from the recipient for this item in a COD (Cash On Delivery) order. | [optional] [default to null]
**per_unit_tax** | [***::models::Money**](Money.md) | The tax on the amount to be collected from the recipient for this item in a COD (Cash On Delivery) order. | [optional] [default to null]
**per_unit_declared_value** | [***::models::Money**](Money.md) | The monetary value assigned by the seller to this item. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


