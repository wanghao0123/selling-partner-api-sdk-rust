# FulfillmentShipment

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_shipment_id** | **String** | A shipment identifier assigned by Amazon. | [default to null]
**fulfillment_center_id** | **String** | An identifier for the fulfillment center that the shipment will be sent from. | [default to null]
**fulfillment_shipment_status** | **String** | The current status of the shipment. | [default to null]
**shipping_date** | [***::models::Timestamp**](Timestamp.md) | The meaning of the shippingDate value depends on the current status of the shipment. If the current value of FulfillmentShipmentStatus is:  * Pending - shippingDate represents the estimated time that the shipment will leave the Amazon fulfillment center.  * Shipped - shippingDate represents the date that the shipment left the Amazon fulfillment center. If a shipment includes more than one package, shippingDate applies to all of the packages in the shipment. If the value of FulfillmentShipmentStatus is CancelledByFulfiller or CancelledBySeller, shippingDate is not returned. The value must be in ISO 8601 date time format. | [optional] [default to null]
**estimated_arrival_date** | [***::models::Timestamp**](Timestamp.md) | The estimated arrival date and time of the shipment, in ISO 8601 date time format. Note that this value can change over time. If a shipment includes more than one package, estimatedArrivalDate applies to all of the packages in the shipment. If the shipment has been cancelled, estimatedArrivalDate is not returned. | [optional] [default to null]
**shipping_notes** | **Vec<String>** | Provides additional insight into shipment timeline. Primairly used to communicate that actual delivery dates aren&#39;t available. | [optional] [default to null]
**fulfillment_shipment_item** | [***::models::FulfillmentShipmentItemList**](FulfillmentShipmentItemList.md) |  | [default to null]
**fulfillment_shipment_package** | [***::models::FulfillmentShipmentPackageList**](FulfillmentShipmentPackageList.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


