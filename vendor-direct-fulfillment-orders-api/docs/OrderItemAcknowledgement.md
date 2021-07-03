# OrderItemAcknowledgement

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Line item sequence number for the item. | [default to null]
**buyer_product_identifier** | **String** | Buyer&#39;s standard identification number (ASIN) of an item. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. Should be the same as was provided in the purchase order. | [optional] [default to null]
**acknowledged_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Details of quantity acknowledged with the above acknowledgement code. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


