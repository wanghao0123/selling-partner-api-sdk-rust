# FulfillmentPreviewShipment

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**earliest_ship_date** | [***::models::Timestamp**](Timestamp.md) | The earliest date that the shipment is expected to be sent from the fulfillment center, in ISO 8601 date time format. | [optional] [default to null]
**latest_ship_date** | [***::models::Timestamp**](Timestamp.md) | The latest date that the shipment is expected to be sent from the fulfillment center, in ISO 8601 date time format. | [optional] [default to null]
**earliest_arrival_date** | [***::models::Timestamp**](Timestamp.md) | The earliest date that the shipment is expected to arrive at its destination. | [optional] [default to null]
**latest_arrival_date** | [***::models::Timestamp**](Timestamp.md) | The latest date that the shipment is expected to arrive at its destination, in ISO 8601 date time format. | [optional] [default to null]
**shipping_notes** | **Vec<String>** | Provides additional insight into the shipment timeline when exact delivery dates are not able to be precomputed. | [optional] [default to null]
**fulfillment_preview_items** | [***::models::FulfillmentPreviewItemList**](FulfillmentPreviewItemList.md) | Information about the items in the shipment. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


