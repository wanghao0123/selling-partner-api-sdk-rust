# ReturnItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_return_item_id** | **String** | An identifier assigned by the seller to the return item. | [default to null]
**seller_fulfillment_order_item_id** | **String** | The identifier assigned to the item by the seller when the fulfillment order was created. | [default to null]
**amazon_shipment_id** | **String** | The identifier for the shipment that is associated with the return item. | [default to null]
**seller_return_reason_code** | **String** | The return reason code assigned to the return item by the seller. | [default to null]
**return_comment** | **String** | An optional comment about the return item. | [optional] [default to null]
**amazon_return_reason_code** | **String** | The return reason code that the Amazon fulfillment center assigned to the return item. | [optional] [default to null]
**status** | [***::models::FulfillmentReturnItemStatus**](FulfillmentReturnItemStatus.md) | Indicates if the return item has been processed by an Amazon fulfillment center. | [default to null]
**status_changed_date** | [***::models::Timestamp**](Timestamp.md) | Indicates when the status last changed. | [default to null]
**return_authorization_id** | **String** | Identifies the return authorization used to return this item. See ReturnAuthorization. | [optional] [default to null]
**return_received_condition** | [***::models::ReturnItemDisposition**](ReturnItemDisposition.md) |  | [optional] [default to null]
**fulfillment_center_id** | **String** | The identifier for the Amazon fulfillment center that processed the return item. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


