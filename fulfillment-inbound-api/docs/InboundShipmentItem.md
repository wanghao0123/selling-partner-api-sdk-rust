# InboundShipmentItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | **String** | A shipment identifier originally returned by the createInboundShipmentPlan operation. | [optional] [default to null]
**seller_sku** | **String** | The seller SKU of the item. | [default to null]
**fulfillment_network_sku** | **String** | Amazon&#39;s fulfillment network SKU of the item. | [optional] [default to null]
**quantity_shipped** | [***::models::Quantity**](Quantity.md) | The item quantity that you are shipping. | [default to null]
**quantity_received** | [***::models::Quantity**](Quantity.md) | The item quantity that has been received at an Amazon fulfillment center. | [optional] [default to null]
**quantity_in_case** | [***::models::Quantity**](Quantity.md) | The item quantity in each case, for case-packed items. Note that QuantityInCase multiplied by the number of boxes in the inbound shipment equals QuantityShipped. Also note that all of the boxes of an inbound shipment must either be case packed or individually packed. For that reason, when you submit the createInboundShipment or the updateInboundShipment operation, the value of QuantityInCase must be provided for every item in the shipment or for none of the items in the shipment. | [optional] [default to null]
**release_date** | [***::models::DateStringType**](DateStringType.md) | The date that a pre-order item will be available for sale. | [optional] [default to null]
**prep_details_list** | [***::models::PrepDetailsList**](PrepDetailsList.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


