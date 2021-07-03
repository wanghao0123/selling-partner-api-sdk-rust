# Item

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **i32** | Item Sequence Number for the item. This must be the same value as sent in order for a given item. | [default to null]
**buyer_product_identifier** | **String** | Buyer&#39;s Standard Identification Number (ASIN) of an item. Either buyerProductIdentifier or vendorProductIdentifier is required. | [optional] [default to null]
**vendor_product_identifier** | **String** | The vendor selected product identification of the item. Should be the same as was sent in the purchase order, like SKU Number. | [optional] [default to null]
**shipped_quantity** | [***::models::ItemQuantity**](ItemQuantity.md) | Total item quantity shipped in this shipment. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


