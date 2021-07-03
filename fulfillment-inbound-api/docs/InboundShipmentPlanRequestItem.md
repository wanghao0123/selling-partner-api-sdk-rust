# InboundShipmentPlanRequestItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU of the item. | [default to null]
**ASIN** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [default to null]
**condition** | [***::models::Condition**](Condition.md) |  | [default to null]
**quantity** | [***::models::Quantity**](Quantity.md) |  | [default to null]
**quantity_in_case** | [***::models::Quantity**](Quantity.md) | The item quantity in each case, for case-packed items. Note that QuantityInCase multiplied by the number of cases in the inbound shipment equals Quantity. Also note that all of the boxes of an inbound shipment must either be case packed or individually packed. For that reason, when you submit the createInboundShipmentPlan operation, the value of QuantityInCase must be provided for every item in the shipment or for none of the items in the shipment. | [optional] [default to null]
**prep_details_list** | [***::models::PrepDetailsList**](PrepDetailsList.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


