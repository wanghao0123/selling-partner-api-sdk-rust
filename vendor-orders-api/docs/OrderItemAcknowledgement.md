# OrderItemAcknowledgement

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acknowledgement_code** | **String** | This indicates the acknowledgement code. | [default to null]
**acknowledged_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Details of quantity acknowledged with the above acknowledgement code. | [default to null]
**scheduled_ship_date** | **String** | Estimated ship date per line item. Must be in ISO-8601 date/time format. | [optional] [default to null]
**scheduled_delivery_date** | **String** | Estimated delivery date per line item. Must be in ISO-8601 date/time format. | [optional] [default to null]
**rejection_reason** | **String** | Indicates the reason for rejection. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


