# ItemDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buyer_product_identifier** | **String** | The buyer selected product identification of the item. Either buyerProductIdentifier or vendorProductIdentifier should be submitted. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. Either buyerProductIdentifier or vendorProductIdentifier should be submitted. | [optional] [default to null]
**available_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Total item quantity available in the warehouse. | [default to null]
**is_obsolete** | **bool** | When true, the item is permanently unavailable. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


